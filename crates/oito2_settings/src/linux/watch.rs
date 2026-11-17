// Copyright (C) 2026  OITO2
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Watches the Linux settings stores on a thread of its own: GSettings
//! signals, file monitors on the configuration directories and the portal's
//! `SettingChanged` signal, all attached to a GLib main context that only
//! this thread runs, so the watcher works with or without the application's
//! own GLib main loop.

use std::cell::RefCell;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use gio::prelude::*;
use oito2_foundation::raw::GSETTINGS_KEYS;

use crate::watcher::{DEBOUNCE, Event, OnChange, Read, SettingsWatcher, StopSource};
use crate::{Error, RawSettings, Result};

/// Watches every Linux settings store and calls `on_change` with the new
/// settings after each change.
pub fn watch(on_change: impl Fn(&RawSettings) + Send + 'static) -> Result<SettingsWatcher> {
    start(
        super::config_home(),
        Box::new(super::read),
        Box::new(on_change),
        DEBOUNCE,
    )
}

/// Starts a watcher over the configuration files under `home`, GSettings and
/// the portal, reading the settings with `read`.
fn start(
    home: Option<PathBuf>,
    read: Read,
    on_change: OnChange,
    debounce: Duration,
) -> Result<SettingsWatcher> {
    SettingsWatcher::start(read, on_change, debounce, move |events| {
        spawn_sources(home, events)
    })
}

/// Starts the thread that attaches every store to its own main context and
/// runs it, and waits until the stores are attached. Returns the function
/// that quits that main context's loop, and the thread.
fn spawn_sources(
    home: Option<PathBuf>,
    events: Sender<Event>,
) -> Result<(StopSource, JoinHandle<()>)> {
    let context = glib::MainContext::new();
    let main_loop = glib::MainLoop::new(Some(&context), false);
    let (ready, started) = mpsc::channel::<Result<()>>();
    let thread_context = context.clone();
    let thread_loop = main_loop.clone();
    let handle = thread::Builder::new()
        .name("oito2-settings-watch".into())
        .spawn(move || {
            let attached = thread_context.with_thread_default(|| {
                let sources = Sources::attach(home.as_deref(), &events);
                let _ = ready.send(Ok(()));
                thread_loop.run();
                drop(sources);
            });
            if let Err(error) = attached {
                let _ = ready.send(Err(Error::Internal(format!(
                    "could not own the watcher's main context: {error}"
                ))));
            }
        })
        .map_err(|error| Error::Internal(format!("could not start the watcher: {error}")))?;
    let failure = match started.recv() {
        Ok(Ok(())) => None,
        Ok(Err(error)) => Some(error),
        Err(_) => Some(Error::Internal("the watcher stopped while starting".into())),
    };
    if let Some(error) = failure {
        let _ = handle.join();
        return Err(error);
    }
    // Quitting through the context runs once its loop iterates, so a stop
    // requested before the loop starts running is not lost.
    let stop: StopSource = Box::new(move || context.invoke(move || main_loop.quit()));
    Ok((stop, handle))
}

/// The objects that report changes; they report while they are alive.
struct Sources {
    _settings: Vec<gio::Settings>,
    _files: Option<Rc<FileWatch>>,
    portal: Option<(gio::DBusConnection, gio::SignalSubscriptionId)>,
}

impl Sources {
    /// Attaches every store to the thread-default main context; each change
    /// sends [`Event::Changed`] to `events`.
    fn attach(home: Option<&Path>, events: &Sender<Event>) -> Self {
        let notify: Rc<dyn Fn()> = {
            let events = events.clone();
            Rc::new(move || {
                let _ = events.send(Event::Changed);
            })
        };
        Self {
            _settings: watch_gsettings(&notify),
            _files: home.map(|home| FileWatch::start(home, &notify)),
            portal: watch_portal(&notify),
        }
    }
}

impl Drop for Sources {
    fn drop(&mut self) {
        if let Some((connection, id)) = self.portal.take() {
            connection.signal_unsubscribe(id);
        }
    }
}

/// Connects to the `changed` signal of every installed schema that holds a
/// watched key, and reads those keys once so GSettings reports their changes.
fn watch_gsettings(notify: &Rc<dyn Fn()>) -> Vec<gio::Settings> {
    let Some(source) = gio::SettingsSchemaSource::default() else {
        return Vec::new();
    };
    let mut watched = Vec::new();
    for schema_id in super::gsettings::watched_schemas() {
        let Some(schema) = source.lookup(schema_id, true) else {
            continue;
        };
        let settings = gio::Settings::new(schema_id);
        let callback = notify.clone();
        settings.connect_changed(None, move |_, _| callback());
        for (_, key) in GSETTINGS_KEYS
            .iter()
            .filter(|(id, key)| *id == schema_id && schema.has_key(key))
        {
            let _ = settings.value(key);
        }
        watched.push(settings);
    }
    watched
}

