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

//! The `org.freedesktop.appearance` keys of the XDG settings portal, read over
//! the session bus.

use gio::prelude::*;

use crate::{ColorScheme, RawSettings};

/// The bus name of the desktop portal.
pub const BUS_NAME: &str = "org.freedesktop.portal.Desktop";
/// The object path of the desktop portal.
pub const OBJECT_PATH: &str = "/org/freedesktop/portal/desktop";
/// The settings interface of the desktop portal.
pub const INTERFACE: &str = "org.freedesktop.portal.Settings";
/// The namespace of the appearance keys.
pub const APPEARANCE: &str = "org.freedesktop.appearance";
/// How long a portal call may take, in milliseconds.
const TIMEOUT: i32 = 500;

/// Reads an `org.freedesktop.appearance` key from the desktop portal,
/// unwrapping the variant it comes in. Falls back to the older `Read` method.
pub fn read_key(connection: &gio::DBusConnection, key: &str) -> Option<glib::Variant> {
    for method in ["ReadOne", "Read"] {
        let reply = connection.call_sync(
            Some(BUS_NAME),
            OBJECT_PATH,
            INTERFACE,
            method,
            Some(&(APPEARANCE, key).to_variant()),
            None,
            gio::DBusCallFlags::NONE,
            TIMEOUT,
            None::<&gio::Cancellable>,
        );
        let Ok(reply) = reply else { continue };
        // `ReadOne` answers `(v)`; `Read` answers `(v)` holding another `v`.
        let mut value = reply.try_child_value(0)?;
        // `as_variant` asserts on values that aren't boxed variants, so the
        // type is checked before each unboxing.
        while value.type_() == glib::VariantTy::VARIANT {
            value = value.as_variant()?;
        }
        return Some(value);
    }
    None
}

/// Reads the accent color, the contrast preference and the color scheme from
/// the desktop portal into `raw`. Without a session bus or a portal nothing
/// changes.
pub fn read_into(raw: &mut RawSettings) {
    let Ok(connection) = gio::bus_get_sync(gio::BusType::Session, None::<&gio::Cancellable>) else {
        return;
    };
    if let Some(accent) = read_key(&connection, "accent-color")
        && let Some((red, green, blue)) = accent.get::<(f64, f64, f64)>()
    {
        raw.system_accent = crate::color::rgb_hex(red, green, blue);
    }
    if let Some(contrast) = read_key(&connection, "contrast") {
        raw.system_high_contrast = contrast.get::<u32>() == Some(1);
    }
    if let Some(scheme) = read_key(&connection, "color-scheme") {
        raw.color_scheme = scheme.get::<u32>().and_then(ColorScheme::from_portal);
    }
}
