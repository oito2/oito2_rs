🌐 [Português](../../pt-br/architecture/parity.md) | **English** | 🏠 [Index](../index.md)

---

# Parity with oito2_ui

oito2_ui exports the expected result of every rule case to `tools/parity/fixtures/foundation.json`. A copy lives in `crates/oito2_foundation/tests/fixtures/`, with the `manifest.json` of the same commit and a `README.md` recording that commit.

## Tests

`crates/oito2_foundation/tests/parity.rs` checks every section the Rust crates implement (`contrast` belongs to oito2_iced):

| Section | Cases | Checked against |
|---|---|---|
| `desktopEnvironments` | 24 | `DesktopEnvironment::detect` |
| `windowButtonLayouts.gnome` / `kde` / `xfwm` | 10 / 5 / 7 | `parse_gnome`, `parse_kde`, `parse_xfwm` |
| `windowButtonLayouts.defaultFor` | 7 | `WindowButtonLayout::default_for` |
| `windowButtonLayouts.resolve` | 144 | `WindowControlStyle::resolve` |
| `platforms` | 12 | the `Platform` flags |
| `systemSettings` | 24 | `SystemSettings::fallback`, field by field and as JSON |

The fixture covers `fallback` only. The composition rules are tested in `tests/compose.rs` with the cases of oito2_uikit's `compose.test.ts` and oito2_ui_settings' `settings_reader_test.dart`.

## Workflow

`.github/workflows/parity.yml` runs every Monday at 06:00 UTC and on demand: it checks out `oito2/oito2_ui` and compares its `foundation.json` with the copy byte for byte. A failure means oito2_ui moved: copy both files again, update the commit in `tests/fixtures/README.md`, port the change and run the checks.
