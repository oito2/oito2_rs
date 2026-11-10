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

//! The native window menu: where it exists and which positions it accepts.

/// The largest coordinate accepted for the window menu position, in CSS
/// pixels.
pub const MAX_COORDINATE: f64 = 100_000.0;

/// Whether this platform has a native window menu an app can show: Linux and
/// Windows.
pub fn supported() -> bool {
    cfg!(any(target_os = "linux", windows))
}

/// The error of a window menu position outside a plausible window.
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
#[error("menu position ({x}, {y})")]
pub struct InvalidPositionError {
    /// The rejected horizontal coordinate.
    pub x: f64,
    /// The rejected vertical coordinate.
    pub y: f64,
}

/// Checks that a menu position is a finite point inside a plausible window:
/// both coordinates between zero and [`MAX_COORDINATE`].
pub fn validate_position(x: f64, y: f64) -> Result<(f64, f64), InvalidPositionError> {
    let valid = |value: f64| value.is_finite() && (0.0..=MAX_COORDINATE).contains(&value);
    if valid(x) && valid(y) {
        Ok((x, y))
    } else {
        Err(InvalidPositionError { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_positions_inside_a_window() {
        assert_eq!(validate_position(0.0, 0.0), Ok((0.0, 0.0)));
        assert_eq!(validate_position(320.5, 23.0), Ok((320.5, 23.0)));
    }

    #[test]
    fn rejects_negative_huge_or_invalid_positions() {
        assert!(validate_position(-1.0, 10.0).is_err());
        assert!(validate_position(10.0, 1e9).is_err());
        assert!(validate_position(f64::NAN, 10.0).is_err());
        assert!(validate_position(10.0, f64::INFINITY).is_err());
        assert_eq!(
            validate_position(-1.0, 10.0).unwrap_err().to_string(),
            "menu position (-1, 10)"
        );
    }

    #[test]
    fn support_follows_the_platform() {
        assert_eq!(supported(), cfg!(any(target_os = "linux", windows)));
    }
}