/// The file monitors of the configuration directories under `home`.
///
/// A configuration directory that doesn't exist yet (a desktop writes it the
/// first time a setting leaves its default) is waited for through a monitor
/// on its nearest existing ancestor inside `home`. When the next directory on
/// the way appears, or a monitored directory is deleted, every monitor is
/// rebuilt and `notify` runs, so files written together with their directory
/// are read too.
struct FileWatch {
    home: PathBuf,
    notify: Rc<dyn Fn()>,
    monitors: RefCell<Vec<gio::FileMonitor>>,
}

/// How many times in a row `FileWatch::arm` rebuilds the monitors when the
/// directories change while it creates them.
const MAX_ARM_ATTEMPTS: usize = 8;

impl FileWatch {
    /// Creates the monitors for the configuration directories under `home`.
    fn start(home: &Path, notify: &Rc<dyn Fn()>) -> Rc<Self> {
        let watch = Rc::new(Self {
            home: home.to_path_buf(),
            notify: notify.clone(),
            monitors: RefCell::new(Vec::new()),
        });
        watch.arm();
        watch
    }

    /// Replaces the monitors with new ones that match the directories that
    /// exist now. When a directory appears or disappears while the monitors
    /// are created, they are rebuilt and `notify` runs, since that change
    /// produced no event.
    fn arm(self: &Rc<Self>) {
        for _ in 0..MAX_ARM_ATTEMPTS {
            if self.arm_once() {
                return;
            }
            (self.notify)();
        }
    }

    /// Replaces the monitors once; returns `false` when the directories
    /// changed between the existence checks and the creation of the monitors.
    fn arm_once(self: &Rc<Self>) -> bool {
        let weak = Rc::downgrade(self);
        let rearm: Rc<dyn Fn()> = Rc::new(move || {
            if let Some(watch) = weak.upgrade() {
                watch.arm();
                (watch.notify)();
            }
        });
        let mut monitors = Vec::new();
        let mut watched = Vec::new();
        // Existing ancestors and the names of their children on the way to
        // missing directories.
        let mut waiting: Vec<(PathBuf, Vec<OsString>)> = Vec::new();
        for (directory, names) in crate::config::watched_files(&self.home) {
            if directory.is_dir() {
                monitors.extend(watch_names(
                    &directory,
                    names,
                    self.notify.clone(),
                    rearm.clone(),
                ));
                watched.push(directory);
            } else if let Some((ancestor, child)) = nearest_existing(&self.home, &directory) {
                match waiting.iter_mut().find(|(dir, _)| *dir == ancestor) {
                    Some((_, children)) if children.contains(&child) => {}
                    Some((_, children)) => children.push(child),
                    None => waiting.push((ancestor, vec![child])),
                }
            }
        }
        for (ancestor, children) in &waiting {
            monitors.extend(watch_children(ancestor, children.clone(), rearm.clone()));
        }
        for monitor in self.monitors.replace(monitors) {
            monitor.cancel();
        }
        // A change made before its monitor existed produced no event.
        watched.iter().all(|directory| directory.is_dir())
            && waiting.iter().all(|(ancestor, children)| {
                ancestor.is_dir() && children.iter().all(|child| !ancestor.join(child).exists())
            })
    }
}

impl Drop for FileWatch {
    fn drop(&mut self) {
        for monitor in self.monitors.get_mut().drain(..) {
            monitor.cancel();
        }
    }
}

/// Returns the nearest ancestor of the missing `directory` that exists inside
/// `home` (or is `home`), with the name of its child on the way to
/// `directory`; `None` when `home` doesn't exist or `directory` is outside it.
fn nearest_existing(home: &Path, directory: &Path) -> Option<(PathBuf, OsString)> {
    let mut child = directory;
    while let Some(parent) = child.parent() {
        if !parent.starts_with(home) {
            return None;
        }
        if parent.is_dir() {
            return Some((parent.to_path_buf(), child.file_name()?.to_os_string()));
        }
        child = parent;
    }
    None
}

