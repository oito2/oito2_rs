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

//! Parity with `oito2_ui`: every case of `fixtures/foundation.json`, the
//! fixture its parity exporter writes, gives the same result here.

use oito2_foundation::{
    DesktopEnvironment, OperatingSystem, Platform, SystemSettings, WindowButton,
    WindowButtonLayout, WindowControlStyle,
};
use serde_json::Value;

/// The fixture, parsed.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/foundation.json")).expect("valid fixture")
}

/// A string field of `case`, or `None` when it is null.
fn text<'a>(case: &'a Value, key: &str) -> Option<&'a str> {
    case[key].as_str()
}

/// A required string field of `case`.
fn required<'a>(case: &'a Value, key: &str) -> &'a str {
    text(case, key).expect("string field")
}

/// A required boolean field of `case`.
fn flag(case: &Value, key: &str) -> bool {
    case[key].as_bool().expect("boolean field")
}

/// The platform of a case with `os` and `isWeb`.
fn platform(case: &Value) -> Platform {
    let os = OperatingSystem::from_name(required(case, "os")).expect("known os");
    Platform::new(os, flag(case, "isWeb"))
}

/// The desktop environment of a case, `None` when null.
fn environment(case: &Value) -> Option<DesktopEnvironment> {
    text(case, "desktopEnvironment")
        .map(|name| DesktopEnvironment::from_name(name).expect("known environment"))
}

/// The buttons a JSON array names.
fn buttons(value: &Value) -> Vec<WindowButton> {
    value
        .as_array()
        .expect("button array")
        .iter()
        .map(|name| match name.as_str() {
            Some("minimize") => WindowButton::Minimize,
            Some("maximize") => WindowButton::Maximize,
            _ => {
                assert_eq!(name.as_str(), Some("close"), "unknown button");
                WindowButton::Close
            }
        })
        .collect()
}

/// The layout a JSON object with `leading` and `trailing` describes.
fn layout(value: &Value) -> WindowButtonLayout {
    WindowButtonLayout::new(buttons(&value["leading"]), buttons(&value["trailing"]))
}

#[test]
fn desktop_environments_match() {
    let fixture = fixture();
    let cases = fixture["desktopEnvironments"].as_array().expect("cases");
    assert_eq!(cases.len(), 24);
    for case in cases {
        let detected = DesktopEnvironment::detect(
            text(case, "xdgCurrentDesktop"),
            text(case, "desktopSession"),
        );
        assert_eq!(detected.as_str(), required(case, "result"), "{case}");
    }
}

#[test]
fn gnome_layouts_match() {
    let fixture = fixture();
    let cases = fixture["windowButtonLayouts"]["gnome"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 10);
    for case in cases {
        let parsed = WindowButtonLayout::parse_gnome(required(case, "value"));
        assert_eq!(parsed, layout(case), "{case}");
    }
}

#[test]
fn kde_layouts_match() {
    let fixture = fixture();
    let cases = fixture["windowButtonLayouts"]["kde"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 5);
    for case in cases {
        let parsed = WindowButtonLayout::parse_kde(
            required(case, "buttonsOnLeft"),
            required(case, "buttonsOnRight"),
        );
        assert_eq!(parsed, layout(case), "{case}");
    }
}

#[test]
fn xfwm_layouts_match() {
    let fixture = fixture();
    let cases = fixture["windowButtonLayouts"]["xfwm"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 7);
    for case in cases {
        let parsed = WindowButtonLayout::parse_xfwm(required(case, "value"));
        assert_eq!(parsed, layout(case), "{case}");
    }
}

#[test]
fn default_layouts_match() {
    let fixture = fixture();
    let cases = fixture["windowButtonLayouts"]["defaultFor"]
        .as_object()
        .expect("cases");
    assert_eq!(cases.len(), 7);
    for (name, expected) in cases {
        let style = WindowControlStyle::from_name(name).expect("known style");
        assert_eq!(
            WindowButtonLayout::default_for(style),
            layout(expected),
            "{name}"
        );
    }
}

#[test]
fn control_styles_match() {
    let fixture = fixture();
    let cases = fixture["windowButtonLayouts"]["resolve"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 144);
    for case in cases {
        let style = WindowControlStyle::resolve(platform(case), environment(case));
        assert_eq!(style.as_str(), required(case, "result"), "{case}");
    }
}

#[test]
fn platform_flags_match() {
    let fixture = fixture();
    let cases = fixture["platforms"].as_array().expect("cases");
    assert_eq!(cases.len(), 12);
    for case in cases {
        let platform = platform(case);
        let flags = [
            ("isDesktopNative", platform.is_desktop_native()),
            ("isTouch", platform.is_touch()),
            ("isWebTouch", platform.is_web_touch()),
            ("usesTouchInput", platform.uses_touch_input()),
            ("isLinux", platform.is_linux()),
            ("isWindows", platform.is_windows()),
            ("isMacOS", platform.is_macos()),
            ("isAndroid", platform.is_android()),
            ("isIOS", platform.is_ios()),
        ];
        for (key, actual) in flags {
            assert_eq!(actual, flag(case, key), "{key} of {case}");
        }
    }
}

#[test]
fn fallback_settings_match() {
    let fixture = fixture();
    let cases = fixture["systemSettings"].as_array().expect("cases");
    assert_eq!(cases.len(), 24);
    for case in cases {
        let settings = SystemSettings::fallback(platform(case), environment(case));
        let result = &case["result"];
        assert_eq!(
            settings.desktop_environment.map(DesktopEnvironment::as_str),
            text(result, "desktopEnvironment"),
            "{case}"
        );
        assert_eq!(
            settings.window_control_style.as_str(),
            required(result, "windowControlStyle"),
            "{case}"
        );
        assert_eq!(
            settings.window_button_layout,
            layout(&result["windowButtonLayout"]),
            "{case}"
        );
        assert_eq!(
            settings.accent_color.as_deref(),
            text(result, "accentColor"),
            "{case}"
        );
        assert_eq!(
            settings.color_scheme.map(|scheme| scheme.as_str()),
            text(result, "colorScheme"),
            "{case}"
        );
        assert_eq!(
            settings.high_contrast,
            flag(result, "highContrast"),
            "{case}"
        );
    }
}

#[cfg(feature = "serde")]
#[test]
fn fallback_settings_serialize_as_the_fixture() {
    let fixture = fixture();
    for case in fixture["systemSettings"].as_array().expect("cases") {
        let settings = SystemSettings::fallback(platform(case), environment(case));
        let mut json = serde_json::to_value(&settings).expect("serializable");
        json.as_object_mut().expect("object").remove("platform");
        assert_eq!(json, case["result"], "{case}");
    }
}
