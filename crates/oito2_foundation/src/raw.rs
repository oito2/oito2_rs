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

//! What a platform reader collects before the settings are composed.

use std::collections::BTreeMap;

use crate::ColorScheme;

/// A GSettings value: a string or a boolean.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SettingValue {
    /// A string value.
    Text(String),
    /// A boolean value.
    Flag(bool),
}

/// The values the desktop configuration files hold, already extracted.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default, rename_all = "camelCase"))]
pub struct ConfigValues {
    /// `ButtonsOnLeft` of `[org.kde.kdecoration2]` in `kwinrc`.
    pub kwin_buttons_on_left: Option<String>,
    /// `ButtonsOnRight` of `[org.kde.kdecoration2]` in `kwinrc`.
    pub kwin_buttons_on_right: Option<String>,
    /// `AccentColor` of `[General]` in `kdeglobals`, as `#RRGGBB`.
    pub kde_accent_color: Option<String>,
    /// `ColorScheme` of `[General]` in `kdeglobals`.
    pub kde_color_scheme: Option<String>,
    /// `LookAndFeelPackage` of `[KDE]` in `kdeglobals`.
    pub kde_look_and_feel_package: Option<String>,
    /// The `button_layout` property of `xfwm4.xml`.
    pub xfwm_button_layout: Option<String>,
    /// COSMIC's `show_minimize` flag.
    pub cosmic_show_minimize: Option<bool>,
    /// COSMIC's `show_maximize` flag.
    pub cosmic_show_maximize: Option<bool>,
    /// COSMIC's `is_dark` theme mode flag.
    pub cosmic_is_dark: Option<bool>,
}

/// Everything a platform reader collects: the session variables, the
/// GSettings keys, the configuration values and the system's accent,
/// contrast and color scheme. [`SystemSettings::compose`] turns it into a
/// [`SystemSettings`].
///
/// [`SystemSettings`]: crate::SystemSettings
/// [`SystemSettings::compose`]: crate::SystemSettings::compose
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default, rename_all = "camelCase"))]
pub struct RawSettings {
    /// `XDG_CURRENT_DESKTOP`.
    pub xdg_current_desktop: Option<String>,
    /// `DESKTOP_SESSION`.
    pub desktop_session: Option<String>,
    /// GSettings values by `schema/key`, for the keys whose schema is
    /// installed.
    pub gsettings: BTreeMap<String, SettingValue>,
    /// Values from `kwinrc`, `kdeglobals`, `xfwm4.xml` and the COSMIC
    /// settings.
    pub config: ConfigValues,
    /// The accent color of the toolkit or system (the settings portal on
    /// Linux), as `#RRGGBB`.
    pub system_accent: Option<String>,
    /// Whether the system asks for high contrast.
    pub system_high_contrast: bool,
    /// The scheme the system asks for: the settings portal on Linux, the
    /// system appearance elsewhere; `None` when it states no preference.
    pub color_scheme: Option<ColorScheme>,
}

impl RawSettings {
    /// The string value of `key` in `schema`, if it was read and is a string.
    pub fn gsetting_text(&self, schema: &str, key: &str) -> Option<&str> {
        match self.gsettings.get(&format!("{schema}/{key}")) {
            Some(SettingValue::Text(text)) => Some(text),
            _ => None,
        }
    }

    /// The boolean value of `key` in `schema`, if it was read and is a
    /// boolean.
    pub fn gsetting_flag(&self, schema: &str, key: &str) -> Option<bool> {
        match self.gsettings.get(&format!("{schema}/{key}")) {
            Some(SettingValue::Flag(flag)) => Some(*flag),
            _ => None,
        }
    }
}

/// The GSettings schemas the settings come from.
pub mod schemas {
    /// Window manager preferences of GNOME, Budgie, Pantheon and Unity.
    pub const GNOME_WM: &str = "org.gnome.desktop.wm.preferences";
    /// Window manager preferences of Cinnamon.
    pub const CINNAMON_WM: &str = "org.cinnamon.desktop.wm.preferences";
    /// Window manager preferences of MATE.
    pub const MATE_WM: &str = "org.mate.Marco.general";
    /// Interface preferences of GNOME: accent, GTK theme and color scheme.
    pub const GNOME_INTERFACE: &str = "org.gnome.desktop.interface";
    /// Accessibility preferences of GNOME.
    pub const GNOME_A11Y: &str = "org.gnome.desktop.a11y.interface";
}

/// The GSettings keys a reader collects, by schema.
pub const GSETTINGS_KEYS: &[(&str, &str)] = &[
    (schemas::GNOME_WM, "button-layout"),
    (schemas::CINNAMON_WM, "button-layout"),
    (schemas::MATE_WM, "button-layout"),
    (schemas::GNOME_INTERFACE, "accent-color"),
    (schemas::GNOME_INTERFACE, "gtk-theme"),
    (schemas::GNOME_INTERFACE, "color-scheme"),
    (schemas::GNOME_A11Y, "high-contrast"),
];

