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

//! The named accent colors of the GNOME palette.

use core::fmt;

/// A named accent color of the GNOME palette, as `org.gnome.desktop.interface
/// accent-color` reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum AccentColor {
    /// Blue, the default accent: `#3584E4`.
    Blue,
    /// Teal: `#2190A4`.
    Teal,
    /// Green: `#3A944A`.
    Green,
    /// Yellow: `#C88800`.
    Yellow,
    /// Orange: `#ED5B00`.
    Orange,
    /// Red: `#E62D42`.
    Red,
    /// Pink: `#D56199`.
    Pink,
    /// Purple: `#9141AC`.
    Purple,
    /// Slate: `#6F8396`.
    Slate,
}

impl AccentColor {
    /// Every accent, in palette order.
    pub const ALL: [Self; 9] = [
        Self::Blue,
        Self::Teal,
        Self::Green,
        Self::Yellow,
        Self::Orange,
        Self::Red,
        Self::Pink,
        Self::Purple,
        Self::Slate,
    ];

    /// The accent used when the system names none.
    pub const FALLBACK: Self = Self::Blue;

    /// The lowercase name, as the GSettings key holds it.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Teal => "teal",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Orange => "orange",
            Self::Red => "red",
            Self::Pink => "pink",
            Self::Purple => "purple",
            Self::Slate => "slate",
        }
    }

    /// The color as `#RRGGBB`, uppercase.
    pub fn hex(self) -> &'static str {
        match self {
            Self::Blue => "#3584E4",
            Self::Teal => "#2190A4",
            Self::Green => "#3A944A",
            Self::Yellow => "#C88800",
            Self::Orange => "#ED5B00",
            Self::Red => "#E62D42",
            Self::Pink => "#D56199",
            Self::Purple => "#9141AC",
            Self::Slate => "#6F8396",
        }
    }

    /// The accent with the exact lowercase `name` of [`Self::as_str`], or
    /// `None` for any other string.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|accent| accent.as_str() == name)
    }
}

impl fmt::Display for AccentColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        for accent in AccentColor::ALL {
            assert_eq!(AccentColor::from_name(accent.as_str()), Some(accent));
            assert_eq!(accent.hex().len(), 7);
        }
        assert_eq!(AccentColor::from_name("Blue"), None);
        assert_eq!(AccentColor::from_name("toString"), None);
    }
}
