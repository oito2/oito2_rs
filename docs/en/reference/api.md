🌐 [Português](../../pt-br/reference/api.md) | **English** | 🏠 [Index](../index.md)

---

# API

The full documentation with every signature is on docs.rs (`cargo doc --open` in a clone). This page lists the public items.

## `oito2_foundation`

| Item | Kind | Purpose |
|---|---|---|
| `OperatingSystem` | enum | `Android`, `Fuchsia`, `Ios`, `Linux`, `Macos`, `Windows`; `current`, `as_str`, `from_name`, `ALL` |
| `Platform` | struct | `{ os, is_web }`; `new`, `current`, `is_desktop_native`, `is_touch`, `is_web_touch`, `uses_touch_input`, `is_linux`, `is_windows`, `is_macos`, `is_android`, `is_ios`, `density` |
| `Density` | enum | `Pointer`, `Touch` |
| `DesktopEnvironment` | enum | 10 desktops and `Unknown`; `detect`, `as_str`, `from_name`, `ALL` |
| `WindowButton` | enum | `Minimize`, `Maximize`, `Close`; `as_str` |
| `WindowButtonLayout` | struct | `{ leading, trailing }`; `new`, `parse_gnome`, `parse_kde`, `parse_xfwm`, `default_for`, `is_empty` |
| `WindowControlStyle` | enum | `Gnome`, `Breeze`, `Xfwm`, `Cosmic`, `Windows`, `Macos`, `None`; `resolve`, `as_str`, `from_name`, `ALL` |
| `DecorationMode` | enum | `Native`, `Undecorated`, `GtkHeaderless`; `platform_default`, `as_str`, `FromStr`, `ALL` |
| `ParseDecorationModeError` | struct | `FromStr` error of `DecorationMode`, with the rejected `name` |
| `window_menu` | module | `supported`, `validate_position`, `MAX_COORDINATE`, `InvalidPositionError` |
| `AccentColor` | enum | The nine GNOME palette names; `hex`, `as_str`, `from_name`, `ALL`, `FALLBACK` |
| `ColorScheme` | enum | `Light`, `Dark`; `from_portal`, `from_gnome_preference`, `as_str`, `from_name`, `ALL` |
| `RawSettings` | struct | The values a reader collected; `gsetting_text`, `gsetting_flag` |
| `ConfigValues` | struct | The values of the desktop configuration files |
| `SettingValue` | enum | A GSettings value: `Text(String)` or `Flag(bool)` |
| `raw::schemas`, `raw::files`, `raw::GSETTINGS_KEYS` | consts | The schemas, keys and files the readers use |
| `SystemSettings` | struct | `{ platform, desktop_environment, window_control_style, window_button_layout, accent_color, color_scheme, high_contrast }`; `fallback`, `compose` |

## `oito2_settings`

| Item | Kind | Purpose |
|---|---|---|
| `read_raw_settings()` | fn | Reads the stores of the running platform into a `RawSettings` |
| `read_system_settings()` | fn | `read_raw_settings` composed for `Platform::current()` |
| `watch(on_change)` | fn (feature `watch`) | Starts a `SettingsWatcher`; `Error::Unsupported` outside Linux, Windows and macOS |
| `SettingsWatcher` | struct (feature `watch`) | Reports changes until dropped; `Send + Sync` |
| `Error`, `Result` | enum, alias | `Unsupported(String)`, `Internal(String)` |
| `foundation` | re-export | The whole `oito2_foundation` crate |
| `RawSettings`, `SystemSettings`, `Platform`, `ColorScheme`, `ConfigValues`, `SettingValue` | re-exports | From `oito2_foundation` |

## Features

| Crate | Feature | Default | Effect |
|---|---|---|---|
| `oito2_foundation` | `serde` | yes | `Serialize`/`Deserialize` for every public type |
| `oito2_settings` | `serde` | yes | Enables `oito2_foundation/serde` and serializes `Error` |
| `oito2_settings` | `watch` | no | `watch`, `SettingsWatcher` |
