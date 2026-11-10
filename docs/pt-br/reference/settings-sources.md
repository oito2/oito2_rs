🌐 [English](../../en/reference/settings-sources.md) | **Português** | 🏠 [Índice](../index.md)

---

# Fontes das configurações

## Linux — Sessão

| Variável | Campo do `RawSettings` |
|---|---|
| `XDG_CURRENT_DESKTOP` | `xdg_current_desktop` |
| `DESKTOP_SESSION` | `desktop_session` |

## Linux — GSettings

Guardados em `gsettings` por `schema/chave`, quando o schema está instalado.

| Schema | Chave | Usada para |
|---|---|---|
| `org.gnome.desktop.wm.preferences` | `button-layout` | Layout dos botões do GNOME, Budgie, Pantheon, Unity, LXQt e desconhecido |
| `org.cinnamon.desktop.wm.preferences` | `button-layout` | Layout dos botões do Cinnamon |
| `org.mate.Marco.general` | `button-layout` | Layout dos botões do MATE |
| `org.gnome.desktop.interface` | `accent-color` | Destaque nomeado |
| `org.gnome.desktop.interface` | `gtk-theme` | Alto contraste ("highcontrast" no nome) |
| `org.gnome.desktop.interface` | `color-scheme` | Fallback do esquema de cores |
| `org.gnome.desktop.a11y.interface` | `high-contrast` | Alto contraste |

## Linux — Arquivos de configuração

Relativos a `XDG_CONFIG_HOME` (ou `$HOME/.config`); arquivos maiores que 1 MiB são ignorados.

| Arquivo | Entrada | Campo do `ConfigValues` |
|---|---|---|
| `kwinrc` | `[org.kde.kdecoration2] ButtonsOnLeft` | `kwin_buttons_on_left` |
| `kwinrc` | `[org.kde.kdecoration2] ButtonsOnRight` | `kwin_buttons_on_right` |
| `kdeglobals` | `[General] AccentColor` (`r,g,b`) | `kde_accent_color` (`#RRGGBB`) |
| `kdeglobals` | `[General] ColorScheme` | `kde_color_scheme` |
| `kdeglobals` | `[KDE] LookAndFeelPackage` | `kde_look_and_feel_package` |
| `xfce4/xfconf/xfce-perchannel-xml/xfwm4.xml` | propriedade `button_layout` | `xfwm_button_layout` |
| `cosmic/com.system76.CosmicTk/v1/show_minimize` | `true`/`false` | `cosmic_show_minimize` |
| `cosmic/com.system76.CosmicTk/v1/show_maximize` | `true`/`false` | `cosmic_show_maximize` |
| `cosmic/com.system76.CosmicTheme.Mode/v1/is_dark` | `true`/`false` | `cosmic_is_dark` |

## Linux — Portal de configurações

`org.freedesktop.portal.Desktop`, `/org/freedesktop/portal/desktop`, interface `org.freedesktop.portal.Settings`, namespace `org.freedesktop.appearance`.

| Chave | Tipo | Campo do `RawSettings` |
|---|---|---|
| `accent-color` | `(ddd)`, componentes de 0 a 1 (fora da faixa quando não definida) | `system_accent` |
| `contrast` | `u`: 1 alto, 0 normal | `system_high_contrast` |
| `color-scheme` | `u`: 1 escuro, 2 claro, 0 sem preferência | `color_scheme` |

## Windows

| API | Campo do `RawSettings` |
|---|---|
| `UISettings.GetColorValue(UIColorType::Accent)` | `system_accent` |
| `UISettings.GetColorValue(UIColorType::Background)` | `color_scheme` |
| `AccessibilitySettings.HighContrast` | `system_high_contrast` |

## macOS

| API | Campo do `RawSettings` |
|---|---|
| `NSColor.controlAccentColor` (sRGB) | `system_accent` |
| `NSWorkspace.accessibilityDisplayShouldIncreaseContrast` | `system_high_contrast` |
| `AppleInterfaceStyle` de `NSUserDefaults` | `color_scheme` |
