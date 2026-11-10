🌐 [Português](../../pt-br/reference/settings-sources.md) | **English** | 🏠 [Index](../index.md)

---

# Settings Sources

## Linux — Session

| Variable | `RawSettings` field |
|---|---|
| `XDG_CURRENT_DESKTOP` | `xdg_current_desktop` |
| `DESKTOP_SESSION` | `desktop_session` |

## Linux — GSettings

Stored in `gsettings` by `schema/key`, when the schema is installed.

| Schema | Key | Used for |
|---|---|---|
| `org.gnome.desktop.wm.preferences` | `button-layout` | Button layout of GNOME, Budgie, Pantheon, Unity, LXQt, unknown |
| `org.cinnamon.desktop.wm.preferences` | `button-layout` | Button layout of Cinnamon |
| `org.mate.Marco.general` | `button-layout` | Button layout of MATE |
| `org.gnome.desktop.interface` | `accent-color` | Named accent |
| `org.gnome.desktop.interface` | `gtk-theme` | High contrast ("highcontrast" in the name) |
| `org.gnome.desktop.interface` | `color-scheme` | Color scheme fallback |
| `org.gnome.desktop.a11y.interface` | `high-contrast` | High contrast |

## Linux — Configuration Files

Relative to `XDG_CONFIG_HOME` (or `$HOME/.config`); files larger than 1 MiB are ignored.

| File | Entry | `ConfigValues` field |
|---|---|---|
| `kwinrc` | `[org.kde.kdecoration2] ButtonsOnLeft` | `kwin_buttons_on_left` |
| `kwinrc` | `[org.kde.kdecoration2] ButtonsOnRight` | `kwin_buttons_on_right` |
| `kdeglobals` | `[General] AccentColor` (`r,g,b`) | `kde_accent_color` (`#RRGGBB`) |
| `kdeglobals` | `[General] ColorScheme` | `kde_color_scheme` |
| `kdeglobals` | `[KDE] LookAndFeelPackage` | `kde_look_and_feel_package` |
| `xfce4/xfconf/xfce-perchannel-xml/xfwm4.xml` | property `button_layout` | `xfwm_button_layout` |
| `cosmic/com.system76.CosmicTk/v1/show_minimize` | `true`/`false` | `cosmic_show_minimize` |
| `cosmic/com.system76.CosmicTk/v1/show_maximize` | `true`/`false` | `cosmic_show_maximize` |
| `cosmic/com.system76.CosmicTheme.Mode/v1/is_dark` | `true`/`false` | `cosmic_is_dark` |

## Linux — Settings Portal

`org.freedesktop.portal.Desktop`, `/org/freedesktop/portal/desktop`, interface `org.freedesktop.portal.Settings`, namespace `org.freedesktop.appearance`.

| Key | Type | `RawSettings` field |
|---|---|---|
| `accent-color` | `(ddd)`, components 0–1 (out of range when unset) | `system_accent` |
| `contrast` | `u`: 1 high, 0 normal | `system_high_contrast` |
| `color-scheme` | `u`: 1 dark, 2 light, 0 no preference | `color_scheme` |

## Windows

| API | `RawSettings` field |
|---|---|
| `UISettings.GetColorValue(UIColorType::Accent)` | `system_accent` |
| `UISettings.GetColorValue(UIColorType::Background)` | `color_scheme` |
| `AccessibilitySettings.HighContrast` | `system_high_contrast` |

## macOS

| API | `RawSettings` field |
|---|---|
| `NSColor.controlAccentColor` (sRGB) | `system_accent` |
| `NSWorkspace.accessibilityDisplayShouldIncreaseContrast` | `system_high_contrast` |
| `NSUserDefaults` `AppleInterfaceStyle` | `color_scheme` |
