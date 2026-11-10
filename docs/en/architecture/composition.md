🌐 [Português](../../pt-br/architecture/composition.md) | **English** | 🏠 [Index](../index.md)

---

# Composition Rules

`SystemSettings::compose(&raw, platform)` in `crates/oito2_foundation/src/system_settings.rs` mirrors `Oito2SystemSettingsReader` of `oito2_ui_settings`. Its cases are tested in `crates/oito2_foundation/tests/compose.rs`.

## Desktop Environment

Only on native Linux (`platform.is_linux() && platform.is_desktop_native()`); `None` elsewhere. `DesktopEnvironment::detect` checks each colon-separated entry of `XDG_CURRENT_DESKTOP` in order, then `DESKTOP_SESSION`, ignoring case and surrounding spaces:

| Names | Environment |
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
| anything else | `Unknown` |

## Window Control Style

`WindowControlStyle::resolve(platform, desktop)`: `None` on the web and on touch systems; `Windows` on Windows; `Macos` on macOS; on Linux `Breeze` for KDE, `Xfwm` for Xfce, `Cosmic` for COSMIC and `Gnome` for every other desktop.

## Button Layout

| Desktop | Source | When missing |
|---|---|---|
| KDE | `ButtonsOnLeft`/`ButtonsOnRight` of `kwinrc` (`I` minimize, `A` maximize, `X` close) | `MS` / `HIAX` |
| Xfce | `button_layout` of `xfwm4.xml` (`H`, `M`, `C`; `\|` marks the title) | `O\|SHMC` |
| COSMIC | `show_minimize` and `show_maximize` flags; close always shown | the button is shown |
| Cinnamon | `org.cinnamon.desktop.wm.preferences button-layout` | style default |
| MATE | `org.mate.Marco.general button-layout` | style default |
| GNOME, Budgie, Pantheon, Unity, LXQt, unknown | `org.gnome.desktop.wm.preferences button-layout` | style default |

In every format, unknown entries are ignored, each button is kept only at its first position, and only the first two parts around the separator count (`minimize:maximize:close` gives trailing `[maximize]`). The style default is trailing minimize, maximize and close, except leading close, minimize and maximize for `macos` and no buttons for `none`.

## Accent Color

- **KDE:** `AccentColor` of `kdeglobals` (`r,g,b`, stored as `#RRGGBB`).
- **Other Linux desktops:** `org.gnome.desktop.interface accent-color`, when it is one of the nine palette names (`blue` `#3584E4`, `teal` `#2190A4`, `green` `#3A944A`, `yellow` `#C88800`, `orange` `#ED5B00`, `red` `#E62D42`, `pink` `#D56199`, `purple` `#9141AC`, `slate` `#6F8396`).
- **Then, and on other platforms:** the system accent (`raw.system_accent`).

## High Contrast

`raw.system_high_contrast`, or on Linux:

- **KDE:** a `kdeglobals` color scheme whose name contains "contrast".
- **Other desktops:** `org.gnome.desktop.a11y.interface high-contrast`, or a GTK theme whose name contains "highcontrast".

## Color Scheme

`raw.color_scheme` (the portal on Linux, the system appearance elsewhere). On Linux, when it is `None`:

| Desktop | Fallback |
|---|---|
| KDE | `Dark` when the `kdeglobals` color scheme (or, without one, the look-and-feel package) contains "dark" |
| COSMIC | the `is_dark` flag |
| Xfce, LXQt | none: portal only |
| Every other desktop | `org.gnome.desktop.interface color-scheme`: `prefer-dark` or `prefer-light` |
