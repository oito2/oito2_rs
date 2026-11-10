🌐 [English](../../../en/guides/workflows/examples.md) | **Português** | 🏠 [Índice](../../index.md)

---

# Exemplos de integração

## Um plugin Tauri

Repasse os valores brutos para um webview, que os compõe com as regras TypeScript do `@oito2/ui-settings`:

```rust,ignore
use tauri::{Emitter, Manager, Runtime, plugin::{Builder, TauriPlugin}};

#[tauri::command]
async fn read_settings() -> oito2_settings::RawSettings {
    tauri::async_runtime::spawn_blocking(oito2_settings::read_raw_settings)
        .await
        .unwrap_or_default()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("oito2-settings")
        .invoke_handler(tauri::generate_handler![read_settings])
        .setup(|app, _api| {
            let handle = app.clone();
            let watcher = oito2_settings::watch(move |raw| {
                let _ = handle.emit("oito2-settings://changed", raw);
            })?;
            // The watcher reports while the app holds it.
            app.manage(watcher);
            Ok(())
        })
        .build()
}
```

## Um loop de eventos

Qualquer loop de eventos pode receber as mudanças por um canal; o callback roda numa thread do observador:

```rust,no_run
use std::sync::mpsc;
use oito2_settings::{Platform, SystemSettings};

fn main() -> Result<(), oito2_settings::Error> {
    let (sender, receiver) = mpsc::channel();
    let _watcher = oito2_settings::watch(move |raw| {
        let _ = sender.send(SystemSettings::compose(raw, Platform::current()));
    })?;

    // In the loop: apply every new value.
    while let Ok(settings) = receiver.recv() {
        println!("apply {:?}", settings.color_scheme);
    }
    Ok(())
}
```

Num toolkit gráfico, troque o `recv` bloqueante pelo mecanismo de despertar do próprio toolkit (por exemplo um proxy do loop de eventos do winit, ou um canal assíncrono ligado a uma subscription do Iced).

## Compor JSON vindo de outro lugar

```rust
use oito2_foundation::{Platform, RawSettings, SystemSettings};

fn main() -> Result<(), serde_json::Error> {
    let raw: RawSettings = serde_json::from_str(r#"{ "xdgCurrentDesktop": "KDE" }"#)?;
    let settings = SystemSettings::compose(&raw, Platform::current());
    println!("{settings:?}");
    Ok(())
}
```

Campos ausentes recebem os valores padrão, então o JSON de leitores mais antigos (sem `colorScheme`, `kdeLookAndFeelPackage` ou `cosmicIsDark`) continua sendo lido.

## Decorações da janela

```rust
use oito2_foundation::{DecorationMode, window_menu};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode: DecorationMode = "gtk-headerless".parse()?;
    let default = DecorationMode::platform_default(); // gtk-headerless on Linux, native on macOS, undecorated elsewhere
    let (x, y) = window_menu::validate_position(120.0, 18.0)?;
    let show_menu = window_menu::supported(); // Linux and Windows
    println!("{mode} {default} ({x}, {y}) {show_menu}");
    Ok(())
}
```
