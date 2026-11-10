🌐 [English](../../en/getting-started/quickstart.md) | **Português** | 🏠 [Índice](../index.md)

---

# Início rápido

## Ler as configurações uma vez

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

`read_system_settings` lê todos os armazenamentos da plataforma em execução para um `RawSettings` e o compõe para `Platform::current()`. A leitura nunca falha: um armazenamento que não pode ser lido deixa seus valores vazios, e a composição usa os padrões do desktop.

## Guardar os valores brutos

Quando os valores precisam ir para outro lugar antes (um webview, outro processo), leia os valores brutos e componha-os onde forem usados (este exemplo também usa `serde_json`):

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

## Observar as mudanças

Com a feature `watch`:

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

- O callback roda numa thread do observador, uma vez por rajada de mudanças e só quando os valores mudaram.
- O observador não informa os valores iniciais: leia-os antes com `read_raw_settings`.
- No macOS as mudanças chegam pelo run loop principal do aplicativo (todo aplicativo AppKit, inclusive winit e Tauri, roda um).

## Usar só as regras

O `oito2_foundation` funciona sem ler nada, para testes, previews ou valores vindos de outro lugar:

```rust
use oito2_foundation::{OperatingSystem, Platform, SystemSettings, WindowButtonLayout, WindowControlStyle};

let windows = Platform::new(OperatingSystem::Windows, false);
let settings = SystemSettings::fallback(windows, None);
assert_eq!(settings.window_control_style, WindowControlStyle::Windows);
assert_eq!(settings.window_button_layout, WindowButtonLayout::default_for(WindowControlStyle::Windows));
```

Próximo: [Como o oito2_rs funciona](../concepts/how-oito2_rs-works.md).
