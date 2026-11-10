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

//! Color helpers shared by the platform readers.

/// Formats red, green and blue components from 0 to 1 as `#RRGGBB`, or `None`
/// when one is outside that range (the portal reports an unset accent that
/// way).
#[cfg(any(target_os = "linux", target_os = "macos", test))]
pub fn rgb_hex(red: f64, green: f64, blue: f64) -> Option<String> {
    let channel = |value: f64| {
        (value.is_finite() && (0.0..=1.0).contains(&value)).then(|| (value * 255.0).round() as u8)
    };
    Some(format!(
        "#{:02X}{:02X}{:02X}",
        channel(red)?,
        channel(green)?,
        channel(blue)?
    ))
}

/// Whether a color is light: its weighted brightness `(5G + 2R + B) / 8`
/// is above 128, the test Windows documents for telling a light theme from a
/// dark one.
#[cfg(any(windows, test))]
pub fn is_light(red: u8, green: u8, blue: u8) -> bool {
    5 * u32::from(green) + 2 * u32::from(red) + u32::from(blue) > 8 * 128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_hex_formats_unit_components() {
        assert_eq!(rgb_hex(1.0, 0.0, 0.5).as_deref(), Some("#FF0080"));
        assert_eq!(rgb_hex(-1.0, -1.0, -1.0), None);
        assert_eq!(rgb_hex(f64::NAN, 0.0, 0.0), None);
    }

    #[test]
    fn is_light_tells_the_windows_themes_apart() {
        assert!(is_light(255, 255, 255));
        assert!(is_light(243, 243, 243));
        assert!(!is_light(0, 0, 0));
        assert!(!is_light(32, 32, 32));
        assert!(!is_light(128, 128, 128));
    }
}
