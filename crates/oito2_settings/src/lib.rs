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

//! Reads the desktop settings the OITO2 user interfaces follow: window button
//! layout, accent color, high contrast and color scheme, on Linux (GSettings,
//! configuration files and the settings portal, through GIO), Windows
//! (Windows Runtime) and macOS (AppKit and the user defaults).
//!
//! [`read_raw_settings`] collects what the platform stores into a
//! [`RawSettings`], the type the OITO2 Tauri plugins send to their webview;
//! [`read_system_settings`] composes it into the [`SystemSettings`] of the
//! running platform with the rules of `oito2_foundation`, re-exported here as
//! [`foundation`].
//!
//! Every reader is safe to call from any thread and never fails: a store that
//! can't be read leaves its values empty, so the composition falls back to
//! the desktop's defaults.
//!
//! With the `watch` feature, [`watch`] reports every change of the settings
//! from a background thread, on Linux, Windows and macOS.

#![deny(missing_docs)]

mod color;
#[cfg(target_os = "linux")]
mod config;
mod error;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(any(target_os = "linux", test))]
mod parsers;
#[cfg(feature = "watch")]
mod watcher;
#[cfg(windows)]
mod windows;

pub use oito2_foundation as foundation;
pub use oito2_foundation::{
    ColorScheme, ConfigValues, Platform, RawSettings, SettingValue, SystemSettings,
};

pub use error::{Error, Result};
#[cfg(feature = "watch")]
pub use watcher::SettingsWatcher;

/// Reads the settings of the running platform. Outside Linux, Windows and
/// macOS nothing is read and every value is empty.
pub fn read_raw_settings() -> RawSettings {
    #[cfg(target_os = "linux")]
    return linux::read();
    #[cfg(windows)]
    return windows::read();
    #[cfg(target_os = "macos")]
    return macos::read();
    #[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
    return RawSettings::default();
}

/// Reads the settings of the running platform and composes them with
/// [`SystemSettings::compose`] for [`Platform::current`].
pub fn read_system_settings() -> SystemSettings {
    SystemSettings::compose(&read_raw_settings(), Platform::current())
}

/// Watches the settings of the running platform and calls `on_change` with
/// the new [`RawSettings`] after each change, from a thread of the returned
/// [`SettingsWatcher`], until the watcher is dropped. Read the settings once
/// with [`read_raw_settings`] for their value before the first change.
///
/// On Linux it watches GSettings, the configuration files and the settings
/// portal, on a GLib main context of its own: it needs no main loop from the
/// application and doesn't disturb one the application runs.
///
/// On Windows it registers for the `ColorValuesChanged` event of `UISettings`
/// (accent and light or dark theme) and the `HighContrastChanged` event of
/// `AccessibilitySettings`, which the Windows Runtime raises on threads of its
/// own.
///
/// On macOS it observes the `NSSystemColorsDidChangeNotification`,
/// `AppleInterfaceThemeChangedNotification` and
/// `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification`
/// notifications, which are delivered through the application's main run
/// loop: the application must run one, as every AppKit application does.
///
/// # Errors
///
/// [`Error::Unsupported`] outside Linux, Windows and macOS;
/// [`Error::Internal`] when a watcher thread can't start or, on Windows, when
/// the `UISettings` events are unavailable.
#[cfg(feature = "watch")]
pub fn watch(on_change: impl Fn(&RawSettings) + Send + 'static) -> Result<SettingsWatcher> {
    #[cfg(target_os = "linux")]
    {
        linux::watch::watch(on_change)
    }
    #[cfg(windows)]
    {
        windows::watch::watch(on_change)
    }
    #[cfg(target_os = "macos")]
    {
        macos::watch::watch(on_change)
    }
    #[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
    {
        let _ = on_change;
        Err(Error::Unsupported(
            "watching the settings on this platform".into(),
        ))
    }
}

/// Compiles the examples of `README.md` as documentation tests.
#[cfg(all(doctest, feature = "watch"))]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Compiles the examples of the quickstart page as documentation tests.
#[cfg(all(doctest, feature = "watch"))]
#[doc = include_str!("../../../docs/en/getting-started/quickstart.md")]
struct QuickstartDoctests;

/// Compiles the examples of the integration guide as documentation tests.
#[cfg(all(doctest, feature = "watch"))]
#[doc = include_str!("../../../docs/en/guides/workflows/examples.md")]
struct ExamplesDoctests;
