# Changelog

All notable changes to `oito2_settings` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-17

### Changed

- The README on crates.io shows the oito2_rs header, and the documentation on docs.rs its logo and favicon.

### Fixed

- Linux `watch`: configuration directories that don't exist when the watcher starts (for example `~/.config/cosmic/com.system76.CosmicTk/v1` on a fresh COSMIC session, written only when a setting first leaves its default) are now waited for on their nearest existing ancestor, so the first change made there is reported instead of being missed until the application read the settings again.
- Linux `watch`: a configuration directory that is deleted is reported as a change and watched again when it is created again, and a directory that appears while the monitors are being created is no longer missed.

## [0.1.0] - 2026-11-10

### Added

- `README.md` with usage and features, the GPL-3.0 `LICENSE` in the package, and docs.rs metadata (all features; Linux, Windows and macOS targets).
- `read_raw_settings` and `read_system_settings`, moved from `tauri-plugin-oito2-settings`, plus the re-exports of `oito2_foundation` (`foundation`, `RawSettings`, `SystemSettings`, `Platform`, `ColorScheme`).
- Linux: GSettings through GIO and the `org.freedesktop.appearance` portal keys (`accent-color`, `contrast` and, new, `color-scheme`); `kdeglobals` `[KDE] LookAndFeelPackage` and the COSMIC `is_dark` flag in the configuration values.
- Windows: the color scheme from the brightness of the system background color, next to the accent and high-contrast readers.
- macOS: the color scheme from the `AppleInterfaceStyle` user default, next to the accent and "Increase contrast" readers.
- `Error` (`Unsupported`, `Internal`) with the `{kind, message}` JSON shape of the plugins.
- `examples/print_settings.rs`, which with the `watch` feature also prints every change.
- `watch` and `SettingsWatcher` (feature `watch`), on Linux, Windows and macOS. Linux: GSettings signals, file monitors on the configuration directories and the portal's `SettingChanged` signal on a GLib main context of the watcher's own thread, so no application main loop is needed; one callback per burst of changes (150 ms debounce), only when the settings differ; stops on drop. Windows: the `UISettings.ColorValuesChanged` and `AccessibilitySettings.HighContrastChanged` events. macOS: the `NSSystemColorsDidChangeNotification`, `AppleInterfaceThemeChangedNotification` (distributed) and `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification` notifications, delivered through the application's main run loop. Other platforms return `Error::Unsupported`.

[Unreleased]: https://github.com/oito2/oito2_rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/oito2/oito2_rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/oito2/oito2_rs/releases/tag/v0.1.0
