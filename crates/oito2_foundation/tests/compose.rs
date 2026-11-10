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

//! The composition rules, with the cases of `compose.test.ts` in
//! `oito2_uikit` and of `settings_reader_test.dart` in `oito2_ui_settings`.

use oito2_foundation::raw::schemas::{GNOME_A11Y, GNOME_INTERFACE, GNOME_WM};
use oito2_foundation::{
    AccentColor, ColorScheme, DesktopEnvironment, OperatingSystem, Platform, RawSettings,
    SettingValue, SystemSettings, WindowButton, WindowButtonLayout, WindowControlStyle,
};

const LINUX: Platform = Platform::new(OperatingSystem::Linux, false);
const WINDOWS: Platform = Platform::new(OperatingSystem::Windows, false);
const MIN: WindowButton = WindowButton::Minimize;
const MAX: WindowButton = WindowButton::Maximize;
const CLOSE: WindowButton = WindowButton::Close;

/// Raw settings for a session on `desktop`, with nothing else read.
fn raw(desktop: &str) -> RawSettings {
    RawSettings {
        xdg_current_desktop: Some(desktop.to_owned()),
        ..Default::default()
    }
}

/// Adds the string `value` of `key` in `schema`.
fn text(mut raw: RawSettings, schema: &str, key: &str, value: &str) -> RawSettings {
    raw.gsettings.insert(
        format!("{schema}/{key}"),
        SettingValue::Text(value.to_owned()),
    );
    raw
}

/// Adds the boolean `value` of `key` in `schema`.
fn flag(mut raw: RawSettings, schema: &str, key: &str, value: bool) -> RawSettings {
    raw.gsettings
        .insert(format!("{schema}/{key}"), SettingValue::Flag(value));
    raw
}

/// Sets the system accent.
fn accent(mut raw: RawSettings, color: &str) -> RawSettings {
    raw.system_accent = Some(color.to_owned());
    raw
}

fn compose(raw: &RawSettings) -> SystemSettings {
    SystemSettings::compose(raw, LINUX)
}

mod gnome_family {
    use super::*;

    #[test]
    fn reads_the_button_layout_from_gsettings() {
        let settings = compose(&text(raw("GNOME"), GNOME_WM, "button-layout", "close:"));
        assert_eq!(
            settings.desktop_environment,
            Some(DesktopEnvironment::Gnome)
        );
        assert_eq!(settings.window_control_style, WindowControlStyle::Gnome);
        assert_eq!(
            settings.window_button_layout,
            WindowButtonLayout::new(vec![CLOSE], vec![])
        );
    }

    #[test]
    fn falls_back_to_the_default_layout_without_gsettings() {
        let settings = compose(&raw("GNOME"));
        assert_eq!(
            settings.window_button_layout,
            WindowButtonLayout::default_for(WindowControlStyle::Gnome)
        );
    }

    #[test]
    fn budgie_pantheon_and_unity_read_the_gnome_schema() {
        for desktop in ["Budgie:GNOME", "Pantheon", "Unity"] {
            let settings = compose(&text(raw(desktop), GNOME_WM, "button-layout", "close:"));
            assert_eq!(settings.window_button_layout.leading, [CLOSE], "{desktop}");
        }
    }

    #[test]
    fn cinnamon_and_mate_read_their_own_schemas() {
        let cinnamon = compose(&text(
            raw("X-Cinnamon"),
            "org.cinnamon.desktop.wm.preferences",
            "button-layout",
            ":minimize,close",
        ));
        assert_eq!(cinnamon.window_button_layout.trailing, [MIN, CLOSE]);
        let mate = compose(&text(
            raw("MATE"),
            "org.mate.Marco.general",
            "button-layout",
            "close,maximize:",
        ));
        assert_eq!(mate.window_button_layout.leading, [CLOSE, MAX]);
    }

    #[test]
    fn maps_the_gnome_47_accent_color_to_the_palette() {
        let settings = compose(&accent(
            text(raw("GNOME"), GNOME_INTERFACE, "accent-color", "purple"),
            "#123456",
        ));
        assert_eq!(
            settings.accent_color.as_deref(),
            Some(AccentColor::Purple.hex())
        );
    }

    #[test]
    fn ignores_an_accent_color_name_outside_the_palette() {
        let settings = compose(&accent(
            text(raw("GNOME"), GNOME_INTERFACE, "accent-color", "toString"),
            "#123456",
        ));
        assert_eq!(settings.accent_color.as_deref(), Some("#123456"));
    }

