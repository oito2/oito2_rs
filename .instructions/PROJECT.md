# Project Rules

Rules defined for this project. They take precedence over `AGENTS.md` and over the standards in `.instructions/` — use them to add a rule or to change a default. Perci creates this file once and never rewrites it.

<!-- One concise bullet per rule, under a heading for its topic. When a rule replaces a default, say which. -->

## Session Start

- Load the project memory with Sync82 (`load_project_context`, marker `.sync82.json`) before any task; update it at the end of the session.
- The development plan lives in `/srv/workspace/MyProjects/DevNotes/oito2_rs/oito2-rs.md` (decisions R1–R17, phases F0–F7). Read it before starting a phase and keep its status notes current.

## Developer Notes

- Personal notes, plans and test procedures live in `/srv/workspace/MyProjects/DevNotes/oito2_rs/` (outside the repository) and may be written in pt-BR (exception to the English-only documentation rule of `AGENTS.md`, which applies to files inside the repository).
- `.references/` (symbolic links to the local `oito2_uikit`, `oito2_ui` and `mundy` checkouts), `.sync82.json` and the whole `.claude/` folder stay out of git (machine-specific).

## Crates

- Two published crates, one workspace version: `oito2_foundation` (pure types and rules, `RawSettings`, `SystemSettings::fallback` and `compose`) and `oito2_settings` (per-OS readers and watchers). `compose` lives in the pure crate on purpose, unlike the Dart and TS mirrors, so consumers test without GIO.
- Public type names carry no `Oito2` prefix.
- The JSON contract (serde) stays identical to the one the `oito2_uikit` Tauri plugins emit: `RawSettings` camelCase, `SettingValue` untagged, `DecorationMode` kebab-case, `Error` as `{kind, message}`. New fields are always optional and additive; every type with a serde contract has a serialization test.
- The behaviour source of truth is `oito2_ui` (Dart). A rule changes there first, reaches `tools/parity/fixtures/foundation.json`, and only then is ported here. The fixture copy in `crates/oito2_foundation/tests/fixtures/` is updated only by copying both files again and recording the source commit in its `README.md`.
- Linux settings go through GIO (`gio`/`glib` 0.18, the same line as the `gtk` crate Tauri uses) and the `org.freedesktop.portal.Settings` portal over `gio::DBusConnection`; no `gsettings` subprocess, no `zbus`, no `mundy` dependency.

## Source Files

- Every hand-written `.rs` file starts with the GPL-3.0 header "Copyright (C) 2026  OITO2" (two spaces, uppercase), as in `crates/*/src/lib.rs`.
- Toolchain 1.99.0 (`rust-toolchain.toml`), edition 2024, MSRV 1.90. Lints come from the workspace: `unsafe_code = "forbid"`, clippy `unwrap_used`, `panic` and `todo` denied; `clippy.toml` relaxes them in tests. Every public item has rustdoc (`#![deny(missing_docs)]`).
- Exception to `unsafe_code = "forbid"`: `oito2_settings` declares its own lints with `unsafe_code = "deny"` and `clippy::undocumented_unsafe_blocks = "deny"`, and only `src/macos/watch.rs` (`#[allow(unsafe_code)]` on the module) may use `unsafe`, because the objc2 notification observer APIs are `unsafe`. Every `unsafe` block carries a `// SAFETY:` comment. Keep its `[lints]` in sync with `[workspace.lints]`.

## Versioning & Changelog

- One `CHANGELOG.md` per crate (Keep a Changelog), maintained by hand under `[Unreleased]`; release notes are built from them by `release.yml`.
- Pre-1.0 SemVer: a minor bump may break, a patch never does. The first publication of each crate is manual; later ones go through trusted publishing on a `vX.Y.Z` tag.

## Testing & Verification

- Run before reporting: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features`, `cargo deny check` (`deny.toml` already resolves every feature), `cargo test --workspace --all-features --locked` (on Linux inside `dbus-run-session`), `cargo +1.90 check --workspace --all-features --locked`.
- Never automate clicks or input on the user's real desktop; manual desktop checks (LiveCDs, theme switching) are the user's.

## Git

- Commit flow: write the commit message to `/srv/workspace/MyProjects/DevNotes/oito2_rs/commit-msg.txt` and hand the user `git add -A; git commit -F <that file>; git push`. Tags and publishing are the user's too.
- Git commit trailer: `Co-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>` (or the current model's name).

## Agent Conduct

- Genuine choices go through the selectable-question tool with a **[RECOMENDADO]** option.
- When a session permission rule denies a command, hand the user the command instead of working around it.
