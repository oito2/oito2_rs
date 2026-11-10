🌐 [Português](../../pt-br/concepts/how-oito2_rs-works.md) | **English** | 🏠 [Index](../index.md)

---

# How oito2_rs Works

```text
system stores ──read──▶ RawSettings ──compose(platform)──▶ SystemSettings
      │                     ▲
      └──change──▶ watcher ─┘ (debounce, read again, deduplicate, callback)
```

## 1. Read: `RawSettings`

A reader collects the values as the system stores them, without deciding anything:

- **Linux** — `XDG_CURRENT_DESKTOP` and `DESKTOP_SESSION`; the GSettings keys of the GNOME, Cinnamon and MATE schemas; `kwinrc`, `kdeglobals`, `xfwm4.xml` and the COSMIC settings files under `XDG_CONFIG_HOME`; the `accent-color`, `contrast` and `color-scheme` keys of the settings portal.
- **Windows** — the accent and background colors of `UISettings`, and the high-contrast flag of `AccessibilitySettings`.
- **macOS** — `NSColor.controlAccentColor`, the "Increase contrast" preference and the `AppleInterfaceStyle` user default.

The result is a `RawSettings`. See [Settings sources](../reference/settings-sources.md) for every key.

## 2. Compose: `SystemSettings`

`SystemSettings::compose(&raw, platform)` applies the rules of oito2_ui:

1. On native Linux, the desktop environment is detected from the session variables.
2. The window control style follows the platform and the desktop (Breeze on KDE, Xfwm on Xfce, COSMIC on COSMIC, GNOME on the other Linux desktops, Windows and macOS styles on those systems, none on the web and on touch systems).
3. The button layout comes from the desktop's own store, or the style's default.
4. The accent, high contrast and color scheme come from the desktop's store first, then from the system value, with each desktop's fallbacks.

`SystemSettings::fallback(platform, desktop)` gives the same result with nothing read. See [Composition rules](../architecture/composition.md).

## 3. Watch

`watch(callback)` starts two threads:

- a **platform thread** that listens to the system's change notifications — GSettings signals, file monitors and the portal signal on Linux; `UISettings` and `AccessibilitySettings` events on Windows; notification observers on macOS — and only reports "something changed";
- an **emitter thread** that waits 150 ms without new reports, reads the settings again, compares them with the last ones and calls the callback only when they differ.

Dropping the returned `SettingsWatcher` stops both threads. See [Watchers](../architecture/watchers.md).

### Where the changes come from, per platform

| OS | Needs from the application |
|---|---|
| Linux | Nothing: the watcher runs its own GLib main context |
| Windows | Nothing: the Windows Runtime raises the events on its own threads |
| macOS | A running main run loop, which every AppKit application has |
