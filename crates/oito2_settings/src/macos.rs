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

//! macOS sources: the accent color and the "Increase contrast" preference
//! through AppKit, and the appearance through the user defaults.

use objc2_app_kit::{NSColor, NSColorSpace, NSWorkspace};
use objc2_foundation::{NSString, NSUserDefaults};

use crate::{ColorScheme, RawSettings};

/// Reads the system accent color in sRGB, the "Increase contrast" preference
/// and the color scheme: dark when the `AppleInterfaceStyle` user default is
/// `Dark`, light otherwise.
pub fn read() -> RawSettings {
    let system_accent = NSColor::controlAccentColor()
        .colorUsingColorSpace(&NSColorSpace::sRGBColorSpace())
        .and_then(|color| {
            crate::color::rgb_hex(
                color.redComponent(),
                color.greenComponent(),
                color.blueComponent(),
            )
        });
    let system_high_contrast =
        NSWorkspace::sharedWorkspace().accessibilityDisplayShouldIncreaseContrast();
    let style = NSUserDefaults::standardUserDefaults()
        .stringForKey(&NSString::from_str("AppleInterfaceStyle"));
    let color_scheme = Some(if style.is_some_and(|style| style.to_string() == "Dark") {
        ColorScheme::Dark
    } else {
        ColorScheme::Light
    });
    RawSettings {
        system_accent,
        system_high_contrast,
        color_scheme,
        ..Default::default()
    }
}
