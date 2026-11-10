🌐 [English](../../en/architecture/parity.md) | **Português** | 🏠 [Índice](../index.md)

---

# Paridade com o oito2_ui

O oito2_ui exporta o resultado esperado de cada caso das regras para `tools/parity/fixtures/foundation.json`. Uma cópia fica em `crates/oito2_foundation/tests/fixtures/`, com o `manifest.json` do mesmo commit e um `README.md` que registra esse commit.

## Testes

`crates/oito2_foundation/tests/parity.rs` confere cada seção que os crates Rust implementam (`contrast` fica com o oito2_iced):

| Seção | Casos | Conferido contra |
|---|---|---|
| `desktopEnvironments` | 24 | `DesktopEnvironment::detect` |
| `windowButtonLayouts.gnome` / `kde` / `xfwm` | 10 / 5 / 7 | `parse_gnome`, `parse_kde`, `parse_xfwm` |
| `windowButtonLayouts.defaultFor` | 7 | `WindowButtonLayout::default_for` |
| `windowButtonLayouts.resolve` | 144 | `WindowControlStyle::resolve` |
| `platforms` | 12 | as flags de `Platform` |
| `systemSettings` | 24 | `SystemSettings::fallback`, campo a campo e como JSON |

A fixture cobre só o `fallback`. As regras de composição são testadas em `tests/compose.rs` com os casos do `compose.test.ts` do oito2_uikit e do `settings_reader_test.dart` do oito2_ui_settings.

## Workflow

`.github/workflows/parity.yml` roda toda segunda-feira às 06:00 UTC e sob demanda: faz checkout de `oito2/oito2_ui` e compara o `foundation.json` dele com a cópia, byte a byte. Uma falha significa que o oito2_ui mudou: copie os dois arquivos de novo, atualize o commit em `tests/fixtures/README.md`, porte a mudança e rode as verificações.
