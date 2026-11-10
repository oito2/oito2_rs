🌐 [English](../../en/concepts/architecture.md) | **Português** | 🏠 [Índice](../index.md)

---

# Arquitetura

## Fluxo de dados

```text
                       oito2_settings                               oito2_foundation
┌──────────────────────────────────────────────────┐   ┌──────────────────────────────────┐
│ linux/  (GSettings, files, portal)  ─┐            │   │ RawSettings                      │
│ windows/ (UISettings, Accessibility) ├─ read() ───┼──▶│   └─ SystemSettings::compose ──▶ │ SystemSettings
│ macos/  (AppKit, user defaults)     ─┘            │   │ DesktopEnvironment, layouts,     │
│ watcher.rs ◀── */watch.rs (change events)         │   │ styles, accents, color schemes   │
└──────────────────────────────────────────────────┘   └──────────────────────────────────┘
```

## Organização

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

## Principais dependências

| Dependência | Onde | Por quê |
|---|---|---|
| `serde` | ambos (feature `serde`) | O contrato JSON compartilhado com os plugins Tauri |
| `thiserror` | ambos | Tipos de erro |
| `gio`, `glib` 0.18 | `oito2_settings`, Linux | GSettings, monitores de arquivo, D-Bus. A linha 0.18 é a do crate `gtk` que o Tauri usa, então um app com os dois liga uma única cópia dos bindings do GLib |
| `windows` 0.62 | `oito2_settings`, Windows | `UISettings`, `AccessibilitySettings` e seus eventos |
| `objc2`, `objc2-foundation`, `objc2-app-kit`, `block2` | `oito2_settings`, macOS | Cores e acessibilidade do AppKit, user defaults, observadores de notificação |

## Decisões de projeto

- **Composição no crate puro.** Nos espelhos em Dart e TypeScript a composição fica no pacote de configurações. Aqui `RawSettings` e `compose` ficam no `oito2_foundation`, para que consumidores e testes componham configurações sem ligar o GIO.
- **Leitores nunca falham.** Um schema, arquivo, barramento ou portal ausente deixa valores vazios; a composição fornece os padrões do desktop. O GIO aborta o processo diante de schema ou chave desconhecidos, então os dois são consultados antes de cada leitura.
- **GIO, não um crate de D-Bus.** O portal é acessado por `gio::DBusConnection`, que apps Tauri já ligam; sem `zbus` e sem subprocesso `gsettings`.
- **Um contexto principal próprio no Linux.** Os objetos GSettings, os monitores de arquivo e a assinatura D-Bus do observador ficam anexados a um contexto principal do GLib que só a thread dele roda, então ele não precisa do loop do aplicativo nem o perturba.
- **Debounce fora do GLib.** Os timeouts locais do GLib se anexam ao contexto principal padrão global, que dentro do Tauri pertence à thread principal do GTK. Por isso o debounce roda numa thread emissora separada e independente de plataforma, alimentada por um canal, que os observadores do Windows e do macOS reaproveitam.
- **`unsafe` isolado no macOS.** O workspace proíbe `unsafe`. As APIs de observador de notificação do objc2 são `unsafe`, então o `oito2_settings` só nega o lint e o libera em `src/macos/watch.rs`, com um comentário `SAFETY` em cada bloco.
- **Uma versão para os dois crates**, publicados juntos a partir de uma tag `vX.Y.Z`.