/// The configuration files a reader collects, relative to `XDG_CONFIG_HOME`.
pub mod files {
    /// KWin's settings, with the decoration buttons.
    pub const KWINRC: &str = "kwinrc";
    /// KDE's global settings, with the accent, color scheme and look-and-feel.
    pub const KDEGLOBALS: &str = "kdeglobals";
    /// Xfwm4's xfconf channel, with the button layout.
    pub const XFWM: &str = "xfce4/xfconf/xfce-perchannel-xml/xfwm4.xml";
    /// COSMIC's `show_minimize` flag.
    pub const COSMIC_MINIMIZE: &str = "cosmic/com.system76.CosmicTk/v1/show_minimize";
    /// COSMIC's `show_maximize` flag.
    pub const COSMIC_MAXIMIZE: &str = "cosmic/com.system76.CosmicTk/v1/show_maximize";
    /// COSMIC's `is_dark` theme mode flag.
    pub const COSMIC_DARK: &str = "cosmic/com.system76.CosmicTheme.Mode/v1/is_dark";
    /// Every file, in the order above.
    pub const ALL: [&str; 6] = [
        KWINRC,
        KDEGLOBALS,
        XFWM,
        COSMIC_MINIMIZE,
        COSMIC_MAXIMIZE,
        COSMIC_DARK,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_typed_gsettings_values() {
        let mut raw = RawSettings::default();
        raw.gsettings.insert(
            "org.gnome.desktop.interface/gtk-theme".into(),
            SettingValue::Text("Adwaita".into()),
        );
        raw.gsettings.insert(
            "org.gnome.desktop.a11y.interface/high-contrast".into(),
            SettingValue::Flag(true),
        );
        assert_eq!(
            raw.gsetting_text(schemas::GNOME_INTERFACE, "gtk-theme"),
            Some("Adwaita")
        );
        assert_eq!(
            raw.gsetting_flag(schemas::GNOME_INTERFACE, "gtk-theme"),
            None
        );
        assert_eq!(
            raw.gsetting_flag(schemas::GNOME_A11Y, "high-contrast"),
            Some(true)
        );
        assert_eq!(
            raw.gsetting_text(schemas::GNOME_A11Y, "high-contrast"),
            None
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serializes_for_the_webview() {
        let mut raw = RawSettings {
            xdg_current_desktop: Some("GNOME".into()),
            ..Default::default()
        };
        raw.gsettings.insert(
            "org.gnome.desktop.a11y.interface/high-contrast".into(),
            SettingValue::Flag(true),
        );
        raw.config.cosmic_show_minimize = Some(false);
        raw.color_scheme = Some(ColorScheme::Dark);
        let json = serde_json::to_value(&raw).unwrap();
        assert_eq!(json["xdgCurrentDesktop"], "GNOME");
        assert_eq!(json["desktopSession"], serde_json::Value::Null);
        assert_eq!(
            json["gsettings"]["org.gnome.desktop.a11y.interface/high-contrast"],
            true
        );
        assert_eq!(json["config"]["cosmicShowMinimize"], false);
        assert_eq!(
            json["config"]["kdeLookAndFeelPackage"],
            serde_json::Value::Null
        );
        assert_eq!(json["systemAccent"], serde_json::Value::Null);
        assert_eq!(json["systemHighContrast"], false);
        assert_eq!(json["colorScheme"], "dark");
        let back: RawSettings = serde_json::from_value(json).unwrap();
        assert_eq!(back, raw);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserializes_the_payload_of_older_readers() {
        let json = r##"{
            "xdgCurrentDesktop": "KDE",
            "desktopSession": null,
            "gsettings": { "org.gnome.desktop.interface/accent-color": "teal" },
            "config": { "kwinButtonsOnLeft": "MS", "kwinButtonsOnRight": "HIAX",
                        "kdeAccentColor": null, "kdeColorScheme": null, "xfwmButtonLayout": null,
                        "cosmicShowMinimize": null, "cosmicShowMaximize": null },
            "systemAccent": "#3DAEE9",
            "systemHighContrast": false
        }"##;
        let raw: RawSettings = serde_json::from_str(json).unwrap();
        assert_eq!(raw.xdg_current_desktop.as_deref(), Some("KDE"));
        assert_eq!(raw.config.kwin_buttons_on_left.as_deref(), Some("MS"));
        assert_eq!(raw.config.kde_look_and_feel_package, None);
        assert_eq!(raw.config.cosmic_is_dark, None);
        assert_eq!(raw.color_scheme, None);
        assert_eq!(
            raw.gsettings["org.gnome.desktop.interface/accent-color"],
            SettingValue::Text("teal".into())
        );
    }
}
