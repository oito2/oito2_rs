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

//! The operating system and the ergonomics that follow from it.

use core::fmt;

/// An operating system: the native one, or the browser host's on the web.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum OperatingSystem {
    /// Android.
    Android,
    /// Fuchsia.
    Fuchsia,
    /// iOS, including iPadOS.
    Ios,
    /// Linux.
    Linux,
    /// macOS.
    Macos,
    /// Windows.
    Windows,
}

impl OperatingSystem {
    /// Every operating system, in name order.
    pub const ALL: [Self; 6] = [
        Self::Android,
        Self::Fuchsia,
        Self::Ios,
        Self::Linux,
        Self::Macos,
        Self::Windows,
    ];

    /// The operating system this program was compiled for. Targets outside
    /// the six known systems count as Linux.
    pub fn current() -> Self {
        if cfg!(target_os = "android") {
            Self::Android
        } else if cfg!(target_os = "fuchsia") {
            Self::Fuchsia
        } else if cfg!(target_os = "ios") {
            Self::Ios
        } else if cfg!(target_os = "macos") {
            Self::Macos
        } else if cfg!(windows) {
            Self::Windows
        } else {
            Self::Linux
        }
    }

    /// The lowercase name: `android`, `fuchsia`, `ios`, `linux`, `macos` or
    /// `windows`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Fuchsia => "fuchsia",
            Self::Ios => "ios",
            Self::Linux => "linux",
            Self::Macos => "macos",
            Self::Windows => "windows",
        }
    }

    /// The operating system with the lowercase `name` of [`Self::as_str`], or
    /// `None` for any other string.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|os| os.as_str() == name)
    }

    /// Whether the system is Linux, Windows or macOS.
    fn is_desktop(self) -> bool {
        matches!(self, Self::Linux | Self::Macos | Self::Windows)
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The input density of a platform: pointer or touch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Density {
    /// A mouse or trackpad drives the interface.
    Pointer,
    /// Fingers drive the interface, so controls are larger.
    Touch,
}

/// The platform an app runs on and the ergonomics that follow from it.
///
/// [`Platform::current`] describes the running program; the fields are public
/// so tests and previews can describe any platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Platform {
    /// The operating system, or the browser host's on the web.
    pub os: OperatingSystem,
    /// Whether the app runs in a web browser instead of a native window.
    pub is_web: bool,
}

impl Platform {
    /// Describes the platform of `os`, in a browser when `is_web` is true.
    pub const fn new(os: OperatingSystem, is_web: bool) -> Self {
        Self { os, is_web }
    }

    /// The running platform: the compiled-for operating system, outside a
    /// browser.
    pub fn current() -> Self {
        Self::new(OperatingSystem::current(), false)
    }

    /// Whether the app runs as a native desktop app (Linux, Windows or macOS).
    pub fn is_desktop_native(self) -> bool {
        !self.is_web && self.os.is_desktop()
    }

    /// Whether the app runs natively on a touch-first system (Android, iOS or
    /// Fuchsia).
    pub fn is_touch(self) -> bool {
        !self.is_web && !self.os.is_desktop()
    }

    /// Whether the app runs in a browser on a touch-first system.
    pub fn is_web_touch(self) -> bool {
        self.is_web && !self.os.is_desktop()
    }

    /// Whether the primary input is touch, natively or in a browser.
    pub fn uses_touch_input(self) -> bool {
        self.is_touch() || self.is_web_touch()
    }

    /// Whether the operating system is Linux.
    pub fn is_linux(self) -> bool {
        self.os == OperatingSystem::Linux
    }

    /// Whether the operating system is Windows.
    pub fn is_windows(self) -> bool {
        self.os == OperatingSystem::Windows
    }

    /// Whether the operating system is macOS.
    pub fn is_macos(self) -> bool {
        self.os == OperatingSystem::Macos
    }

    /// Whether the operating system is Android.
    pub fn is_android(self) -> bool {
        self.os == OperatingSystem::Android
    }

    /// Whether the operating system is iOS.
    pub fn is_ios(self) -> bool {
        self.os == OperatingSystem::Ios
    }

    /// The density for this platform: touch for touch input, pointer otherwise.
    pub fn density(self) -> Density {
        if self.uses_touch_input() {
            Density::Touch
        } else {
            Density::Pointer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        for os in OperatingSystem::ALL {
            assert_eq!(OperatingSystem::from_name(os.as_str()), Some(os));
        }
        assert_eq!(OperatingSystem::from_name("Linux"), None);
    }

    #[test]
    fn the_current_platform_is_native() {
        let platform = Platform::current();
        assert!(!platform.is_web);
        assert_eq!(platform.os, OperatingSystem::current());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serializes_as_os_and_is_web() {
        let platform = Platform::new(OperatingSystem::Macos, true);
        let json = serde_json::to_value(platform).unwrap();
        assert_eq!(json, serde_json::json!({ "os": "macos", "isWeb": true }));
        let back: Platform = serde_json::from_value(json).unwrap();
        assert_eq!(back, platform);
    }
}
