# oito2_rs Documentation

🌐 [Português](../pt-br/index.md) | **English** | 🏠 [Back to README](../../README.md)

oito2_rs holds the platform rules shared by the Rust side of oito2: `oito2_foundation` (pure types and rules) and `oito2_settings` (readers and watchers of the system settings on Linux, Windows and macOS). These pages explain how to use the crates, how they work and how they are built.

## 🚀 Getting Started

- [Installation](getting-started/installation.md) — requirements per operating system, adding the crates, building from source.
- [Quickstart](getting-started/quickstart.md) — read, compose and watch the settings in a few lines.

## 🧠 Concepts

- [Why oito2_rs](concepts/why-oito2_rs.md) — the problem the crates solve and who uses them.
- [How oito2_rs works](concepts/how-oito2_rs-works.md) — from the system stores to `SystemSettings`.
- [Architecture](concepts/architecture.md) — data flow, layout, dependencies and design decisions.
- [Glossary](concepts/glossary.md) — the terms used across these pages.

## 🏗️ Architecture

- [Composition rules](architecture/composition.md) — how `compose` turns raw values into settings, per desktop.
- [Readers](architecture/readers.md) — how each platform's stores are read.
- [Watchers](architecture/watchers.md) — threads, debounce, deduplication and stopping.
- [Parity with oito2_ui](architecture/parity.md) — the fixture, its tests and its workflow.

## 📖 Guides

- [Integration examples](guides/workflows/examples.md) — a Tauri plugin, an event loop channel, JSON for a webview.

## 📚 Reference

- [API](reference/api.md) — every public item of both crates, and their features.
- [JSON contract](reference/json-contract.md) — the `serde` shape of every type.
- [Settings sources](reference/settings-sources.md) — every key, file and API read, per platform.

## 🛠️ Troubleshooting

- [Common issues](troubleshooting/common-issues.md) — build errors, missing values and missed changes.
