🌐 [Português](../../pt-br/getting-started/installation.md) | **English** | 🏠 [Index](../index.md)

---

# Installation

## Requirements

| OS | Status | Minimum version | Runtime dependencies |
|---|---|---|---|
| Linux | Supported | Any distribution with GLib 2.56+ | `libglib2.0-0` (GIO); a D-Bus session bus and `xdg-desktop-portal` for the portal values |
| Windows | Supported | Windows 10 | — |
| macOS | Supported | macOS 10.14 | — |

- **Rust 1.90** or later. Inside a clone, `rustup` installs the 1.99.0 toolchain pinned in `rust-toolchain.toml`.
- **`oito2_foundation`** needs nothing else on any system.
- **`oito2_settings` on Linux** links GLib and GIO through `pkg-config`, so building needs their development files.

### Linux build dependencies

| Distribution | Command |
|---|---|
| Debian, Ubuntu | `sudo apt install libglib2.0-dev pkg-config` |
| Fedora | `sudo dnf install glib2-devel pkgconf-pkg-config` |

Windows and macOS need no system package.

## Adding the Crates

```bash
# Readers, watcher and the re-exported foundation (oito2_settings::foundation)
cargo add oito2_settings --features watch

# Only the pure rules
cargo add oito2_foundation
```

Or in `Cargo.toml`:

```toml
[dependencies]
oito2_settings = { version = "0.1", features = ["watch"] }
```

### Features

| Crate | Feature | Default | Effect |
|---|---|---|---|
| `oito2_foundation` | `serde` | yes | JSON (de)serialization of every public type |
| `oito2_settings` | `serde` | yes | JSON (de)serialization; enables `oito2_foundation/serde` |
| `oito2_settings` | `watch` | no | `watch` and `SettingsWatcher` |

### From Git

To follow the repository instead of a release:

```toml
[dependencies]
oito2_settings = { git = "https://github.com/oito2/oito2_rs", features = ["watch"] }
```

## Building from Source

```bash
git clone https://github.com/oito2/oito2_rs.git
cd oito2_rs
cargo build --workspace --all-features
cargo run -p oito2_settings --example print_settings --features watch
```

The example prints the raw values and the composed settings of the machine, then each change until you press Ctrl+C. On macOS it prints only the current values: changes are delivered through an application's main run loop, which a command-line example doesn't run.

## Uninstalling

The crates install nothing on the system: remove them from `Cargo.toml` (`cargo remove oito2_settings`).
