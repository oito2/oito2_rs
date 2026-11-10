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

//! Watches the Windows settings: the `ColorValuesChanged` event of
//! `UISettings` (accent and light or dark theme) and the
//! `HighContrastChanged` event of `AccessibilitySettings`. The Windows Runtime
//! raises them on threads of its own; a thread of the watcher keeps the
//! registrations alive until the watcher stops.

use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

use windows::Foundation::TypedEventHandler;
use windows::UI::ViewManagement::{AccessibilitySettings, UISettings};

use crate::watcher::{DEBOUNCE, Event, SettingsWatcher, StopSource};
use crate::{Error, RawSettings, Result};

/// Watches the Windows settings and calls `on_change` with the new settings
/// after each change.
pub fn watch(on_change: impl Fn(&RawSettings) + Send + 'static) -> Result<SettingsWatcher> {
    SettingsWatcher::start(
        Box::new(super::read),
        Box::new(on_change),
        DEBOUNCE,
        spawn_sources,
    )
}

/// Starts the thread that registers the event handlers and keeps them until
/// it is told to stop, and waits until they are registered.
fn spawn_sources(events: Sender<Event>) -> Result<(StopSource, JoinHandle<()>)> {
    let (stop, stopped) = mpsc::channel::<()>();
    let (ready, started) = mpsc::channel::<Result<()>>();
    let handle = thread::Builder::new()
        .name("oito2-settings-watch".into())
        .spawn(move || match Registrations::register(&events) {
            Ok(registrations) => {
                let _ = ready.send(Ok(()));
                let _ = stopped.recv();
                drop(registrations);
            }
            Err(error) => {
                let _ = ready.send(Err(error));
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
    let stop: StopSource = Box::new(move || {
        let _ = stop.send(());
    });
    Ok((stop, handle))
}

/// The registered event handlers; they are removed on drop.
struct Registrations {
    ui: UISettings,
    ui_token: i64,
    accessibility: Option<(AccessibilitySettings, i64)>,
}

impl Registrations {
    /// Registers a handler that sends [`Event::Changed`] to `events` on each
    /// event. Fails when `UISettings` is unavailable; a missing
    /// `AccessibilitySettings` only leaves high contrast unwatched.
    fn register(events: &Sender<Event>) -> Result<Self> {
        let handler = || {
            let events = events.clone();
            move || {
                let _ = events.send(Event::Changed);
            }
        };
        let internal = |error: windows::core::Error| {
            Error::Internal(format!("could not watch the Windows settings: {error}"))
        };
        let ui = UISettings::new().map_err(internal)?;
        let notify = handler();
        let ui_token = ui
            .ColorValuesChanged(&TypedEventHandler::new(move |_, _| {
                notify();
                Ok(())
            }))
            .map_err(internal)?;
        let accessibility = AccessibilitySettings::new().ok().and_then(|settings| {
            let notify = handler();
            let token = settings
                .HighContrastChanged(&TypedEventHandler::new(move |_, _| {
                    notify();
                    Ok(())
                }))
                .ok()?;
            Some((settings, token))
        });
        Ok(Self {
            ui,
            ui_token,
            accessibility,
        })
    }
}

impl Drop for Registrations {
    fn drop(&mut self) {
        let _ = self.ui.RemoveColorValuesChanged(self.ui_token);
        if let Some((settings, token)) = self.accessibility.take() {
            let _ = settings.RemoveHighContrastChanged(token);
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
