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

//! Watches the macOS settings through notifications: the local
//! `NSSystemColorsDidChangeNotification` (accent color), the distributed
//! `AppleInterfaceThemeChangedNotification` (light or dark appearance) and
//! the workspace's `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification`
//! ("Increase contrast").
//!
//! The observers run their blocks on the thread that posts each
//! notification, which AppKit and the distributed notification center do from
//! the application's main run loop: the application must run one, as every
//! AppKit application (winit, Tauri) does. A thread of the watcher keeps the
//! observers registered until the watcher stops.

use std::ptr::NonNull;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2_app_kit::{
    NSSystemColorsDidChangeNotification, NSWorkspace,
    NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification,
};
use objc2_foundation::{
    NSDistributedNotificationCenter, NSNotification, NSNotificationCenter, NSObjectProtocol,
    NSString,
};

use crate::watcher::{DEBOUNCE, Event, SettingsWatcher, StopSource};
use crate::{Error, RawSettings, Result};

/// The distributed notification macOS posts when the appearance changes
/// between light and dark.
const THEME_CHANGED: &str = "AppleInterfaceThemeChangedNotification";

/// Watches the macOS settings and calls `on_change` with the new settings
/// after each change.
pub fn watch(on_change: impl Fn(&RawSettings) + Send + 'static) -> Result<SettingsWatcher> {
    SettingsWatcher::start(
        Box::new(super::read),
        Box::new(on_change),
        DEBOUNCE,
        spawn_sources,
    )
}

/// Starts the thread that registers the observers and keeps them until it is
/// told to stop, and waits until they are registered.
fn spawn_sources(events: Sender<Event>) -> Result<(StopSource, JoinHandle<()>)> {
    let (stop, stopped) = mpsc::channel::<()>();
    let (ready, started) = mpsc::channel::<()>();
    let handle = thread::Builder::new()
        .name("oito2-settings-watch".into())
        .spawn(move || {
            let observers = Observers::register(&events);
            let _ = ready.send(());
            let _ = stopped.recv();
            drop(observers);
        })
        .map_err(|error| Error::Internal(format!("could not start the watcher: {error}")))?;
    if started.recv().is_err() {
        let _ = handle.join();
        return Err(Error::Internal("the watcher stopped while starting".into()));
    }
    let stop: StopSource = Box::new(move || {
        let _ = stop.send(());
    });
    Ok((stop, handle))
}

/// An observer token and the center it is registered with.
type Registration = (
    Retained<NSNotificationCenter>,
    Retained<ProtocolObject<dyn NSObjectProtocol>>,
);

/// The registered observers; they are removed on drop.
struct Observers(Vec<Registration>);

impl Observers {
    /// Registers, for each watched notification, a block that sends
    /// [`Event::Changed`] to `events`.
    fn register(events: &Sender<Event>) -> Self {
        let block = {
            let events = events.clone();
            RcBlock::new(move |_: NonNull<NSNotification>| {
                let _ = events.send(Event::Changed);
            })
        };
        let local = NSNotificationCenter::defaultCenter();
        let distributed: Retained<NSNotificationCenter> =
            Retained::into_super(NSDistributedNotificationCenter::defaultCenter());
        let workspace = NSWorkspace::sharedWorkspace().notificationCenter();
        let theme = NSString::from_str(THEME_CHANGED);
        // SAFETY: the two names are immutable `NSString` constants AppKit
        // exports for the whole life of the process.
        let (colors, accessibility) = unsafe {
            (
                NSSystemColorsDidChangeNotification,
                NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification,
            )
        };
        let mut registrations = Vec::with_capacity(3);
        for (center, name) in [
            (local, colors),
            (distributed, &*theme),
            (workspace, accessibility),
        ] {
            // SAFETY: no sender object or queue is given, so the block runs
            // on the posting thread with any sender; the block only owns a
            // `Sender`, which is `Send`, so running it on any thread is sound.
            let token = unsafe {
                center.addObserverForName_object_queue_usingBlock(Some(name), None, None, &block)
            };
            registrations.push((center, token));
        }
        Self(registrations)
    }
}

impl Drop for Observers {
    fn drop(&mut self) {
        for (center, token) in self.0.drain(..) {
            let observer: &AnyObject = token.as_ref();
            // SAFETY: `token` is the observer object the same center returned
            // when the block was registered.
            unsafe { center.removeObserver(observer) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_and_stops() {
        let watcher = watch(|_| {}).unwrap();
        let (done, finished) = mpsc::channel();
        thread::spawn(move || {
            drop(watcher);
            let _ = done.send(());
        });
        finished
            .recv_timeout(std::time::Duration::from_secs(5))
            .unwrap();
    }
}
