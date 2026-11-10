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

//! The readers on the machine running the tests: they never fail and their
//! result composes for the current platform.

use oito2_settings::{Platform, SystemSettings};

#[test]
fn reads_and_composes_for_the_current_platform() {
    let raw = oito2_settings::read_raw_settings();
    let settings = SystemSettings::compose(&raw, Platform::current());
    assert_eq!(settings.platform, Platform::current());
    assert_eq!(
        settings.window_control_style,
        SystemSettings::fallback(Platform::current(), settings.desktop_environment)
            .window_control_style
    );
    let read = oito2_settings::read_system_settings();
    assert_eq!(read.platform, Platform::current());
}

/// Prints what the readers see on this machine, for checking a desktop by
/// hand: `cargo test -p oito2_settings --test read -- --ignored --nocapture`.
#[test]
#[ignore = "reads the real desktop session"]
fn prints_the_settings_of_this_machine() {
    println!("{:#?}", oito2_settings::read_raw_settings());
    println!("{:#?}", oito2_settings::read_system_settings());
}
