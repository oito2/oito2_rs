# oito2_settings

Reads and watches the desktop settings the oito2 user interfaces follow: window button layout, accent color, high contrast and light or dark scheme, on Linux, Windows and macOS.

Part of [oito2_rs](https://github.com/oito2/oito2_rs). The values are composed with the rules of [`oito2_foundation`](https://crates.io/crates/oito2_foundation), re-exported as `oito2_settings::foundation`.

## Usage

```toml
[dependencies]
oito2_settings = { version = "0.1", features = ["watch"] }
```

```rust,no_run
use oito2_settings::{Platform, SystemSettings};

fn main() -> Result<(), oito2_settings::Error> {
    // One read: the composed settings of the running platform.
    let settings = oito2_settings::read_system_settings();
    println!("{:?} {:?}", settings.desktop_environment, settings.color_scheme);

    // Every change, from a background thread, until the watcher is dropped.
    let _watcher = oito2_settings::watch(|raw| {
        let settings = SystemSettings::compose(raw, Platform::current());
        println!("changed: {settings:?}");
    })?;
    std::thread::park();
    Ok(())
}
```

## Platforms

| OS | Read from | Watched through |
|---|---|---|
| Linux | GSettings, `kwinrc`/`kdeglobals`, `xfwm4.xml`, COSMIC settings files, the `org.freedesktop.appearance` portal keys | GSettings signals, file monitors and the portal's `SettingChanged`, on a GLib main context of the watcher's own thread |
| Windows | `UISettings` (accent, background color) and `AccessibilitySettings` (high contrast) | `ColorValuesChanged` and `HighContrastChanged` events |
| macOS | `NSColor.controlAccentColor`, "Increase contrast", the `AppleInterfaceStyle` user default | Notifications delivered through the application's main run loop |

Building on Linux needs the GLib development files (`libglib2.0-dev` on Debian/Ubuntu, `glib2-devel` on Fedora) and `pkg-config`. Inside a Flatpak sandbox only the portal values are available.

## Features

| Feature | Default | Effect |
|---|---|---|
| `serde` | yes | JSON (de)serialization with the names of the oito2 Tauri plugins |
| `watch` | no | `watch` and `SettingsWatcher` |

## License

GPL-3.0-only — see [LICENSE](LICENSE).
