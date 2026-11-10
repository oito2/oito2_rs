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

//! Prints what this machine's settings stores hold and the system settings
//! composed from them:
//! `cargo run -p oito2_settings --example print_settings`.

use oito2_settings::{Platform, SystemSettings};

fn main() {
    let raw = oito2_settings::read_raw_settings();
    println!("RawSettings:\n{raw:#?}\n");
    let settings = SystemSettings::compose(&raw, Platform::current());
    println!("SystemSettings:\n{settings:#?}");
}
