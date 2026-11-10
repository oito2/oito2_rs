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

//! The Linux desktop environment and its detection from the session variables.

use core::fmt;

/// A Linux desktop environment, or [`DesktopEnvironment::Unknown`] when it
/// isn't recognized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum DesktopEnvironment {
    /// GNOME, including Ubuntu's and Pop!_OS's GNOME sessions.
    Gnome,
    /// KDE Plasma.
    Kde,
    /// Xfce.
    Xfce,
    /// Budgie.
    Budgie,
    /// COSMIC.
    Cosmic,
    /// Cinnamon.
    Cinnamon,
    /// MATE.
    Mate,
    /// LXQt.
    Lxqt,
    /// Pantheon (elementary OS).
    Pantheon,
    /// Unity.
    Unity,
    /// A desktop environment [`DesktopEnvironment::detect`] doesn't recognize.
    Unknown,
}

impl DesktopEnvironment {
    /// Every desktop environment, [`DesktopEnvironment::Unknown`] last.
    pub const ALL: [Self; 11] = [
        Self::Gnome,
        Self::Kde,
        Self::Xfce,
        Self::Budgie,
        Self::Cosmic,
        Self::Cinnamon,
        Self::Mate,
        Self::Lxqt,
        Self::Pantheon,
        Self::Unity,
        Self::Unknown,
    ];

    /// The lowercase name: `gnome`, `kde`, `xfce`, `budgie`, `cosmic`,
    /// `cinnamon`, `mate`, `lxqt`, `pantheon`, `unity` or `unknown`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gnome => "gnome",
            Self::Kde => "kde",
            Self::Xfce => "xfce",
            Self::Budgie => "budgie",
            Self::Cosmic => "cosmic",
            Self::Cinnamon => "cinnamon",
            Self::Mate => "mate",
            Self::Lxqt => "lxqt",
            Self::Pantheon => "pantheon",
            Self::Unity => "unity",
            Self::Unknown => "unknown",
        }
    }

    /// The environment with the lowercase `name` of [`Self::as_str`], or
    /// `None` for any other string.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|de| de.as_str() == name)
    }

    /// Detects the desktop environment from the values of the
    /// `XDG_CURRENT_DESKTOP` and `DESKTOP_SESSION` environment variables.
    ///
    /// `xdg_current_desktop` is a colon-separated list such as `ubuntu:GNOME`;
    /// its entries are checked in order and the first recognized one wins.
    /// `desktop_session` is checked only when no entry is recognized.
    /// Matching ignores case and surrounding spaces. Returns
    /// [`DesktopEnvironment::Unknown`] when neither value is recognized.
    pub fn detect(xdg_current_desktop: Option<&str>, desktop_session: Option<&str>) -> Self {
        xdg_current_desktop
            .unwrap_or_default()
            .split(':')
            .find_map(Self::from_session_name)
            .or_else(|| Self::from_session_name(desktop_session.unwrap_or_default()))
            .unwrap_or(Self::Unknown)
    }

    /// Maps one desktop or session name, ignoring case and surrounding spaces,
    /// to its environment.
    fn from_session_name(name: &str) -> Option<Self> {
        Some(match name.trim().to_ascii_lowercase().as_str() {
            "gnome" | "gnome-classic" | "gnome-xorg" | "gnome-wayland" => Self::Gnome,
            "kde" | "plasma" | "plasmawayland" | "plasmax11" => Self::Kde,
            "xfce" | "xfce4" | "xubuntu" => Self::Xfce,
            "budgie" | "budgie-desktop" => Self::Budgie,
            "cosmic" => Self::Cosmic,
            "x-cinnamon" | "cinnamon" | "cinnamon2d" => Self::Cinnamon,
            "mate" => Self::Mate,
            "lxqt" | "lubuntu" => Self::Lxqt,
            "pantheon" => Self::Pantheon,
            "unity" | "unity7" => Self::Unity,
            _ => return None,
        })
    }
}

impl fmt::Display for DesktopEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_the_first_recognized_entry() {
        assert_eq!(
            DesktopEnvironment::detect(Some("Hyprland:plasmawayland"), None),
            DesktopEnvironment::Kde
        );
        assert_eq!(
            DesktopEnvironment::detect(Some("Unity:Unity7:ubuntu"), None),
            DesktopEnvironment::Unity
        );
        assert_eq!(
            DesktopEnvironment::detect(Some(" gnome "), Some("kde")),
            DesktopEnvironment::Gnome
        );
    }

    #[test]
    fn falls_back_to_the_session_then_unknown() {
        assert_eq!(
            DesktopEnvironment::detect(Some("Hyprland"), Some("xubuntu")),
            DesktopEnvironment::Xfce
        );
        assert_eq!(
            DesktopEnvironment::detect(None, None),
            DesktopEnvironment::Unknown
        );
        assert_eq!(
            DesktopEnvironment::detect(Some("constructor"), Some("")),
            DesktopEnvironment::Unknown
        );
    }

    #[test]
    fn names_round_trip() {
        for de in DesktopEnvironment::ALL {
            assert_eq!(DesktopEnvironment::from_name(de.as_str()), Some(de));
        }
        assert_eq!(DesktopEnvironment::from_name("plasma"), None);
    }
}
