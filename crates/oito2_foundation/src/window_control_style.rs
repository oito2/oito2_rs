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

//! The visual family of the window control buttons.

use core::fmt;

use crate::{DesktopEnvironment, Platform};

/// The visual family used to draw window control buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum WindowControlStyle {
    /// Round buttons on a neutral background, as in GNOME and its relatives.
    Gnome,
    /// Flat glyph buttons, as in KDE Plasma's Breeze decoration.
    Breeze,
    /// Compact glyph buttons, as in Xfce's window manager.
    Xfwm,
    /// Round buttons, as in COSMIC.
    Cosmic,
    /// Rectangular buttons with a red close hover, as in Windows.
    Windows,
    /// The native macOS traffic lights.
    Macos,
    /// No window controls: the browser or the mobile system owns the window.
    None,
}

impl WindowControlStyle {
    /// Every style, [`WindowControlStyle::None`] last.
    pub const ALL: [Self; 7] = [
        Self::Gnome,
        Self::Breeze,
        Self::Xfwm,
        Self::Cosmic,
        Self::Windows,
        Self::Macos,
        Self::None,
    ];

    /// The lowercase name: `gnome`, `breeze`, `xfwm`, `cosmic`, `windows`,
    /// `macos` or `none`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gnome => "gnome",
            Self::Breeze => "breeze",
            Self::Xfwm => "xfwm",
            Self::Cosmic => "cosmic",
            Self::Windows => "windows",
            Self::Macos => "macos",
            Self::None => "none",
        }
    }

    /// The style with the lowercase `name` of [`Self::as_str`], or `None` for
    /// any other string.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|style| style.as_str() == name)
    }

    /// Resolves the control style for `platform` and, on Linux, for
    /// `desktop_environment`.
    ///
    /// Returns [`WindowControlStyle::None`] on the web and on touch
    /// platforms. Linux desktop environments without their own style use
    /// [`WindowControlStyle::Gnome`].
    pub fn resolve(platform: Platform, desktop_environment: Option<DesktopEnvironment>) -> Self {
        if !platform.is_desktop_native() {
            Self::None
        } else if platform.is_windows() {
            Self::Windows
        } else if platform.is_macos() {
            Self::Macos
        } else {
            match desktop_environment {
                Some(DesktopEnvironment::Kde) => Self::Breeze,
                Some(DesktopEnvironment::Xfce) => Self::Xfwm,
                Some(DesktopEnvironment::Cosmic) => Self::Cosmic,
                _ => Self::Gnome,
            }
        }
    }
}

impl fmt::Display for WindowControlStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OperatingSystem;

    #[test]
    fn names_round_trip() {
        for style in WindowControlStyle::ALL {
            assert_eq!(WindowControlStyle::from_name(style.as_str()), Some(style));
        }
    }

    #[test]
    fn linux_without_an_environment_uses_gnome() {
        let linux = Platform::new(OperatingSystem::Linux, false);
        assert_eq!(
            WindowControlStyle::resolve(linux, None),
            WindowControlStyle::Gnome
        );
        assert_eq!(
            WindowControlStyle::resolve(linux, Some(DesktopEnvironment::Lxqt)),
            WindowControlStyle::Gnome
        );
    }
}
