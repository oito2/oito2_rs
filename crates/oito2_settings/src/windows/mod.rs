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

//! Windows sources: the accent color, the high-contrast preference and the
//! theme, through the Windows Runtime.

#[cfg(feature = "watch")]
pub mod watch;

use windows::UI::ViewManagement::{AccessibilitySettings, UIColorType, UISettings};

use crate::{ColorScheme, RawSettings};

/// Reads the system accent color, the high-contrast preference and the color
/// scheme (dark when the system background color is dark).
pub fn read() -> RawSettings {
    let settings = UISettings::new().ok();
    let color = |kind| {
        settings
            .as_ref()
            .and_then(|settings| settings.GetColorValue(kind).ok())
    };
    let system_accent =
        color(UIColorType::Accent).map(|c| format!("#{:02X}{:02X}{:02X}", c.R, c.G, c.B));
    let color_scheme = color(UIColorType::Background).map(|c| {
        if crate::color::is_light(c.R, c.G, c.B) {
            ColorScheme::Light
        } else {
            ColorScheme::Dark
        }
    });
    let system_high_contrast = AccessibilitySettings::new()
        .and_then(|settings| settings.HighContrast())
        .unwrap_or(false);
    RawSettings {
        system_accent,
        system_high_contrast,
        color_scheme,
        ..Default::default()
    }
}
