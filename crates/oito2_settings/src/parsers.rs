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

//! Extracts values from the desktop configuration files, like the Dart
//! `Oito2ConfigParsers`.

/// Returns the value of `key` in the INI `group` of `content`, with surrounding spaces
/// trimmed, or `None` when the group or key is missing. Lines starting with `#` or `;`
/// are comments; KDE's nested groups such as `[Windows][Rules]` are named
/// `Windows][Rules`.
pub fn ini_value(content: &str, group: &str, key: &str) -> Option<String> {
    let mut current: Option<&str> = None;
    for raw in content.split('\n') {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if line.len() >= 2 && line.starts_with('[') && line.ends_with(']') {
            current = line.get(1..line.len() - 1);
            continue;
        }
        if current != Some(group) {
            continue;
        }
        let Some(separator) = line.find('=') else {
            continue;
        };
        if separator == 0 {
            continue;
        }
        if line[..separator].trim() == key {
            return Some(line[separator + 1..].trim().to_owned());
        }
    }
    None
}

/// Returns the `value` attribute of the xfconf `<property name="…">` element called
/// `name`, with XML entities unescaped, or `None` when there is none.
pub fn xfconf_property(content: &str, name: &str) -> Option<String> {
    let needle = format!("<property name=\"{name}\"");
    let mut rest = content;
    while let Some(start) = rest.find(&needle) {
        let element = &rest[start..];
        let end = element.find('>').unwrap_or(element.len());
        let tag = &element[..end];
        if let Some(position) = tag.find(" value=\"") {
            let value = &tag[position + 8..];
            if let Some(close) = value.find('"') {
                return Some(unescape_xml(&value[..close]));
            }
        }
        rest = &element[needle.len()..];
    }
    None
}

/// Parses a COSMIC boolean setting file: `true` or `false`, ignoring surrounding space.
pub fn cosmic_bool(content: &str) -> Option<bool> {
    match content.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

/// Parses a KDE `r,g,b` color into `#RRGGBB`, or `None` unless it has three components
/// from 0 to 255.
pub fn kde_color(value: &str) -> Option<String> {
    let parts: Vec<u8> = value
        .split(',')
        .map(|part| part.trim().parse::<u8>())
        .collect::<Result<_, _>>()
        .ok()?;
    match parts.as_slice() {
        [r, g, b] => Some(format!("#{r:02X}{g:02X}{b:02X}")),
        _ => None,
    }
}

fn unescape_xml(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    const KWINRC: &str = "# KWin settings
[Desktops]
Number=4

[org.kde.kdecoration2]
BorderSize=Normal
ButtonsOnLeft = XIA
ButtonsOnRight=M
library=org.kde.breeze

[Windows][Rules]
ButtonsOnLeft=ignored
";

    const XFWM: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<channel name="xfwm4" version="1.0">
  <property name="general" type="empty">
    <property name="box_move" type="bool" value="false"/>
    <property name="button_layout" type="string" value="CHM|O"/>
    <property name="title_font" type="string" value="Sans &amp; Serif Bold 9"/>
  </property>
</channel>
"#;

    #[test]
    fn ini_reads_a_key_in_its_group_and_trims_spaces() {
        let group = "org.kde.kdecoration2";
        assert_eq!(
            ini_value(KWINRC, group, "ButtonsOnLeft").as_deref(),
            Some("XIA")
        );
        assert_eq!(
            ini_value(KWINRC, group, "ButtonsOnRight").as_deref(),
            Some("M")
        );
    }

    #[test]
    fn ini_keeps_groups_apart_and_supports_nested_kde_groups() {
        assert_eq!(
            ini_value(KWINRC, "Desktops", "Number").as_deref(),
            Some("4")
        );
        assert_eq!(
            ini_value(KWINRC, "Windows][Rules", "ButtonsOnLeft").as_deref(),
            Some("ignored")
        );
    }

    #[test]
    fn ini_returns_none_for_missing_groups_keys_and_comments() {
        assert_eq!(ini_value(KWINRC, "Nope", "Number"), None);
        assert_eq!(ini_value(KWINRC, "Desktops", "Missing"), None);
        assert_eq!(ini_value("", "Desktops", "Number"), None);
    }

    #[test]
    fn xfconf_reads_unescapes_and_misses() {
        assert_eq!(
            xfconf_property(XFWM, "button_layout").as_deref(),
            Some("CHM|O")
        );
        assert_eq!(
            xfconf_property(XFWM, "title_font").as_deref(),
            Some("Sans & Serif Bold 9")
        );
        assert_eq!(xfconf_property(XFWM, "missing"), None);
        // A property without a value (a group) is skipped.
        assert_eq!(xfconf_property(XFWM, "general"), None);
    }

    #[test]
    fn cosmic_bool_parses_true_and_false_only() {
        assert_eq!(cosmic_bool("true\n"), Some(true));
        assert_eq!(cosmic_bool(" false "), Some(false));
        assert_eq!(cosmic_bool("maybe"), None);
    }

    #[test]
    fn kde_color_parses_rgb_triples() {
        assert_eq!(kde_color("61,174,233").as_deref(), Some("#3DAEE9"));
        assert_eq!(kde_color(" 1, 2, 3 ").as_deref(), Some("#010203"));
        assert_eq!(kde_color("256,0,0"), None);
        assert_eq!(kde_color("1,2"), None);
        assert_eq!(kde_color("red"), None);
    }
}
