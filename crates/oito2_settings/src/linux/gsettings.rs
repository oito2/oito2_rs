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

//! The GSettings keys, read through GIO.

use gio::prelude::*;
use oito2_foundation::raw::GSETTINGS_KEYS;

use crate::{RawSettings, SettingValue};

/// Reads the GSettings keys of [`GSETTINGS_KEYS`] whose schema and key are
/// installed into `raw.gsettings`, by `schema/key`. String and boolean values
/// are kept; keys of other types are skipped.
pub fn read_into(raw: &mut RawSettings) {
    let Some(source) = gio::SettingsSchemaSource::default() else {
        return;
    };
    for (schema_id, key) in GSETTINGS_KEYS {
        // GIO aborts the process on an unknown schema or key, so both are
        // checked first.
        let Some(schema) = source.lookup(schema_id, true) else {
            continue;
        };
        if !schema.has_key(key) {
            continue;
        }
        let value = gio::Settings::new(schema_id).value(key);
        let value = if let Some(text) = value.str() {
            SettingValue::Text(text.to_owned())
        } else if let Some(flag) = value.get::<bool>() {
            SettingValue::Flag(flag)
        } else {
            continue;
        };
        raw.gsettings.insert(format!("{schema_id}/{key}"), value);
    }
}

#[cfg(test)]
/// The GSettings schemas that hold watched keys, each once, in sorted order.
pub fn watched_schemas() -> Vec<&'static str> {
    let mut schemas: Vec<&str> = GSETTINGS_KEYS.iter().map(|(schema, _)| *schema).collect();
    schemas.sort_unstable();
    schemas.dedup();
    schemas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_each_watched_schema_once() {
        let schemas = watched_schemas();
        assert_eq!(
            schemas,
            vec![
                "org.cinnamon.desktop.wm.preferences",
                "org.gnome.desktop.a11y.interface",
                "org.gnome.desktop.interface",
                "org.gnome.desktop.wm.preferences",
                "org.mate.Marco.general",
            ]
        );
        for (schema, _) in GSETTINGS_KEYS {
            assert!(schemas.contains(schema));
        }
    }

    #[test]
    fn reads_only_installed_schemas() {
        let mut raw = RawSettings::default();
        read_into(&mut raw);
        for key in raw.gsettings.keys() {
            let (schema, key) = key.split_once('/').expect("schema/key");
            assert!(GSETTINGS_KEYS.contains(&(schema, key)), "{schema}/{key}");
        }
    }
}