    #[test]
    fn uses_the_toolkit_accent_without_accent_color() {
        let settings = compose(&accent(raw("GNOME"), "#123456"));
        assert_eq!(settings.accent_color.as_deref(), Some("#123456"));
    }

    #[test]
    fn reads_high_contrast_from_the_a11y_key_the_gtk_theme_or_the_system() {
        let contrast = |raw: RawSettings| compose(&raw).high_contrast;
        assert!(contrast(flag(
            raw("GNOME"),
            GNOME_A11Y,
            "high-contrast",
            true
        )));
        assert!(contrast(text(
            raw("GNOME"),
            GNOME_INTERFACE,
            "gtk-theme",
            "HighContrast"
        )));
        assert!(!contrast(text(
            raw("GNOME"),
            GNOME_INTERFACE,
            "gtk-theme",
            "Adwaita"
        )));
        let mut system = raw("GNOME");
        system.system_high_contrast = true;
        assert!(contrast(system));
    }
}

mod kde_plasma {
    use super::*;

    #[test]
    fn reads_the_button_layout_from_kwinrc() {
        let mut raw = raw("KDE");
        raw.config.kwin_buttons_on_left = Some("X".into());
        raw.config.kwin_buttons_on_right = Some("IA".into());
        let settings = compose(&raw);
        assert_eq!(settings.window_control_style, WindowControlStyle::Breeze);
        assert_eq!(
            settings.window_button_layout,
            WindowButtonLayout::new(vec![CLOSE], vec![MIN, MAX])
        );
    }

    #[test]
    fn uses_the_breeze_defaults_without_kwinrc() {
        let settings = compose(&raw("KDE"));
        assert_eq!(settings.window_button_layout.trailing, [MIN, MAX, CLOSE]);
    }

    #[test]
    fn reads_the_accent_and_contrast_from_kdeglobals() {
        let mut raw = accent(raw("KDE"), "#123456");
        raw.config.kde_accent_color = Some("#3DAEE9".into());
        raw.config.kde_color_scheme = Some("BreezeHighContrast".into());
        let settings = compose(&raw);
        assert_eq!(settings.accent_color.as_deref(), Some("#3DAEE9"));
        assert!(settings.high_contrast);
    }

    #[test]
    fn falls_back_to_the_toolkit_accent_without_accent_color() {
        let mut raw = accent(raw("KDE"), "#123456");
        raw.config.kde_color_scheme = Some("Breeze".into());
        let settings = compose(&raw);
        assert_eq!(settings.accent_color.as_deref(), Some("#123456"));
        assert!(!settings.high_contrast);
    }
}

mod xfce_and_cosmic {
    use super::*;

    #[test]
    fn reads_the_xfwm_layout_or_its_default() {
        let mut raw = raw("XFCE");
        raw.config.xfwm_button_layout = Some("CMH|".into());
        let set = compose(&raw);
        assert_eq!(set.window_control_style, WindowControlStyle::Xfwm);
        assert_eq!(set.window_button_layout.leading, [CLOSE, MAX, MIN]);
        assert!(set.window_button_layout.trailing.is_empty());
        let unset = compose(&super::raw("XFCE"));
        assert_eq!(unset.window_button_layout.trailing, [MIN, MAX, CLOSE]);
    }

    #[test]
    fn hides_the_cosmic_buttons_turned_off_and_shows_every_one_by_default() {
        let mut raw = raw("COSMIC");
        raw.config.cosmic_show_minimize = Some(false);
        raw.config.cosmic_show_maximize = Some(true);
        let set = compose(&raw);
        assert_eq!(set.window_control_style, WindowControlStyle::Cosmic);
        assert_eq!(set.window_button_layout.trailing, [MAX, CLOSE]);
        let unset = compose(&super::raw("COSMIC"));
        assert_eq!(unset.window_button_layout.trailing, [MIN, MAX, CLOSE]);
    }
}

mod other_platforms {
    use super::*;

    #[test]
    fn use_the_platform_defaults_and_the_system_accent() {
        let raw = accent(
            text(raw("GNOME"), GNOME_WM, "button-layout", "close:"),
            "#0078D4",
        );
        let settings = SystemSettings::compose(&raw, WINDOWS);
        assert_eq!(settings.desktop_environment, None);
        assert_eq!(settings.window_control_style, WindowControlStyle::Windows);
        assert_eq!(
            settings.window_button_layout,
            WindowButtonLayout::default_for(WindowControlStyle::Windows)
        );
        assert_eq!(settings.accent_color.as_deref(), Some("#0078D4"));
    }

