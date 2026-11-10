🌐 [Português](../../pt-br/getting-started/quickstart.md) | **English** | 🏠 [Index](../index.md)

---

# Quickstart

## Read the Settings Once

```rust
fn main() {
    let settings = oito2_settings::read_system_settings();
    println!("desktop:  {:?}", settings.desktop_environment);
    println!("buttons:  {:?}", settings.window_button_layout);
    println!("accent:   {:?}", settings.accent_color);
    println!("scheme:   {:?}", settings.color_scheme);
    println!("contrast: {}", settings.high_contrast);
}
```

`read_system_settings` reads every store of the running platform into a `RawSettings` and composes it for `Platform::current()`. Reading never fails: a store that can't be read leaves its values empty, and the composition falls back to the desktop's defaults.

## Keep the Raw Values

When the values travel somewhere else first (a webview, another process), read the raw values and compose them where they are used (this example also uses `serde_json`):

```rust
use oito2_settings::{Platform, SystemSettings};

fn main() -> Result<(), serde_json::Error> {
    let raw = oito2_settings::read_raw_settings();
    let json = serde_json::to_string(&raw)?; // the JSON the oito2 Tauri plugins send
    let settings = SystemSettings::compose(&raw, Platform::current());
    println!("{json}\n{settings:?}");
    Ok(())
}
```

## Watch the Changes

With the `watch` feature:

```rust,no_run
use oito2_settings::{Platform, SystemSettings};

fn main() -> Result<(), oito2_settings::Error> {
    let _watcher = oito2_settings::watch(|raw| {
        let settings = SystemSettings::compose(raw, Platform::current());
        println!("{:?} {:?}", settings.accent_color, settings.color_scheme);
    })?;
    // The watcher reports until it is dropped.
    std::thread::park();
    Ok(())
}
```

- The callback runs on a thread of the watcher, once per burst of changes and only when the values changed.
- The watcher doesn't report the initial values: read them first with `read_raw_settings`.
- On macOS the changes arrive through the application's main run loop (every AppKit application, winit and Tauri included, runs one).

## Use Only the Rules

`oito2_foundation` works without reading anything, for tests, previews or values that come from elsewhere:

```rust
use oito2_foundation::{OperatingSystem, Platform, SystemSettings, WindowButtonLayout, WindowControlStyle};

let windows = Platform::new(OperatingSystem::Windows, false);
let settings = SystemSettings::fallback(windows, None);
assert_eq!(settings.window_control_style, WindowControlStyle::Windows);
assert_eq!(settings.window_button_layout, WindowButtonLayout::default_for(WindowControlStyle::Windows));
```

Next: [How oito2_rs works](../concepts/how-oito2_rs-works.md).
