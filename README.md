# oito2_rs

Rust crates with the platform rules shared by the oito2 user interfaces: desktop environment detection, window button layout and the system settings (accent color, high contrast, light or dark scheme) of Linux, Windows and macOS, read and watched live.

[![CI](https://github.com/oito2/oito2_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/oito2/oito2_rs/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-1.90%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/github/license/oito2/oito2_rs?label=license)](LICENSE)

🌐 **Language:** English · [Português](docs/pt-br/leiame.md)

## Table of Contents

- [Overview](#overview)
- [Prerequisites & Quick Installation](#prerequisites--quick-installation)
- [Documentation](#documentation)
- [License](#license)

## Overview

oito2 apps are written in Dart + Flutter ([oito2_ui](https://github.com/oito2/oito2_ui)) and in Rust: Tauri v2 apps with [oito2_uikit](https://github.com/oito2/oito2_uikit) and native apps with oito2_iced. This workspace holds the platform logic the Rust side shares, so the Tauri plugins and the native app follow the system the same way the Flutter packages do. oito2_ui stays the source of truth: its parity fixture is checked here in CI.

Two crates, released together with one version:

| Crate | What it holds |
|---|---|
| [`oito2_foundation`](crates/oito2_foundation/README.md) | Pure types and rules, no I/O: `Platform`, `DesktopEnvironment`, `WindowButtonLayout`, `WindowControlStyle`, `DecorationMode`, `AccentColor`, `ColorScheme`, `RawSettings` and `SystemSettings` with `fallback` and `compose`. |
| [`oito2_settings`](crates/oito2_settings/README.md) | Readers and watchers of the system stores: GSettings, configuration files and the settings portal on Linux, the Windows Runtime on Windows, AppKit on macOS. |

Main features:

- **Desktop detection** — GNOME, KDE Plasma, Xfce, Budgie, COSMIC, Cinnamon, MATE, LXQt, Pantheon and Unity, from `XDG_CURRENT_DESKTOP` and `DESKTOP_SESSION`.
- **Window controls** — the button layout of each desktop (`button-layout`, `kwinrc`, `xfwm4.xml`, COSMIC flags) and the style to draw them in.
- **System settings** — accent color, high contrast and light or dark scheme, with the per-desktop fallbacks of oito2_ui.
- **Live changes** — `watch` reports every change from a background thread, with no main loop required on Linux.
- **Same JSON as the plugins** — `serde` output identical to what the oito2_uikit Tauri plugins send to their webview.

## Prerequisites & Quick Installation

| OS | Status | Minimum version | Runtime dependencies |
|---|---|---|---|
| Linux | Supported | Any distribution with GLib 2.56+ | `libglib2.0-0` (GIO); a D-Bus session bus and `xdg-desktop-portal` for the portal values |
| Windows | Supported | Windows 10 | — |
| macOS | Supported | macOS 10.14 | — |

Requirements to build:

- **Rust 1.90** or later (`rustup` installs the pinned 1.99.0 toolchain inside this repository).
- **Linux only:** the GLib development files and `pkg-config`.

```bash
# Debian / Ubuntu
sudo apt install libglib2.0-dev pkg-config
# Fedora
sudo dnf install glib2-devel pkgconf-pkg-config
```

Add the crates to a project:

```bash
cargo add oito2_settings --features watch   # readers, watcher and the re-exported foundation
cargo add oito2_foundation                  # only the pure rules, no system dependency
```

Or try them from a clone:

```bash
git clone https://github.com/oito2/oito2_rs.git
cd oito2_rs
cargo run -p oito2_settings --example print_settings --features watch
```

See [Installation](docs/en/getting-started/installation.md) for every option.

## Documentation

- [Documentation index](docs/en/index.md)
- [Quickstart](docs/en/getting-started/quickstart.md)
- [How oito2_rs works](docs/en/concepts/how-oito2_rs-works.md)
- [API reference](docs/en/reference/api.md) and the full API on docs.rs
- [Contributing](CONTRIBUTING.md) · [Code of Conduct](CODE_OF_CONDUCT.md) · [Security Policy](SECURITY.md)

## License

GPL-3.0 — see [LICENSE](LICENSE).
