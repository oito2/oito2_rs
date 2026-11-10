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

//! The watcher handle shared by the platform watchers.
//!
//! A platform watcher runs the system's change notifications on a thread of
//! its own and sends an event for each one. An emitter thread waits for a
//! pause in those events, reads the settings again and calls the callback
//! only when they differ from the last ones it saw.

// Only the Linux watcher starts a `SettingsWatcher` on other systems' builds
// of this version; the emitter is still compiled and tested there.
#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::{Error, RawSettings, Result};

/// How long the emitter waits after a change before reading again, so a
/// burst of changes produces one callback.
pub(crate) const DEBOUNCE: Duration = Duration::from_millis(150);

/// What a platform watcher tells the emitter.
pub(crate) enum Event {
    /// A store reported a change.
    Changed,
    /// The watcher is stopping.
    Stop,
}

/// Reads the current settings.
pub(crate) type Read = Box<dyn Fn() -> RawSettings + Send>;
/// Receives the settings after each change.
pub(crate) type OnChange = Box<dyn Fn(&RawSettings) + Send>;
/// Stops the thread of a platform watcher.
pub(crate) type StopSource = Box<dyn FnOnce() + Send + Sync>;

/// Watches the system settings in the background and calls its callback with
/// the new [`RawSettings`] after each change, until it is dropped.
///
/// Created by [`watch`](crate::watch). A burst of changes produces one
/// callback, 150 ms after the last change, and only when the settings read
/// then differ from the ones of the previous callback (or, for the first
/// callback, from the ones read when the watcher started). The callback runs
/// on a thread of the watcher.
///
/// Dropping the watcher stops its threads and waits for them; a callback in
/// progress finishes first. Dropping it from inside its own callback stops it
/// without waiting.
#[must_use = "the watcher stops when dropped"]
pub struct SettingsWatcher {
    stop_source: Option<StopSource>,
    source: Option<JoinHandle<()>>,
    events: Sender<Event>,
    emitter: Option<JoinHandle<()>>,
}

impl SettingsWatcher {
    /// Reads the settings once as the reference, starts the emitter thread,
    /// then starts the platform watcher with `source`, which receives the
    /// sender of the change events and returns the function that stops it
    /// and its thread.
    pub(crate) fn start(
        read: Read,
        on_change: OnChange,
        debounce: Duration,
        source: impl FnOnce(Sender<Event>) -> Result<(StopSource, JoinHandle<()>)>,
    ) -> Result<Self> {
        let last = read();
        let (events, received) = mpsc::channel();
        let emitter = thread::Builder::new()
            .name("oito2-settings-emit".into())
            .spawn(move || emit(&received, last, &read, &on_change, debounce))
            .map_err(|error| Error::Internal(format!("could not start the emitter: {error}")))?;
        match source(events.clone()) {
            Ok((stop, handle)) => Ok(Self {
                stop_source: Some(stop),
                source: Some(handle),
                events,
                emitter: Some(emitter),
            }),
            Err(error) => {
                let _ = events.send(Event::Stop);
                let _ = emitter.join();
                Err(error)
            }
        }
    }
}

impl Drop for SettingsWatcher {
    fn drop(&mut self) {
        if let Some(stop) = self.stop_source.take() {
            stop();
        }
        join(self.source.take());
        let _ = self.events.send(Event::Stop);
        join(self.emitter.take());
    }
}

impl std::fmt::Debug for SettingsWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsWatcher").finish_non_exhaustive()
    }
}

/// Waits for `handle`'s thread to finish, unless it is the current thread.
fn join(handle: Option<JoinHandle<()>>) {
    if let Some(handle) = handle
        && handle.thread().id() != thread::current().id()
    {
        let _ = handle.join();
    }
}

/// The emitter loop: after each change, waits until no event arrives for
/// `debounce`, reads the settings and calls `on_change` when they differ from
/// `last`. Returns on [`Event::Stop`] or when every sender is gone.
fn emit(
    events: &Receiver<Event>,
    mut last: RawSettings,
    read: &Read,
    on_change: &OnChange,
    debounce: Duration,
) {
    while let Ok(Event::Changed) = events.recv() {
        loop {
            match events.recv_timeout(debounce) {
                Ok(Event::Changed) => {}
                Err(RecvTimeoutError::Timeout) => break,
                Ok(Event::Stop) | Err(RecvTimeoutError::Disconnected) => return,
            }
        }
        let raw = read();
        if raw != last {
            on_change(&raw);
            last = raw;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    /// A watcher whose platform part does nothing; returns it, the sender
    /// its platform part received, the desktop the reader reports and the
    /// receiver of the callbacks.
    fn fake() -> (
        SettingsWatcher,
        Sender<Event>,
        Arc<Mutex<String>>,
        Receiver<RawSettings>,
    ) {
        let desktop = Arc::new(Mutex::new("GNOME".to_owned()));
        let current = desktop.clone();
        let read: Read = Box::new(move || RawSettings {
            xdg_current_desktop: Some(current.lock().unwrap().clone()),
            ..Default::default()
        });
        let (sent, received) = mpsc::channel();
        let on_change: OnChange = Box::new(move |raw| {
            let _ = sent.send(raw.clone());
        });
        let (sender_out, sender_in) = mpsc::channel();
        let watcher =
            SettingsWatcher::start(read, on_change, Duration::from_millis(50), |events| {
                sender_out.send(events).unwrap();
                let stop: StopSource = Box::new(|| {});
                Ok((stop, thread::spawn(|| {})))
            })
            .unwrap();
        (watcher, sender_in.recv().unwrap(), desktop, received)
    }

    #[test]
    fn is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SettingsWatcher>();
    }

    #[test]
    fn reports_a_burst_once_and_only_real_changes() {
        let (watcher, events, desktop, received) = fake();
        events.send(Event::Changed).unwrap();
        assert!(received.recv_timeout(Duration::from_millis(300)).is_err());

        *desktop.lock().unwrap() = "KDE".into();
        for _ in 0..3 {
            events.send(Event::Changed).unwrap();
        }
        let raw = received.recv_timeout(Duration::from_secs(2)).unwrap();
        assert_eq!(raw.xdg_current_desktop.as_deref(), Some("KDE"));
        assert!(received.recv_timeout(Duration::from_millis(300)).is_err());

        events.send(Event::Changed).unwrap();
        assert!(received.recv_timeout(Duration::from_millis(300)).is_err());
        drop(watcher);
    }

    #[test]
    fn stops_when_dropped() {
        let (watcher, _events, _desktop, received) = fake();
        let (done, finished) = mpsc::channel();
        thread::spawn(move || {
            drop(watcher);
            let _ = done.send(());
        });
        finished.recv_timeout(Duration::from_secs(2)).unwrap();
        assert!(received.recv().is_err());
    }

    #[test]
    fn a_failing_platform_part_stops_the_emitter() {
        let read: Read = Box::new(RawSettings::default);
        let on_change: OnChange = Box::new(|_| {});
        let result = SettingsWatcher::start(read, on_change, DEBOUNCE, |_| {
            Err(Error::Unsupported("test".into()))
        });
        assert_eq!(result.unwrap_err(), Error::Unsupported("test".into()));
    }
}
