🌐 [Português](../../pt-br/concepts/why-oito2_rs.md) | **English** | 🏠 [Index](../index.md)

---

# Why oito2_rs

oito2 apps follow the desktop they run on: the window buttons sit where the desktop puts them, in the desktop's style, with its accent color, contrast and light or dark scheme. The rules that decide this are not trivial — ten Linux desktops store their preferences in different places, with different formats and defaults — and they were first written in Dart for the Flutter packages of oito2_ui.

The Rust side of oito2 needs the same rules twice:

- **oito2_uikit** — its Tauri plugins read the system for a Svelte webview.
- **oito2_iced** — a native Rust app that draws its own window chrome.

Keeping one copy per consumer would let them drift apart. oito2_rs is the single Rust copy:

- **One implementation** of detection, layout parsing and composition, tested against the fixture oito2_ui exports, so Dart and Rust agree case by case.
- **No UI dependency** — no Tauri, no toolkit: the pure rules (`oito2_foundation`) have no system dependency at all, and the readers (`oito2_settings`) depend only on GIO on Linux and on the system frameworks elsewhere.
- **The plugins' JSON** — the `serde` output equals what the Tauri plugins already send, so the webview code doesn't change when the plugins switch to these crates.
- **Watching without a main loop** — the Linux watcher runs its own GLib main context, so it works inside Tauri (which runs GTK) and inside a winit or Iced app (which doesn't).

## Who It Is For

The crates are written for these consumers:

| Consumer | Uses |
|---|---|
| `tauri-plugin-oito2-settings` (oito2_uikit) | `read_raw_settings` and `watch`, forwarding `RawSettings` to the webview |
| `tauri-plugin-oito2-window` (oito2_uikit) | `DecorationMode`, `window_menu` |
| oito2_iced | `read_system_settings`, `watch` and the foundation types |

## What It Doesn't Do

- It doesn't draw anything or change any window: it reports values.
- It doesn't write settings.
- It reads only the settings oito2 follows; it isn't a general settings library.
