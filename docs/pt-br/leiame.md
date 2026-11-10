# oito2_rs

Crates Rust com as regras de plataforma compartilhadas pelas interfaces do oito2: detecção do ambiente de desktop, layout dos botões da janela e as configurações do sistema (cor de destaque, alto contraste, esquema claro ou escuro) no Linux, Windows e macOS, lidas e observadas ao vivo.

[![CI](https://github.com/oito2/oito2_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/oito2/oito2_rs/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-1.90%2B-orange?logo=rust)](https://www.rust-lang.org)
[![Licença](https://img.shields.io/github/license/oito2/oito2_rs?label=licen%C3%A7a)](../../LICENSE)

🌐 **Idioma:** [English](../../README.md) · Português

## Sumário

- [Visão geral](#visão-geral)
- [Pré-requisitos e instalação rápida](#pré-requisitos-e-instalação-rápida)
- [Documentação](#documentação)
- [Licença](#licença)

## Visão geral

Os apps do oito2 são escritos em Dart + Flutter ([oito2_ui](https://github.com/oito2/oito2_ui)) e em Rust: apps Tauri v2 com o [oito2_uikit](https://github.com/oito2/oito2_uikit) e apps nativos com o oito2_iced. Este workspace guarda a lógica de plataforma que o lado Rust compartilha, para que os plugins Tauri e o app nativo sigam o sistema do mesmo jeito que os pacotes Flutter. O oito2_ui continua sendo a fonte da verdade: a fixture de paridade dele é verificada aqui no CI.

Dois crates, publicados juntos com uma única versão:

| Crate | O que contém |
|---|---|
| [`oito2_foundation`](../../crates/oito2_foundation/README.md) | Tipos e regras puros, sem E/S: `Platform`, `DesktopEnvironment`, `WindowButtonLayout`, `WindowControlStyle`, `DecorationMode`, `AccentColor`, `ColorScheme`, `RawSettings` e `SystemSettings` com `fallback` e `compose`. |
| [`oito2_settings`](../../crates/oito2_settings/README.md) | Leitores e observadores dos armazenamentos do sistema: GSettings, arquivos de configuração e o portal de configurações no Linux, o Windows Runtime no Windows, o AppKit no macOS. |

Principais recursos:

- **Detecção do desktop** — GNOME, KDE Plasma, Xfce, Budgie, COSMIC, Cinnamon, MATE, LXQt, Pantheon e Unity, a partir de `XDG_CURRENT_DESKTOP` e `DESKTOP_SESSION`.
- **Controles da janela** — o layout dos botões de cada desktop (`button-layout`, `kwinrc`, `xfwm4.xml`, flags do COSMIC) e o estilo em que desenhá-los.
- **Configurações do sistema** — cor de destaque, alto contraste e esquema claro ou escuro, com os fallbacks por desktop do oito2_ui.
- **Mudanças ao vivo** — `watch` informa cada mudança a partir de uma thread em segundo plano, sem exigir loop principal no Linux.
- **O mesmo JSON dos plugins** — a saída `serde` é idêntica à que os plugins Tauri do oito2_uikit enviam ao webview.

## Pré-requisitos e instalação rápida

| SO | Situação | Versão mínima | Dependências em tempo de execução |
|---|---|---|---|
| Linux | Suportado | Qualquer distribuição com GLib 2.56+ | `libglib2.0-0` (GIO); um barramento de sessão D-Bus e o `xdg-desktop-portal` para os valores do portal |
| Windows | Suportado | Windows 10 | — |
| macOS | Suportado | macOS 10.14 | — |

Requisitos para compilar:

- **Rust 1.90** ou mais novo (dentro deste repositório, o `rustup` instala o toolchain 1.99.0 fixado).
- **Só no Linux:** os arquivos de desenvolvimento do GLib e o `pkg-config`.

```bash
# Debian / Ubuntu
sudo apt install libglib2.0-dev pkg-config
# Fedora
sudo dnf install glib2-devel pkgconf-pkg-config
```

Adicione os crates a um projeto:

```bash
cargo add oito2_settings --features watch   # readers, watcher and the re-exported foundation
cargo add oito2_foundation                  # only the pure rules, no system dependency
```

Ou experimente a partir de um clone:

```bash
git clone https://github.com/oito2/oito2_rs.git
cd oito2_rs
cargo run -p oito2_settings --example print_settings --features watch
```

Veja [Instalação](getting-started/installation.md) para todas as opções.

## Documentação

- [Índice da documentação](index.md)
- [Início rápido](getting-started/quickstart.md)
- [Como o oito2_rs funciona](concepts/how-oito2_rs-works.md)
- [Referência da API](reference/api.md) e a API completa no docs.rs
- [Como contribuir](contribuindo.md) · [Código de Conduta](codigo-de-conduta.md) · [Política de Segurança](seguranca.md)

## Licença

GPL-3.0 — veja [LICENSE](../../LICENSE).
