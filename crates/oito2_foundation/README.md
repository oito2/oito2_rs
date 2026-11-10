# oito2_foundation

Pure platform rules shared by the oito2 user interfaces: desktop environment detection, window button layout, window control style, decoration mode and the composition of system settings. No I/O and no system dependency.

Part of [oito2_rs](https://github.com/oito2/oito2_rs). The rules mirror the [oito2_ui](https://github.com/oito2/oito2_ui) Flutter packages and are tested against their parity fixture.

## Usage

```toml
[dependencies]
oito2_foundation = "0.1"
```

```rust
use oito2_foundation::{
    DesktopEnvironment, OperatingSystem, Platform, RawSettings, SystemSettings, WindowButtonLayout,
    WindowControlStyle,
};

let desktop = DesktopEnvironment::detect(Some("ubuntu:GNOME"), None);
assert_eq!(desktop, DesktopEnvironment::Gnome);

let layout = WindowButtonLayout::parse_gnome("close,minimize:");
assert!(layout.trailing.is_empty());

// Settings from the platform alone, or composed from what a reader collected.
let linux = Platform::new(OperatingSystem::Linux, false);
let fallback = SystemSettings::fallback(linux, Some(desktop));
assert_eq!(fallback.window_control_style, WindowControlStyle::Gnome);
let raw = RawSettings {
    xdg_current_desktop: Some("KDE".into()),
    ..Default::default()
};
let settings = SystemSettings::compose(&raw, linux);
assert_eq!(settings.desktop_environment, Some(DesktopEnvironment::Kde));
```

`RawSettings` is filled by [`oito2_settings`](https://crates.io/crates/oito2_settings), or by any reader that produces the same values (for example the JSON a Tauri plugin sends).

## Features

| Feature | Default | Effect |
|---|---|---|
| `serde` | yes | `Serialize`/`Deserialize` for every public type, with the JSON names of the oito2 Tauri plugins |

## Design Note

In the Dart and TypeScript mirrors the composition lives in the settings package. Here `RawSettings` and `SystemSettings::compose` live in this pure crate, so applications and tests can compose settings without linking GIO.

## License

GPL-3.0-only — see [LICENSE](LICENSE).
