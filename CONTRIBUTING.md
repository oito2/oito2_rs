# Contributing to oito2_rs

🌐 **Language:** English · [Português](docs/pt-br/contribuindo.md)

Thanks for your interest in oito2_rs. This guide covers the development setup, the rules every change follows and how releases are made.

## Where Changes Start

oito2_ui is the source of truth of every rule in `oito2_foundation`. A new desktop, a new setting or a different fallback is changed there first, reaches its parity fixture (`tools/parity/fixtures/foundation.json`), and only then is ported here. Bugs in the Rust readers, watchers and API are fixed directly in this repository.

## Development Setup

- Rust through `rustup`: `rust-toolchain.toml` pins 1.99.0 with `rustfmt` and `clippy`. The MSRV is 1.90.
- Linux: `libglib2.0-dev` and `pkg-config` (or your distribution's equivalent), plus `dbus` for `dbus-run-session`.
- [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) for the license and advisory check.

```bash
git clone https://github.com/oito2/oito2_rs.git
cd oito2_rs
rustup toolchain install 1.90 --profile minimal
cargo install cargo-deny --locked
```

## Repository Layout

| Path | Content |
|---|---|
| `crates/oito2_foundation/` | Pure types and rules; `tests/parity.rs` and `tests/compose.rs`; the parity fixture under `tests/fixtures/` |
| `crates/oito2_settings/` | Platform readers (`src/linux/`, `src/windows/`, `src/macos/`), the watcher (`src/watcher.rs`), `examples/print_settings.rs` |
| `.github/workflows/` | `ci.yml`, `parity.yml`, `release.yml` |
| `deny.toml` | Accepted licenses, ignored advisories and allowed sources |
| `docs/` | This documentation, in English (`en/`) and Portuguese (`pt-br/`) |

## Checks

Run all of them before opening a pull request; CI runs the same ones on Linux, Windows and macOS.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked
cargo deny check
dbus-run-session -- cargo test --workspace --all-features --locked   # Linux; plain `cargo test` elsewhere
cargo test --workspace --no-default-features --locked
cargo +1.90 check --workspace --all-targets --all-features --locked
```

## Code Rules

- **License header.** Every hand-written `.rs` file starts with the GPL-3.0 header used in `crates/*/src/lib.rs` ("Copyright (C) 2026  OITO2").
- **Comments in English**, describing what the code does.
- **Documentation.** Every public item has rustdoc (`#![deny(missing_docs)]`).
- **Lints.** `unsafe_code` is forbidden in the workspace; clippy `unwrap_used`, `panic` and `todo` are denied (relaxed in tests by `clippy.toml`). `oito2_settings` declares the same lints with `unsafe_code = "deny"`, and only `src/macos/watch.rs` allows unsafe code, each block with a `// SAFETY:` comment.
- **JSON contract.** The `serde` output stays identical to what the oito2_uikit plugins send. New fields are optional and additive; every type with a `serde` contract has a serialization test.
- **Linux stores** go through GIO (`gio`/`glib` 0.18, the line of the `gtk` crate Tauri uses) and the settings portal over `gio::DBusConnection`: no `gsettings` subprocess and no `zbus`.

## Parity Fixture

`crates/oito2_foundation/tests/fixtures/foundation.json` and `manifest.json` are copies of oito2_ui's fixtures. The weekly **Parity** workflow compares them with `oito2_ui@main` and fails when oito2_ui moved. To update, copy both files again, record the source commit in `tests/fixtures/README.md`, port the change and run the checks.

## Manual Tests

CI can't change a desktop's theme, so the readers and watchers are also checked by hand. Run:

```bash
cargo run -p oito2_settings --example print_settings --features watch
```

| OS | Change while the example runs | Expected |
|---|---|---|
| Linux (GNOME family) | Window buttons layout, accent, light/dark style, high contrast | One `--- changed ---` block per change, with the new values |
| Linux (KDE, Xfce, COSMIC) | Title bar buttons, accent, color scheme | Same |
| Windows | Accent color, app mode (light/dark), contrast theme | Same |
| macOS | Accent, appearance, "Increase contrast" | The example prints the current values; changes are delivered only inside an app that runs the main run loop (see [How oito2_rs works](docs/en/concepts/how-oito2_rs-works.md)) |

## Pull Requests

- One topic per pull request, with tests for the behavior it changes.
- Update the crate's `CHANGELOG.md` under `[Unreleased]` for user-visible changes, and the documentation in English and Portuguese.
- Fill in the pull request template.

## Releases

Both crates share the workspace version (`[workspace.package] version` in `Cargo.toml`).

1. Set the new version in `Cargo.toml`, move each `CHANGELOG.md` `[Unreleased]` section to the version with its date, and commit.
2. Tag the commit and push the tag:

   ```bash
   git tag -a v0.1.0 -m "Release 0.1.0"
   git push origin v0.1.0
   ```

3. The **Release** workflow checks that the tag equals the workspace version, publishes the crates to crates.io through trusted publishing and drafts a GitHub Release with the changelog sections. Publish the draft after reviewing it.

The very first publication of each crate is manual, because trusted publishing can only be configured for a crate that already exists on crates.io:

```bash
cargo publish -p oito2_foundation
cargo publish -p oito2_settings
```
