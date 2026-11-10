🌐 [Português](../../pt-br/architecture/watchers.md) | **English** | 🏠 [Index](../index.md)

---

# Watchers

`watch(on_change)` (feature `watch`) returns a `SettingsWatcher` built by `SettingsWatcher::start` in `crates/oito2_settings/src/watcher.rs`, with a platform part in `{linux,windows,macos}/watch.rs`.

## Start

1. The settings are read once, synchronously, as the reference.
2. The **emitter thread** (`oito2-settings-emit`) starts.
3. The **platform thread** (`oito2-settings-watch`) starts and attaches its sources; `watch` returns only after it reports that they are attached, so no change right after `watch` returns is lost.

## Change Events

The platform part only sends `Event::Changed` over a `std::sync::mpsc` channel:

| OS | Sources |
|---|---|
| Linux | `gio::Settings` `changed` for each installed schema of `GSETTINGS_KEYS` (each watched key is read once, since GSettings only reports keys that were read); `gio::FileMonitor` (`WATCH_MOVES`) on each existing configuration directory, filtered to the watched file names, which catches writes in place and atomic renames; the portal's `SettingChanged` signal for `org.freedesktop.appearance`. All attached to a `glib::MainContext` the thread owns (`with_thread_default`) and runs with a `MainLoop`. |
| Windows | `UISettings.ColorValuesChanged` and `AccessibilitySettings.HighContrastChanged` handlers; the thread keeps the registrations alive, and the Windows Runtime calls the handlers on its own threads. |
| macOS | Block observers for `NSSystemColorsDidChangeNotification` (default center), `AppleInterfaceThemeChangedNotification` (distributed center) and `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification` (workspace center). Blocks without a queue run on the posting thread, so the notifications arrive through the application's main run loop. |

## Emitter

After an event, the emitter waits until no event arrives for 150 ms (`recv_timeout`), reads the settings, and calls `on_change` only when they differ from the last value it saw. It stops on `Event::Stop` or when every sender is gone.

## Stop

`Drop for SettingsWatcher`:

1. Stops the platform part — on Linux `MainContext::invoke(|| main_loop.quit())`, which runs once the loop iterates, so a stop requested before the loop starts isn't lost; on Windows and macOS a message on a channel — and joins its thread, which removes the handlers or observers.
2. Sends `Event::Stop` and joins the emitter.

A join is skipped when the current thread is the one to join, so dropping the watcher inside its own callback doesn't deadlock. `SettingsWatcher` is `Send + Sync`, so it can be stored in shared application state.

## Known Limits

- On Linux, configuration directories created after `watch` starts are not monitored.
- On macOS, nothing is reported without a running main run loop (a plain command-line program).
