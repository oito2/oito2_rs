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

//! The operating system preferences that shape an app, derived from the
//! platform or composed from what a reader collected.

use crate::raw::schemas;
use crate::{
    AccentColor, ColorScheme, DesktopEnvironment, Platform, RawSettings, WindowButton,
    WindowButtonLayout, WindowControlStyle,
};

/// The operating system preferences that shape an OITO2 app: platform,
/// desktop environment, window controls, accent color, color scheme and
/// contrast.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SystemSettings {
    /// The platform the app runs on.
    pub platform: Platform,
    /// The Linux desktop environment, or `None` outside native Linux.
    pub desktop_environment: Option<DesktopEnvironment>,
    /// The visual family of the window control buttons.
    pub window_control_style: WindowControlStyle,
    /// The order and side of the window control buttons.
    pub window_button_layout: WindowButtonLayout,
    /// The system accent color as `#RRGGBB`, or `None` when the system has
    /// none or it is unknown.
    pub accent_color: Option<String>,
    /// The light or dark scheme the system asks for, or `None` when it states
    /// no preference or the preference is unknown.
    pub color_scheme: Option<ColorScheme>,
    /// Whether the system asks for high contrast.
    pub high_contrast: bool,
}

impl SystemSettings {
    /// Settings derived from `platform` only.
    ///
    /// On native Linux the desktop environment is `desktop_environment`, or
    /// [`DesktopEnvironment::Unknown`] when `None`; elsewhere it is `None`.
    /// The control style and button layout follow
    /// [`WindowControlStyle::resolve`] and [`WindowButtonLayout::default_for`];
    /// there is no accent color, no color scheme preference, and high
    /// contrast is off.
    pub fn fallback(platform: Platform, desktop_environment: Option<DesktopEnvironment>) -> Self {
        let environment = if platform.is_linux() && platform.is_desktop_native() {
            Some(desktop_environment.unwrap_or(DesktopEnvironment::Unknown))
        } else {
            None
        };
        let style = WindowControlStyle::resolve(platform, environment);
        Self {
            platform,
            desktop_environment: environment,
            window_control_style: style,
            window_button_layout: WindowButtonLayout::default_for(style),
            accent_color: None,
            color_scheme: None,
            high_contrast: false,
        }
    }

    /// Turns the values a reader collected into the settings of `platform`.
    ///
    /// On native Linux the desktop environment comes from the session
    /// variables and decides which store each value comes from:
    ///
    /// - Button layout: `button-layout` of the GNOME, Cinnamon or MATE window
    ///   manager schema; `ButtonsOnLeft`/`ButtonsOnRight` of `kwinrc` on KDE
    ///   (`MS`/`HIAX` when missing); `button_layout` of `xfwm4.xml` on Xfce
    ///   (`O|SHMC` when missing); the `show_minimize`/`show_maximize` flags on
    ///   COSMIC (a missing flag shows the button). A missing GNOME-style key
    ///   uses the style's default layout.
    /// - Accent: `AccentColor` of `kdeglobals` on KDE; the named
    ///   `accent-color` of the GNOME palette elsewhere; then the system accent.
    /// - High contrast: the system preference, or a `kdeglobals` color scheme
    ///   containing "contrast" on KDE, or the `high-contrast` key or a GTK
    ///   theme containing "highcontrast" elsewhere.
    /// - Color scheme: the portal value in `raw.color_scheme`; when it states
    ///   no preference, `color-scheme` of the GNOME interface schema on the
    ///   GNOME family, Cinnamon and MATE, a `kdeglobals` color scheme or
    ///   look-and-feel package containing "dark" on KDE, the `is_dark` flag on
    ///   COSMIC; Xfce and LXQt have no store beyond the portal.
    ///
    /// Outside native Linux only the system accent, high contrast and color
    /// scheme count; everything else follows [`SystemSettings::fallback`].
    pub fn compose(raw: &RawSettings, platform: Platform) -> Self {
        let environment = if platform.is_linux() && platform.is_desktop_native() {
            Some(DesktopEnvironment::detect(
                raw.xdg_current_desktop.as_deref(),
                raw.desktop_session.as_deref(),
            ))
        } else {
            None
        };
        let fallback = Self::fallback(platform, environment);
        let layout = environment.and_then(|environment| button_layout(raw, environment));
        Self {
            platform,
            desktop_environment: environment,
            window_control_style: fallback.window_control_style,
            window_button_layout: layout.unwrap_or(fallback.window_button_layout),
            accent_color: match environment {
                Some(environment) => linux_accent(raw, environment),
                None => raw.system_accent.clone(),
            },
            color_scheme: match environment {
                Some(environment) => raw
                    .color_scheme
                    .or_else(|| linux_color_scheme(raw, environment)),
                None => raw.color_scheme,
            },
            high_contrast: raw.system_high_contrast
                || environment.is_some_and(|environment| linux_high_contrast(raw, environment)),
        }
    }
}

