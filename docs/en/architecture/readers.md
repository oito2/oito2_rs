🌐 [Português](../../pt-br/architecture/readers.md) | **English** | 🏠 [Index](../index.md)

---

# Readers

`read_raw_settings()` dispatches to the reader of the running platform (`crates/oito2_settings/src/{linux,windows,macos}/`). Every reader returns a `RawSettings` and never fails.

## Linux

`linux::read` fills, in order:

1. **Session variables** — `XDG_CURRENT_DESKTOP`, `DESKTOP_SESSION`.
2. **GSettings** (`linux/gsettings.rs`) — each `(schema, key)` of `oito2_foundation::raw::GSETTINGS_KEYS` whose schema is installed and has the key. GIO aborts the process on an unknown schema or key, so `SettingsSchemaSource::lookup` and `has_key` run first. String and boolean values are kept, by `schema/key`.
3. **Configuration files** (`config.rs`) — under `XDG_CONFIG_HOME` when it is an absolute path, else `$HOME/.config`. Each file of `raw::files` is read when it exists, is a regular file and is at most 1 MiB, and parsed by `parsers.rs` (INI groups for `kwinrc`/`kdeglobals`, the xfconf XML property for `xfwm4.xml`, `true`/`false` for COSMIC).
4. **Settings portal** (`linux/portal.rs`) — on the session bus, `org.freedesktop.portal.Settings.ReadOne` (falling back to the older `Read`) for `org.freedesktop.appearance` `accent-color` (`(ddd)`, components 0–1), `contrast` (`u`, 1 is high) and `color-scheme` (`u`: 1 dark, 2 light, 0 no preference). Each call times out after 500 ms; without a bus or a portal nothing changes.

## Windows

`windows::read` (`windows/mod.rs`):

- `UISettings.GetColorValue(Accent)` → `system_accent`.
- `UISettings.GetColorValue(Background)` → `color_scheme`: `Light` when `(5·G + 2·R + B) / 8 > 128`, else `Dark`.
- `AccessibilitySettings.HighContrast` → `system_high_contrast`.

## macOS

`macos::read` (`macos/mod.rs`):

- `NSColor.controlAccentColor` converted to sRGB → `system_accent`.
- `NSWorkspace.accessibilityDisplayShouldIncreaseContrast` → `system_high_contrast`.
- `NSUserDefaults.standardUserDefaults` `AppleInterfaceStyle` → `color_scheme`: `Dark` when it is `Dark`, else `Light`. The user default is read instead of `NSApp.effectiveAppearance`, which needs a running `NSApplication`.

## Other Platforms

`read_raw_settings` returns `RawSettings::default()`.
