# Changelog

All notable changes to `oito2_foundation` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `README.md` with usage and features, the GPL-3.0 `LICENSE` in the package, and docs.rs metadata (all features; Linux, Windows and macOS targets).
- `OperatingSystem`, `Density` and `Platform` with the ergonomics flags of `oito2_ui`.
- `DesktopEnvironment` with `detect` from `XDG_CURRENT_DESKTOP` and `DESKTOP_SESSION`.
- `WindowButton`, `WindowButtonLayout` (GNOME, KDE and Xfwm parsers, per-style defaults) and `WindowControlStyle::resolve`.
- `DecorationMode` with `platform_default`, `FromStr` and `ParseDecorationModeError`, moved from `tauri-plugin-oito2-window`.
- `window_menu::{supported, validate_position}`, moved from `tauri-plugin-oito2-window`.
- `AccentColor` with the nine GNOME palette names and their hex values.
- `ColorScheme` with the portal and GNOME preference mappings.
- `raw::{RawSettings, ConfigValues, SettingValue}`, moved from `tauri-plugin-oito2-settings`, with the new optional fields `colorScheme`, `config.kdeLookAndFeelPackage` and `config.cosmicIsDark`, the `schemas` and `files` constants and `GSETTINGS_KEYS` (now with `org.gnome.desktop.interface color-scheme`).
- `SystemSettings` with `fallback` and `compose`, the rules of `oito2_ui_settings` including the color scheme fallback per desktop.
- Parity test against `tests/fixtures/foundation.json` and the composition cases of `oito2_uikit` and `oito2_ui_settings`.
