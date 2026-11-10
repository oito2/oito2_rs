🌐 [Português](../../pt-br/concepts/architecture.md) | **English** | 🏠 [Index](../index.md)

---

# Architecture

## Data Flow

```text
                       oito2_settings                               oito2_foundation
┌──────────────────────────────────────────────────┐   ┌──────────────────────────────────┐
│ linux/  (GSettings, files, portal)  ─┐            │   │ RawSettings                      │
│ windows/ (UISettings, Accessibility) ├─ read() ───┼──▶│   └─ SystemSettings::compose ──▶ │ SystemSettings
│ macos/  (AppKit, user defaults)     ─┘            │   │ DesktopEnvironment, layouts,     │
│ watcher.rs ◀── */watch.rs (change events)         │   │ styles, accents, color schemes   │
└──────────────────────────────────────────────────┘   └──────────────────────────────────┘
```

## Layout

```text
oito2_rs/
├── Cargo.toml                 # workspace: one version, shared dependencies and lints
├── deny.toml                  # cargo-deny: licenses, advisories, sources
├── crates/
│   ├── oito2_foundation/
│   │   ├── src/               # platform, desktop_environment, window_*, decoration_mode,
│   │   │                      # accent_color, color_scheme, raw, system_settings
│   │   └── tests/             # parity.rs, compose.rs, fixtures/ (copied from oito2_ui)
│   └── oito2_settings/
│       ├── src/               # lib, error, color, parsers, config, watcher,
│       │                      # linux/{gsettings, portal, watch}, windows/{mod, watch}, macos/{mod, watch}
│       ├── examples/          # print_settings.rs
│       └── tests/             # read.rs
├── .github/workflows/         # ci.yml, parity.yml, release.yml
└── docs/                      # en/ and pt-br/
```

## Main Dependencies

| Dependency | Where | Why |
|---|---|---|
| `serde` | both (feature `serde`) | The JSON contract shared with the Tauri plugins |
| `thiserror` | both | Error types |
| `gio`, `glib` 0.18 | `oito2_settings`, Linux | GSettings, file monitors, D-Bus. The 0.18 line is the one of the `gtk` crate Tauri uses, so an app with both links one copy of the GLib bindings |
| `windows` 0.62 | `oito2_settings`, Windows | `UISettings`, `AccessibilitySettings` and their events |
| `objc2`, `objc2-foundation`, `objc2-app-kit`, `block2` | `oito2_settings`, macOS | AppKit colors and accessibility, user defaults, notification observers |

## Design Decisions

- **Composition in the pure crate.** In the Dart and TypeScript mirrors the composition lives in the settings package. Here `RawSettings` and `compose` live in `oito2_foundation`, so consumers and tests compose settings without linking GIO.
- **Readers never fail.** A missing schema, file, bus or portal leaves values empty; the composition supplies the desktop's defaults. GIO aborts the process on an unknown schema or key, so both are looked up before every read.
- **GIO, not a D-Bus crate.** The portal is reached through `gio::DBusConnection`, which Tauri apps already link; no `zbus` and no `gsettings` subprocess.
- **A main context of its own on Linux.** The watcher's GSettings objects, file monitors and D-Bus subscription are attached to a GLib main context that only its thread runs, so it neither needs nor disturbs the application's loop.
- **Debounce outside GLib.** GLib's local timeouts attach to the global default main context, which the GTK main thread owns inside Tauri. The debounce therefore runs on a separate, platform-neutral emitter thread fed by a channel, which the Windows and macOS watchers reuse.
- **Isolated `unsafe` on macOS.** The workspace forbids `unsafe`. The objc2 notification observer APIs are `unsafe`, so `oito2_settings` only denies it and allows it in `src/macos/watch.rs`, with a `SAFETY` comment on every block.
- **One version for both crates**, released together from a `vX.Y.Z` tag.
