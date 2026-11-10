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

#![deny(missing_docs)]

mod color;
#[cfg(any(target_os = "linux", test))]
mod config;
mod error;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(any(target_os = "linux", test))]
mod parsers;
#[cfg(windows)]
mod windows;

pub use oito2_foundation as foundation;
pub use oito2_foundation::{
    ColorScheme, ConfigValues, Platform, RawSettings, SettingValue, SystemSettings,
};

pub use error::{Error, Result};

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
