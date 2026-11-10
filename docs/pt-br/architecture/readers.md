🌐 [English](../../en/architecture/readers.md) | **Português** | 🏠 [Índice](../index.md)

---

# Leitores

`read_raw_settings()` chama o leitor da plataforma em execução (`crates/oito2_settings/src/{linux,windows,macos}/`). Todo leitor devolve um `RawSettings` e nunca falha.

## Linux

`linux::read` preenche, em ordem:

1. **Variáveis de sessão** — `XDG_CURRENT_DESKTOP`, `DESKTOP_SESSION`.
2. **GSettings** (`linux/gsettings.rs`) — cada `(schema, chave)` de `oito2_foundation::raw::GSETTINGS_KEYS` cujo schema está instalado e tem a chave. O GIO aborta o processo diante de schema ou chave desconhecidos, então `SettingsSchemaSource::lookup` e `has_key` rodam antes. Valores texto e booleanos são guardados, por `schema/chave`.
3. **Arquivos de configuração** (`config.rs`) — em `XDG_CONFIG_HOME` quando é um caminho absoluto, senão em `$HOME/.config`. Cada arquivo de `raw::files` é lido quando existe, é um arquivo regular e tem no máximo 1 MiB, e é interpretado pelo `parsers.rs` (grupos INI para `kwinrc`/`kdeglobals`, a propriedade XML do xfconf para o `xfwm4.xml`, `true`/`false` para o COSMIC).
4. **Portal de configurações** (`linux/portal.rs`) — no barramento de sessão, `org.freedesktop.portal.Settings.ReadOne` (com recuo para o `Read`, mais antigo) para as chaves `accent-color` (`(ddd)`, componentes de 0 a 1), `contrast` (`u`, 1 é alto) e `color-scheme` (`u`: 1 escuro, 2 claro, 0 sem preferência) de `org.freedesktop.appearance`. Cada chamada expira em 500 ms; sem barramento ou portal, nada muda.

## Windows

`windows::read` (`windows/mod.rs`):

- `UISettings.GetColorValue(Accent)` → `system_accent`.
- `UISettings.GetColorValue(Background)` → `color_scheme`: `Light` quando `(5·G + 2·R + B) / 8 > 128`, senão `Dark`.
- `AccessibilitySettings.HighContrast` → `system_high_contrast`.

## macOS

`macos::read` (`macos/mod.rs`):

- `NSColor.controlAccentColor` convertida para sRGB → `system_accent`.
- `NSWorkspace.accessibilityDisplayShouldIncreaseContrast` → `system_high_contrast`.
- `AppleInterfaceStyle` de `NSUserDefaults.standardUserDefaults` → `color_scheme`: `Dark` quando vale `Dark`, senão `Light`. O user default é lido no lugar de `NSApp.effectiveAppearance`, que exige um `NSApplication` em execução.

## Outras plataformas

`read_raw_settings` devolve `RawSettings::default()`.