/// Monitors `directory`, calling `notify` when a file named in `names` is
/// written, created, deleted or replaced, and `lost` when `directory` itself
/// is deleted or unmounted.
fn watch_names(
    directory: &Path,
    names: Vec<String>,
    notify: Rc<dyn Fn()>,
    lost: Rc<dyn Fn()>,
) -> Option<gio::FileMonitor> {
    let monitor = monitor_directory(directory)?;
    let directory = directory.to_path_buf();
    // Matches writes in place and atomic replacements renamed onto the file.
    monitor.connect_changed(move |_, file, other, event| {
        if is_lost(&directory, file, event) {
            lost();
            return;
        }
        let touches = |file: Option<&gio::File>| {
            file.and_then(|file| file.basename())
                .is_some_and(|name| names.iter().any(|n| name.as_os_str() == n.as_str()))
        };
        if touches(Some(file)) || touches(other) {
            notify();
        }
    });
    Some(monitor)
}

/// Monitors `directory` and calls `changed` when an entry named in
/// `children` is created, or moved or renamed into it, and when `directory`
/// itself is deleted or unmounted.
fn watch_children(
    directory: &Path,
    children: Vec<OsString>,
    changed: Rc<dyn Fn()>,
) -> Option<gio::FileMonitor> {
    let monitor = monitor_directory(directory)?;
    let directory = directory.to_path_buf();
    monitor.connect_changed(move |_, file, other, event| {
        let named = |file: Option<&gio::File>| {
            file.and_then(|file| file.basename())
                .is_some_and(|name| children.iter().any(|child| name.as_os_str() == child))
        };
        let appears = match event {
            gio::FileMonitorEvent::Created | gio::FileMonitorEvent::MovedIn => named(Some(file)),
            gio::FileMonitorEvent::Renamed => named(other),
            _ => false,
        };
        if appears || is_lost(&directory, file, event) {
            changed();
        }
    });
    Some(monitor)
}

/// Tells whether an event of the monitor on `directory` reports that the
/// directory itself was deleted or unmounted.
fn is_lost(directory: &Path, file: &gio::File, event: gio::FileMonitorEvent) -> bool {
    matches!(
        event,
        gio::FileMonitorEvent::Deleted | gio::FileMonitorEvent::Unmounted
    ) && file.path().as_deref() == Some(directory)
}

/// Creates a monitor on `directory` that reports moves as moves.
fn monitor_directory(directory: &Path) -> Option<gio::FileMonitor> {
    gio::File::for_path(directory)
        .monitor_directory(
            gio::FileMonitorFlags::WATCH_MOVES,
            None::<&gio::Cancellable>,
        )
        .ok()
}