/// The window button layout of `environment`, or `None` to use the default.
fn button_layout(raw: &RawSettings, environment: DesktopEnvironment) -> Option<WindowButtonLayout> {
    let gnome_layout = |schema: &str| {
        raw.gsetting_text(schema, "button-layout")
            .map(WindowButtonLayout::parse_gnome)
    };
    match environment {
        DesktopEnvironment::Kde => Some(WindowButtonLayout::parse_kde(
            raw.config.kwin_buttons_on_left.as_deref().unwrap_or("MS"),
            raw.config
                .kwin_buttons_on_right
                .as_deref()
                .unwrap_or("HIAX"),
        )),
        DesktopEnvironment::Xfce => Some(WindowButtonLayout::parse_xfwm(
            raw.config.xfwm_button_layout.as_deref().unwrap_or("O|SHMC"),
        )),
        DesktopEnvironment::Cosmic => {
            let mut trailing = Vec::with_capacity(3);
            if raw.config.cosmic_show_minimize.unwrap_or(true) {
                trailing.push(WindowButton::Minimize);
            }
            if raw.config.cosmic_show_maximize.unwrap_or(true) {
                trailing.push(WindowButton::Maximize);
            }
            trailing.push(WindowButton::Close);
            Some(WindowButtonLayout::new(Vec::new(), trailing))
        }
        DesktopEnvironment::Cinnamon => gnome_layout(schemas::CINNAMON_WM),
        DesktopEnvironment::Mate => gnome_layout(schemas::MATE_WM),
        _ => gnome_layout(schemas::GNOME_WM),
    }
}

/// The Linux accent: KDE's, or GNOME's named accent, else the system's.
fn linux_accent(raw: &RawSettings, environment: DesktopEnvironment) -> Option<String> {
    if environment == DesktopEnvironment::Kde {
        if let Some(color) = raw
            .config
            .kde_accent_color
            .as_deref()
            .filter(|c| !c.is_empty())
        {
            return Some(color.to_owned());
        }
    } else if let Some(accent) = raw
        .gsetting_text(schemas::GNOME_INTERFACE, "accent-color")
        .and_then(AccentColor::from_name)
    {
        return Some(accent.hex().to_owned());
    }
    raw.system_accent.clone()
}

/// Whether the Linux desktop asks for high contrast.
fn linux_high_contrast(raw: &RawSettings, environment: DesktopEnvironment) -> bool {
    if environment == DesktopEnvironment::Kde {
        return raw
            .config
            .kde_color_scheme
            .as_deref()
            .is_some_and(|scheme| scheme.to_lowercase().contains("contrast"));
    }
    if raw.gsetting_flag(schemas::GNOME_A11Y, "high-contrast") == Some(true) {
        return true;
    }
    raw.gsetting_text(schemas::GNOME_INTERFACE, "gtk-theme")
        .is_some_and(|theme| theme.to_lowercase().contains("highcontrast"))
}

/// The scheme the Linux desktop's own store asks for, when the portal states
/// no preference.
fn linux_color_scheme(raw: &RawSettings, environment: DesktopEnvironment) -> Option<ColorScheme> {
    match environment {
        DesktopEnvironment::Kde => raw
            .config
            .kde_color_scheme
            .as_deref()
            .or(raw.config.kde_look_and_feel_package.as_deref())
            .filter(|name| name.to_lowercase().contains("dark"))
            .map(|_| ColorScheme::Dark),
        DesktopEnvironment::Cosmic => raw.config.cosmic_is_dark.map(|dark| {
            if dark {
                ColorScheme::Dark
            } else {
                ColorScheme::Light
            }
        }),
        DesktopEnvironment::Xfce | DesktopEnvironment::Lxqt => None,
        _ => raw
            .gsetting_text(schemas::GNOME_INTERFACE, "color-scheme")
            .and_then(ColorScheme::from_gnome_preference),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OperatingSystem;

    #[cfg(feature = "serde")]
    #[test]
    fn serializes_in_camel_case() {
        let settings = SystemSettings::fallback(
            Platform::new(OperatingSystem::Linux, false),
            Some(DesktopEnvironment::Kde),
        );
        let json = serde_json::to_value(&settings).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "platform": { "os": "linux", "isWeb": false },
                "desktopEnvironment": "kde",
                "windowControlStyle": "breeze",
                "windowButtonLayout": { "leading": [], "trailing": ["minimize", "maximize", "close"] },
                "accentColor": null,
                "colorScheme": null,
                "highContrast": false
            })
        );
        let back: SystemSettings = serde_json::from_value(json).unwrap();
        assert_eq!(back, settings);
    }

    #[test]
    fn fallback_outside_linux_has_no_environment() {
        let settings = SystemSettings::fallback(
            Platform::new(OperatingSystem::Linux, true),
            Some(DesktopEnvironment::Kde),
        );
        assert_eq!(settings.desktop_environment, None);
        assert_eq!(settings.window_control_style, WindowControlStyle::None);
        assert!(settings.window_button_layout.is_empty());
    }
}
