🌐 [English](../../en/reference/api.md) | **Português** | 🏠 [Índice](../index.md)

---

# API

A documentação completa, com todas as assinaturas, fica no docs.rs (`cargo doc --open` num clone). Esta página lista os itens públicos.

## `oito2_foundation`

| Item | Tipo | Finalidade |
|---|---|---|
| `OperatingSystem` | enum | `Android`, `Fuchsia`, `Ios`, `Linux`, `Macos`, `Windows`; `current`, `as_str`, `from_name`, `ALL` |
| `Platform` | struct | `{ os, is_web }`; `new`, `current`, `is_desktop_native`, `is_touch`, `is_web_touch`, `uses_touch_input`, `is_linux`, `is_windows`, `is_macos`, `is_android`, `is_ios`, `density` |
| `Density` | enum | `Pointer`, `Touch` |
| `DesktopEnvironment` | enum | 10 desktops e `Unknown`; `detect`, `as_str`, `from_name`, `ALL` |
| `WindowButton` | enum | `Minimize`, `Maximize`, `Close`; `as_str` |
| `WindowButtonLayout` | struct | `{ leading, trailing }`; `new`, `parse_gnome`, `parse_kde`, `parse_xfwm`, `default_for`, `is_empty` |
| `WindowControlStyle` | enum | `Gnome`, `Breeze`, `Xfwm`, `Cosmic`, `Windows`, `Macos`, `None`; `resolve`, `as_str`, `from_name`, `ALL` |
| `DecorationMode` | enum | `Native`, `Undecorated`, `GtkHeaderless`; `platform_default`, `as_str`, `FromStr`, `ALL` |
| `ParseDecorationModeError` | struct | Erro do `FromStr` de `DecorationMode`, com o `name` rejeitado |
| `window_menu` | módulo | `supported`, `validate_position`, `MAX_COORDINATE`, `InvalidPositionError` |
| `AccentColor` | enum | Os nove nomes da paleta do GNOME; `hex`, `as_str`, `from_name`, `ALL`, `FALLBACK` |
| `ColorScheme` | enum | `Light`, `Dark`; `from_portal`, `from_gnome_preference`, `as_str`, `from_name`, `ALL` |
| `RawSettings` | struct | Os valores que um leitor coletou; `gsetting_text`, `gsetting_flag` |
| `ConfigValues` | struct | Os valores dos arquivos de configuração do desktop |
| `SettingValue` | enum | Um valor GSettings: `Text(String)` ou `Flag(bool)` |
| `raw::schemas`, `raw::files`, `raw::GSETTINGS_KEYS` | consts | Os schemas, chaves e arquivos que os leitores usam |
| `SystemSettings` | struct | `{ platform, desktop_environment, window_control_style, window_button_layout, accent_color, color_scheme, high_contrast }`; `fallback`, `compose` |

## `oito2_settings`

| Item | Tipo | Finalidade |
|---|---|---|
| `read_raw_settings()` | fn | Lê os armazenamentos da plataforma em execução para um `RawSettings` |
| `read_system_settings()` | fn | `read_raw_settings` composto para `Platform::current()` |
| `watch(on_change)` | fn (feature `watch`) | Inicia um `SettingsWatcher`; `Error::Unsupported` fora do Linux, Windows e macOS |
| `SettingsWatcher` | struct (feature `watch`) | Informa mudanças até ser descartado; `Send + Sync` |
| `Error`, `Result` | enum, alias | `Unsupported(String)`, `Internal(String)` |
| `foundation` | reexport | O crate `oito2_foundation` inteiro |
| `RawSettings`, `SystemSettings`, `Platform`, `ColorScheme`, `ConfigValues`, `SettingValue` | reexports | Do `oito2_foundation` |

## Features

| Crate | Feature | Padrão | Efeito |
|---|---|---|---|
| `oito2_foundation` | `serde` | sim | `Serialize`/`Deserialize` para todo tipo público |
| `oito2_settings` | `serde` | sim | Ativa `oito2_foundation/serde` e serializa o `Error` |
| `oito2_settings` | `watch` | não | `watch`, `SettingsWatcher` |
