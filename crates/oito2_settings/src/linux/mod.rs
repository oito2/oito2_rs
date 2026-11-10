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

//! Linux sources: GSettings, the XDG desktop portal and the configuration
//! files. Everything goes through GIO.

pub mod gsettings;
pub mod portal;
#[cfg(feature = "watch")]
pub mod watch;

use std::path::PathBuf;

use crate::RawSettings;

/// Reads every Linux source into a `RawSettings`: the session variables, the
/// GSettings keys, the configuration files and the portal's appearance keys.
pub fn read() -> RawSettings {
    let mut raw = RawSettings {
        xdg_current_desktop: std::env::var("XDG_CURRENT_DESKTOP").ok(),
        desktop_session: std::env::var("DESKTOP_SESSION").ok(),
        ..Default::default()
    };
    gsettings::read_into(&mut raw);
    if let Some(home) = config_home() {
        raw.config = crate::config::read_config(&home);
    }
    portal::read_into(&mut raw);
    raw
}

/// The configuration directory of this process, from `XDG_CONFIG_HOME` and
/// `HOME`.
pub fn config_home() -> Option<PathBuf> {
    crate::config::config_home(
        std::env::var("XDG_CONFIG_HOME").ok().as_deref(),
        std::env::var("HOME").ok().as_deref(),
    )
}
