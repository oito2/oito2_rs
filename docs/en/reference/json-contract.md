🌐 [Português](../../pt-br/reference/json-contract.md) | **English** | 🏠 [Index](../index.md)

---

# JSON Contract

With the `serde` feature, the types serialize to the JSON the oito2 Tauri plugins send to their webview. New fields are always optional; deserialization fills missing fields with their defaults.

## Names

| Type | Format | Values |
|---|---|---|
| `OperatingSystem` | lowercase | `android`, `fuchsia`, `ios`, `linux`, `macos`, `windows` |
| `DesktopEnvironment` | lowercase | `gnome`, `kde`, `xfce`, `budgie`, `cosmic`, `cinnamon`, `mate`, `lxqt`, `pantheon`, `unity`, `unknown` |
| `WindowButton` | lowercase | `minimize`, `maximize`, `close` |
| `WindowControlStyle` | lowercase | `gnome`, `breeze`, `xfwm`, `cosmic`, `windows`, `macos`, `none` |
| `DecorationMode` | kebab-case | `native`, `undecorated`, `gtk-headerless` |
| `AccentColor` | lowercase | `blue`, `teal`, `green`, `yellow`, `orange`, `red`, `pink`, `purple`, `slate` |
| `ColorScheme`, `Density` | lowercase | `light`, `dark`; `pointer`, `touch` |
| `SettingValue` | untagged | a JSON string or boolean |

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

`colorScheme`, `config.kdeLookAndFeelPackage` and `config.cosmicIsDark` are additions of oito2_rs to the plugins' original contract.

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

Serialized only:

```json
{ "kind": "unsupported", "message": "unsupported: watching the settings on this platform" }
```

`kind` is `unsupported` or `internal`; `message` is the error's display text.
