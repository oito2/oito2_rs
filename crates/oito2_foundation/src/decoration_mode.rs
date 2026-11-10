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

//! How the window frame is drawn.

use core::fmt;
use std::str::FromStr;

/// How the window frame is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum DecorationMode {
    /// The system title bar and frame.
    Native,
    /// No system frame at all: the app draws the title bar and its own resize
    /// handles.
    Undecorated,
    /// GTK client-side decorations with an empty title bar: GTK keeps the
    /// window shadow and resize edges, the app draws the title bar. Falls back
    /// to [`DecorationMode::Undecorated`] outside Linux.
    GtkHeaderless,
}

impl DecorationMode {
    /// Every mode, in declaration order.
    pub const ALL: [Self; 3] = [Self::Native, Self::Undecorated, Self::GtkHeaderless];

    /// The mode used when none is requested: GTK client-side decorations on
    /// Linux, the native frame on macOS, no system frame elsewhere.
    pub fn platform_default() -> Self {
        if cfg!(target_os = "linux") {
            Self::GtkHeaderless
        } else if cfg!(target_os = "macos") {
            Self::Native
        } else {
            Self::Undecorated
        }
    }

    /// The name accepted by [`FromStr`]: `native`, `undecorated` or
    /// `gtk-headerless`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Undecorated => "undecorated",
            Self::GtkHeaderless => "gtk-headerless",
        }
    }
}

impl fmt::Display for DecorationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The error of parsing a [`DecorationMode`] from a name it doesn't have.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("decoration mode `{name}`")]
pub struct ParseDecorationModeError {
    /// The rejected name, trimmed and lowercased.
    pub name: String,
}

impl FromStr for DecorationMode {
    type Err = ParseDecorationModeError;

    /// Parses `native`, `undecorated` or `gtk-headerless` (case-insensitive,
    /// surrounding spaces ignored).
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "native" => Ok(Self::Native),
            "undecorated" => Ok(Self::Undecorated),
            "gtk-headerless" | "gtkheaderless" => Ok(Self::GtkHeaderless),
            other => Err(ParseDecorationModeError {
                name: other.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_mode() {
        assert_eq!(
            "native".parse::<DecorationMode>().unwrap(),
            DecorationMode::Native
        );
        assert_eq!(
            " Undecorated ".parse::<DecorationMode>().unwrap(),
            DecorationMode::Undecorated
        );
        assert_eq!(
            "gtk-headerless".parse::<DecorationMode>().unwrap(),
            DecorationMode::GtkHeaderless
        );
        assert_eq!(
            "GtkHeaderless".parse::<DecorationMode>().unwrap(),
            DecorationMode::GtkHeaderless
        );
    }

    #[test]
    fn names_round_trip() {
        for mode in DecorationMode::ALL {
            assert_eq!(mode.as_str().parse::<DecorationMode>().unwrap(), mode);
            assert_eq!(mode.to_string(), mode.as_str());
        }
    }

    #[test]
    fn rejects_unknown_modes_with_their_name() {
        let error = " Frameless ".parse::<DecorationMode>().unwrap_err();
        assert_eq!(error.name, "frameless");
        assert_eq!(error.to_string(), "decoration mode `frameless`");
    }

    #[test]
    fn defaults_per_platform() {
        let expected = if cfg!(target_os = "linux") {
            DecorationMode::GtkHeaderless
        } else if cfg!(target_os = "macos") {
            DecorationMode::Native
        } else {
            DecorationMode::Undecorated
        };
        assert_eq!(DecorationMode::platform_default(), expected);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serializes_as_the_parsed_names() {
        for mode in DecorationMode::ALL {
            let json = serde_json::to_string(&mode).unwrap();
            assert_eq!(json, format!("\"{}\"", mode.as_str()));
            let back: DecorationMode = serde_json::from_str(&json).unwrap();
            assert_eq!(back, mode);
        }
    }
}
