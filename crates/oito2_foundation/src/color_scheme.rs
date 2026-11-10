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

//! The light or dark scheme a system asks for.

use core::fmt;

/// The light or dark scheme the system asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum ColorScheme {
    /// Light surfaces with dark text.
    Light,
    /// Dark surfaces with light text.
    Dark,
}

impl ColorScheme {
    /// Both schemes, light first.
    pub const ALL: [Self; 2] = [Self::Light, Self::Dark];

    /// The lowercase name: `light` or `dark`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    /// The scheme with the lowercase `name` of [`Self::as_str`], or `None` for
    /// any other string.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|scheme| scheme.as_str() == name)
    }

    /// The scheme the `org.freedesktop.appearance color-scheme` portal value
    /// asks for: 1 is dark, 2 is light, 0 (and any other value) no preference.
    pub fn from_portal(value: u32) -> Option<Self> {
        match value {
            1 => Some(Self::Dark),
            2 => Some(Self::Light),
            _ => None,
        }
    }

    /// The scheme the `org.gnome.desktop.interface color-scheme` value asks
    /// for: `prefer-dark`, `prefer-light`, or no preference for `default`
    /// and any other value.
    pub fn from_gnome_preference(value: &str) -> Option<Self> {
        match value {
            "prefer-dark" => Some(Self::Dark),
            "prefer-light" => Some(Self::Light),
            _ => None,
        }
    }
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_the_portal_and_gnome_values() {
        assert_eq!(ColorScheme::from_portal(0), None);
        assert_eq!(ColorScheme::from_portal(1), Some(ColorScheme::Dark));
        assert_eq!(ColorScheme::from_portal(2), Some(ColorScheme::Light));
        assert_eq!(ColorScheme::from_portal(7), None);
        assert_eq!(
            ColorScheme::from_gnome_preference("prefer-dark"),
            Some(ColorScheme::Dark)
        );
        assert_eq!(
            ColorScheme::from_gnome_preference("prefer-light"),
            Some(ColorScheme::Light)
        );
        assert_eq!(ColorScheme::from_gnome_preference("default"), None);
    }

    #[test]
    fn names_round_trip() {
        for scheme in ColorScheme::ALL {
            assert_eq!(ColorScheme::from_name(scheme.as_str()), Some(scheme));
        }
    }
}
