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

//! Platform rules shared by the OITO2 user interfaces: desktop environment
//! detection, window button layout, window control style, decoration mode and
//! the composition of system settings. Pure Rust: no I/O, no toolkit bindings.
//!
//! The rules mirror the `oito2_ui` Flutter packages, the source of truth, and
//! are checked against its parity fixture. [`RawSettings`] holds what a
//! platform reader collects (the `oito2_settings` crate does the reading);
//! [`SystemSettings::compose`] turns it into the settings an app follows, and
//! [`SystemSettings::fallback`] derives them from the platform alone.
//!
//! With the `serde` feature (on by default) every type serializes to the JSON
//! the OITO2 Tauri plugins emit: lowercase enum names, camelCase fields,
//! `DecorationMode` in kebab-case.

#![deny(missing_docs)]

mod accent_color;
mod color_scheme;
mod decoration_mode;
mod desktop_environment;
mod platform;
pub mod raw;
mod system_settings;
mod window_button_layout;
mod window_control_style;
pub mod window_menu;

pub use accent_color::AccentColor;
pub use color_scheme::ColorScheme;
pub use decoration_mode::{DecorationMode, ParseDecorationModeError};
pub use desktop_environment::DesktopEnvironment;
pub use platform::{Density, OperatingSystem, Platform};
pub use raw::{ConfigValues, RawSettings, SettingValue};
pub use system_settings::SystemSettings;
pub use window_button_layout::{WindowButton, WindowButtonLayout};
pub use window_control_style::WindowControlStyle;
