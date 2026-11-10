🌐 [English](../../en/reference/json-contract.md) | **Português** | 🏠 [Índice](../index.md)

---

# Contrato JSON

Com a feature `serde`, os tipos são serializados no JSON que os plugins Tauri do oito2 enviam ao webview. Campos novos são sempre opcionais; a desserialização preenche os campos ausentes com os valores padrão.

## Nomes

| Tipo | Formato | Valores |
|---|---|---|
| `OperatingSystem` | minúsculas | `android`, `fuchsia`, `ios`, `linux`, `macos`, `windows` |
| `DesktopEnvironment` | minúsculas | `gnome`, `kde`, `xfce`, `budgie`, `cosmic`, `cinnamon`, `mate`, `lxqt`, `pantheon`, `unity`, `unknown` |
| `WindowButton` | minúsculas | `minimize`, `maximize`, `close` |
| `WindowControlStyle` | minúsculas | `gnome`, `breeze`, `xfwm`, `cosmic`, `windows`, `macos`, `none` |
| `DecorationMode` | kebab-case | `native`, `undecorated`, `gtk-headerless` |
| `AccentColor` | minúsculas | `blue`, `teal`, `green`, `yellow`, `orange`, `red`, `pink`, `purple`, `slate` |
| `ColorScheme`, `Density` | minúsculas | `light`, `dark`; `pointer`, `touch` |
| `SettingValue` | sem tag | uma string ou um booleano JSON |

## `RawSettings`

```json
{
  "xdgCurrentDesktop": "ubuntu:GNOME",
  "desktopSession": "ubuntu",
  "gsettings": {
    "org.gnome.desktop.wm.preferences/button-layout": "appmenu:minimize,maximize,close",
    "org.gnome.desktop.a11y.interface/high-contrast": false
  },
  "config": {
    "kwinButtonsOnLeft": null,
    "kwinButtonsOnRight": null,
    "kdeAccentColor": null,
    "kdeColorScheme": null,
    "kdeLookAndFeelPackage": null,
    "xfwmButtonLayout": null,
    "cosmicShowMinimize": null,
    "cosmicShowMaximize": null,
    "cosmicIsDark": null
  },
  "systemAccent": "#3584E4",
  "systemHighContrast": false,
  "colorScheme": "dark"
}
```

`colorScheme`, `config.kdeLookAndFeelPackage` e `config.cosmicIsDark` são acréscimos do oito2_rs ao contrato original dos plugins.

## `SystemSettings`

```json
{
  "platform": { "os": "linux", "isWeb": false },
  "desktopEnvironment": "gnome",
  "windowControlStyle": "gnome",
  "windowButtonLayout": { "leading": [], "trailing": ["minimize", "maximize", "close"] },
  "accentColor": "#3584E4",
  "colorScheme": "dark",
  "highContrast": false
}
```

## `Error` (`oito2_settings`)

Só serializado:

```json
{ "kind": "unsupported", "message": "unsupported: watching the settings on this platform" }
```

`kind` é `unsupported` ou `internal`; `message` é o texto de exibição do erro.