    #[test]
    fn honor_the_system_high_contrast_preference() {
        let mut raw = raw("");
        raw.system_high_contrast = true;
        assert!(SystemSettings::compose(&raw, WINDOWS).high_contrast);
    }

    #[test]
    fn give_the_web_no_desktop_environment() {
        let web = Platform::new(OperatingSystem::Linux, true);
        let settings = SystemSettings::compose(&raw("GNOME"), web);
        assert_eq!(settings.desktop_environment, None);
        assert_eq!(settings.window_control_style, WindowControlStyle::None);
    }

    #[test]
    fn report_the_system_color_scheme() {
        let mut raw = raw("");
        raw.color_scheme = Some(ColorScheme::Dark);
        assert_eq!(
            SystemSettings::compose(&raw, WINDOWS).color_scheme,
            Some(ColorScheme::Dark)
        );
        assert_eq!(
            SystemSettings::compose(&super::raw(""), WINDOWS).color_scheme,
            None
        );
    }
}

mod color_scheme {
    use super::*;

    #[test]
    fn the_desktop_portal_wins_over_the_desktop_store() {
        let mut raw = text(
            raw("GNOME"),
            GNOME_INTERFACE,
            "color-scheme",
            "prefer-light",
        );
        raw.color_scheme = Some(ColorScheme::Dark);
        assert_eq!(compose(&raw).color_scheme, Some(ColorScheme::Dark));
    }

    #[test]
    fn the_gnome_family_falls_back_to_color_scheme() {
        let cases = [
            (Some("prefer-dark"), Some(ColorScheme::Dark)),
            (Some("prefer-light"), Some(ColorScheme::Light)),
            (Some("default"), None),
            (None, None),
        ];
        for (value, expected) in cases {
            for desktop in ["GNOME", "Budgie:GNOME", "X-Cinnamon", "MATE"] {
                let raw = match value {
                    Some(value) => text(raw(desktop), GNOME_INTERFACE, "color-scheme", value),
                    None => raw(desktop),
                };
                assert_eq!(compose(&raw).color_scheme, expected, "{desktop} {value:?}");
            }
        }
    }

    #[test]
    fn kde_falls_back_to_a_dark_color_scheme_or_look_and_feel() {
        let read = |scheme: Option<&str>, package: Option<&str>| {
            let mut raw = raw("KDE");
            raw.config.kde_color_scheme = scheme.map(str::to_owned);
            raw.config.kde_look_and_feel_package = package.map(str::to_owned);
            compose(&raw).color_scheme
        };
        assert_eq!(read(Some("BreezeDark"), None), Some(ColorScheme::Dark));
        assert_eq!(read(Some("BreezeLight"), None), None);
        assert_eq!(
            read(None, Some("org.kde.breezedark.desktop")),
            Some(ColorScheme::Dark)
        );
        assert_eq!(
            read(Some("BreezeLight"), Some("org.kde.breezedark.desktop")),
            None
        );
        assert_eq!(read(None, None), None);
    }

    #[test]
    fn cosmic_falls_back_to_its_theme_mode() {
        let read = |dark: Option<bool>| {
            let mut raw = raw("COSMIC");
            raw.config.cosmic_is_dark = dark;
            compose(&raw).color_scheme
        };
        assert_eq!(read(Some(true)), Some(ColorScheme::Dark));
        assert_eq!(read(Some(false)), Some(ColorScheme::Light));
        assert_eq!(read(None), None);
    }

    #[test]
    fn xfce_and_lxqt_rely_on_the_portal_alone() {
        for desktop in ["XFCE", "LXQt"] {
            let raw = text(raw(desktop), GNOME_INTERFACE, "color-scheme", "prefer-dark");
            assert_eq!(compose(&raw).color_scheme, None, "{desktop}");
            let mut portal = super::raw(desktop);
            portal.color_scheme = Some(ColorScheme::Light);
            assert_eq!(
                compose(&portal).color_scheme,
                Some(ColorScheme::Light),
                "{desktop}"
            );
        }
    }
}

#[test]
fn composes_the_same_settings_for_the_same_input() {
    let a = compose(&raw("GNOME"));
    assert_eq!(a, compose(&raw("GNOME")));
    assert_ne!(a, compose(&accent(raw("GNOME"), "#111111")));
}
