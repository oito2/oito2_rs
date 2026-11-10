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
//! composed from them, then, with the `watch` feature, prints the settings
//! again after each change until interrupted:
//! `cargo run -p oito2_settings --example print_settings --features watch`.

use oito2_settings::{Platform, RawSettings, SystemSettings};

/// Prints `raw` and the settings composed from it.
fn print(raw: &RawSettings) {
    println!("RawSettings:\n{raw:#?}\n");
    let settings = SystemSettings::compose(raw, Platform::current());
    println!("SystemSettings:\n{settings:#?}");
}

fn main() {
    print(&oito2_settings::read_raw_settings());
    #[cfg(feature = "watch")]
    match oito2_settings::watch(|raw| {
        println!("\n--- changed ---");
        print(raw);
    }) {
        Ok(_watcher) => {
            println!("\nWatching for changes; press Ctrl+C to stop.");
            loop {
                std::thread::park();
            }
        }
        Err(error) => eprintln!("cannot watch the settings: {error}"),
    }
}
