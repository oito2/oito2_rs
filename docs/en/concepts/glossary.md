🌐 [Português](../../pt-br/concepts/glossary.md) | **English** | 🏠 [Index](../index.md)

---

# Glossary

| Term | Meaning |
|---|---|
| **Accent color** | The highlight color the user picks in the system settings, reported as `#RRGGBB`. |
| **Button layout** | Which window buttons (minimize, maximize, close) sit before and after the title: `WindowButtonLayout { leading, trailing }`. |
| **Color scheme** | The light or dark preference of the system: `ColorScheme::Light`, `ColorScheme::Dark`, or `None` when the system states no preference. |
| **Compose** | `SystemSettings::compose`: turning a `RawSettings` into the `SystemSettings` of a platform. |
| **Debounce** | Waiting until changes stop arriving (150 ms) before reading again, so a burst of changes produces one callback. |
| **Decoration mode** | How a window frame is drawn: `native`, `undecorated` or `gtk-headerless`. |
| **Desktop environment** | The Linux desktop the session runs (GNOME, KDE Plasma, Xfce, ...), detected from `XDG_CURRENT_DESKTOP` and `DESKTOP_SESSION`. |
| **Emitter** | The watcher thread that debounces, reads, deduplicates and calls the callback. |
| **Fallback** | `SystemSettings::fallback`: the settings of a platform with nothing read. |
| **GSettings** | The GNOME settings store (usually backed by dconf), read through GIO. |
| **Main run loop** | The event loop of a macOS application's main thread, which delivers system notifications. |
| **Parity fixture** | `foundation.json`, exported by oito2_ui with the expected result of every rule case. |
| **Raw settings** | `RawSettings`: the values as the system stores them, before any rule. |
| **Settings portal** | `org.freedesktop.portal.Settings`, the XDG desktop portal interface that exposes `org.freedesktop.appearance` keys over D-Bus. |
| **System settings** | `SystemSettings`: what an app follows — platform, desktop, control style, button layout, accent, color scheme, high contrast. |
| **Window control style** | The visual family of the window buttons: `gnome`, `breeze`, `xfwm`, `cosmic`, `windows`, `macos` or `none`. |
