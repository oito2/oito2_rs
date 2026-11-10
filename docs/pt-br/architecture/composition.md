🌐 [English](../../en/architecture/composition.md) | **Português** | 🏠 [Índice](../index.md)

---

# Regras de composição

`SystemSettings::compose(&raw, platform)`, em `crates/oito2_foundation/src/system_settings.rs`, espelha o `Oito2SystemSettingsReader` do `oito2_ui_settings`. Os casos são testados em `crates/oito2_foundation/tests/compose.rs`.

## Ambiente de desktop

Só no Linux nativo (`platform.is_linux() && platform.is_desktop_native()`); `None` nos demais. `DesktopEnvironment::detect` confere cada entrada, separada por dois-pontos, de `XDG_CURRENT_DESKTOP` em ordem, depois `DESKTOP_SESSION`, ignorando maiúsculas e espaços nas pontas:

| Nomes | Ambiente |
|---|---|
| `gnome`, `gnome-classic`, `gnome-xorg`, `gnome-wayland` | `Gnome` |
| `kde`, `plasma`, `plasmawayland`, `plasmax11` | `Kde` |
| `xfce`, `xfce4`, `xubuntu` | `Xfce` |
| `budgie`, `budgie-desktop` | `Budgie` |
| `cosmic` | `Cosmic` |
| `x-cinnamon`, `cinnamon`, `cinnamon2d` | `Cinnamon` |
| `mate` | `Mate` |
| `lxqt`, `lubuntu` | `Lxqt` |
| `pantheon` | `Pantheon` |
| `unity`, `unity7` | `Unity` |
| qualquer outro | `Unknown` |

## Estilo dos controles da janela

`WindowControlStyle::resolve(platform, desktop)`: `None` na web e em sistemas touch; `Windows` no Windows; `Macos` no macOS; no Linux, `Breeze` para o KDE, `Xfwm` para o Xfce, `Cosmic` para o COSMIC e `Gnome` para todos os outros desktops.

## Layout dos botões

| Desktop | Fonte | Quando ausente |
|---|---|---|
| KDE | `ButtonsOnLeft`/`ButtonsOnRight` do `kwinrc` (`I` minimizar, `A` maximizar, `X` fechar) | `MS` / `HIAX` |
| Xfce | `button_layout` do `xfwm4.xml` (`H`, `M`, `C`; `\|` marca o título) | `O\|SHMC` |
| COSMIC | flags `show_minimize` e `show_maximize`; fechar sempre aparece | o botão aparece |
| Cinnamon | `org.cinnamon.desktop.wm.preferences button-layout` | padrão do estilo |
| MATE | `org.mate.Marco.general button-layout` | padrão do estilo |
| GNOME, Budgie, Pantheon, Unity, LXQt, desconhecido | `org.gnome.desktop.wm.preferences button-layout` | padrão do estilo |

Em todos os formatos, entradas desconhecidas são ignoradas, cada botão fica só na primeira posição em que aparece, e só as duas primeiras partes em torno do separador contam (`minimize:maximize:close` dá trailing `[maximize]`). O padrão do estilo é minimizar, maximizar e fechar no fim, exceto fechar, minimizar e maximizar no início para `macos` e nenhum botão para `none`.

## Cor de destaque

- **KDE:** `AccentColor` do `kdeglobals` (`r,g,b`, guardado como `#RRGGBB`).
- **Demais desktops Linux:** `org.gnome.desktop.interface accent-color`, quando é um dos nove nomes da paleta (`blue` `#3584E4`, `teal` `#2190A4`, `green` `#3A944A`, `yellow` `#C88800`, `orange` `#ED5B00`, `red` `#E62D42`, `pink` `#D56199`, `purple` `#9141AC`, `slate` `#6F8396`).
- **Depois disso, e nas outras plataformas:** o destaque do sistema (`raw.system_accent`).

## Alto contraste

`raw.system_high_contrast`, ou no Linux:

- **KDE:** um esquema de cores do `kdeglobals` cujo nome contém "contrast".
- **Demais desktops:** `org.gnome.desktop.a11y.interface high-contrast`, ou um tema GTK cujo nome contém "highcontrast".

## Esquema de cores

`raw.color_scheme` (o portal no Linux, a aparência do sistema nos demais). No Linux, quando é `None`:

| Desktop | Fallback |
|---|---|
| KDE | `Dark` quando o esquema de cores do `kdeglobals` (ou, sem ele, o pacote de look-and-feel) contém "dark" |
| COSMIC | a flag `is_dark` |
| Xfce, LXQt | nenhum: só o portal |
| Todos os outros desktops | `org.gnome.desktop.interface color-scheme`: `prefer-dark` ou `prefer-light` |