/// Subscribes to the portal's `SettingChanged` signal for the appearance
/// keys, when there is a session bus.
fn watch_portal(notify: &Rc<dyn Fn()>) -> Option<(gio::DBusConnection, gio::SignalSubscriptionId)> {
    let connection = gio::bus_get_sync(gio::BusType::Session, None::<&gio::Cancellable>).ok()?;
    let callback = notify.clone();
    let id = connection.signal_subscribe(
        Some(super::portal::BUS_NAME),
        Some(super::portal::INTERFACE),
        Some("SettingChanged"),
        Some(super::portal::OBJECT_PATH),
        Some(super::portal::APPEARANCE),
        gio::DBusSignalFlags::NONE,
        move |_, _, _, _, _, _| callback(),
    );
    Some((connection, id))
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::Receiver;

    use super::*;

    /// Creates an empty temporary configuration directory for one test.
    fn temp_home(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("oito2-watch-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Starts a watcher whose reader only reads the files under `home`;
    /// returns it and the receiver of its callbacks.
    fn watch_home(home: &Path) -> (SettingsWatcher, Receiver<RawSettings>) {
        let read_home = home.to_path_buf();
        let read: Read = Box::new(move || RawSettings {
            config: crate::config::read_config(&read_home),
            ..Default::default()
        });
        let (sent, received) = mpsc::channel();
        let on_change: OnChange = Box::new(move |raw| {
            let _ = sent.send(raw.clone());
        });
        let watcher = start(Some(home.to_path_buf()), read, on_change, DEBOUNCE).unwrap();
        (watcher, received)
    }

    const QUIET: Duration = Duration::from_millis(600);
    const DEADLINE: Duration = Duration::from_secs(2);

    #[test]
    fn reports_a_burst_of_file_changes_once() {
        let home = temp_home("burst");
        let (watcher, received) = watch_home(&home);

        // A file that doesn't matter is ignored.
        std::fs::write(home.join("other.conf"), "x").unwrap();
        assert!(received.recv_timeout(QUIET).is_err());

        // Several writes in a row report one change.
        std::fs::write(
            home.join("kwinrc"),
            "[org.kde.kdecoration2]\nButtonsOnLeft=X\n",
        )
        .unwrap();
        std::fs::write(home.join("kdeglobals"), "[General]\nAccentColor=1,2,3\n").unwrap();
        let raw = received.recv_timeout(DEADLINE).unwrap();
        assert_eq!(raw.config.kwin_buttons_on_left.as_deref(), Some("X"));
        assert_eq!(raw.config.kde_accent_color.as_deref(), Some("#010203"));
        assert!(received.recv_timeout(QUIET).is_err());

        // Writing the same content again reports nothing.
        std::fs::write(
            home.join("kwinrc"),
            "[org.kde.kdecoration2]\nButtonsOnLeft=X\n",
        )
        .unwrap();
        assert!(received.recv_timeout(QUIET).is_err());

        // An atomic replacement counts too.
        std::fs::write(
            home.join("kwinrc.tmp"),
            "[org.kde.kdecoration2]\nButtonsOnLeft=IA\n",
        )
        .unwrap();
        std::fs::rename(home.join("kwinrc.tmp"), home.join("kwinrc")).unwrap();
        let raw = received.recv_timeout(DEADLINE).unwrap();
        assert_eq!(raw.config.kwin_buttons_on_left.as_deref(), Some("IA"));

        drop(watcher);
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn reports_files_in_directories_created_after_starting() {
        let home = temp_home("created");
        let (watcher, received) = watch_home(&home);
        let cosmic = home.join("cosmic/com.system76.CosmicTk/v1");

        // A directory that doesn't matter is ignored.
        std::fs::create_dir_all(home.join("other/v1")).unwrap();
        assert!(received.recv_timeout(QUIET).is_err());

        // The file written together with its directories is read.
        std::fs::create_dir_all(&cosmic).unwrap();
        std::fs::write(cosmic.join("show_minimize"), "false").unwrap();
        let raw = received.recv_timeout(DEADLINE).unwrap();
        assert_eq!(raw.config.cosmic_show_minimize, Some(false));

        // Later writes in the new directory are watched too.
        std::fs::write(cosmic.join("show_minimize"), "true").unwrap();
        let raw = received.recv_timeout(DEADLINE).unwrap();
        assert_eq!(raw.config.cosmic_show_minimize, Some(true));

        drop(watcher);
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn reports_files_in_directories_deleted_and_created_again() {
        let home = temp_home("recreated");
        let cosmic = home.join("cosmic/com.system76.CosmicTk/v1");
        std::fs::create_dir_all(&cosmic).unwrap();
        std::fs::write(cosmic.join("show_minimize"), "false").unwrap();
        let (watcher, received) = watch_home(&home);

        // Deleting the directory tree reports the value that is gone.
        std::fs::remove_dir_all(home.join("cosmic")).unwrap();
        let raw = received.recv_timeout(DEADLINE).unwrap();
        assert_eq!(raw.config.cosmic_show_minimize, None);

        // The directory created again is watched again.
        std::fs::create_dir_all(&cosmic).unwrap();
        std::fs::write(cosmic.join("show_minimize"), "true").unwrap();
        let raw = received.recv_timeout(DEADLINE).unwrap();
        assert_eq!(raw.config.cosmic_show_minimize, Some(true));

        std::fs::write(cosmic.join("show_minimize"), "false").unwrap();
        let raw = received.recv_timeout(DEADLINE).unwrap();
        assert_eq!(raw.config.cosmic_show_minimize, Some(false));

        drop(watcher);
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn nearest_existing_stops_inside_home() {
        let home = temp_home("nearest");
        let missing = home.join("a/b/c");
        assert_eq!(
            nearest_existing(&home, &missing),
            Some((home.clone(), OsString::from("a")))
        );
        std::fs::create_dir_all(home.join("a")).unwrap();
        assert_eq!(
            nearest_existing(&home, &missing),
            Some((home.join("a"), OsString::from("b")))
        );
        assert_eq!(nearest_existing(&home.join("absent"), &missing), None);
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn drop_stops_the_threads_right_after_starting() {
        let home = temp_home("drop");
        let (watcher, received) = watch_home(&home);
        let (done, finished) = mpsc::channel();
        thread::spawn(move || {
            drop(watcher);
            let _ = done.send(());
        });
        finished.recv_timeout(DEADLINE).unwrap();
        // Every sender of the callbacks is gone with the emitter thread.
        assert!(received.recv().is_err());
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn the_public_watcher_starts_and_stops() {
        let watcher = watch(|_| {}).unwrap();
        drop(watcher);
    }
}
