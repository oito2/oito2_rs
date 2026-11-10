🌐 [Português](../../pt-br/troubleshooting/common-issues.md) | **English** | 🏠 [Index](../index.md)

---

# Common Issues

## Build: "The system library `glib-2.0` required by crate `glib-sys` was not found"

The GLib development files or `pkg-config` are missing. Install them (see [Installation](../getting-started/installation.md#linux-build-dependencies)): `sudo apt install libglib2.0-dev pkg-config` on Debian/Ubuntu, `sudo dnf install glib2-devel pkgconf-pkg-config` on Fedora.

## Linux: the accent or color scheme is always `None`

- The portal values need a D-Bus session bus and a running `xdg-desktop-portal` with a backend for the desktop (`xdg-desktop-portal-gnome`, `-kde`, `-gtk`, ...). Without them the portal keys stay unset.
- `color-scheme` 0 means "no preference": the composition then uses the desktop's own store (see [Composition rules](../architecture/composition.md#color-scheme)). On Xfce and LXQt the portal is the only source.
- An accent is reported only when the desktop has one: GNOME 47+ (`accent-color`), KDE (`AccentColor`) or a portal that implements `accent-color`.

## Linux: values missing inside Flatpak

Inside the sandbox, GSettings and the configuration files of the host are not visible; only the portal values are read.

## Linux: a change isn't reported

The watcher monitors the configuration directories that exist when it starts. A settings directory created later (for example `~/.config/cosmic/com.system76.CosmicTheme.Mode/v1` on a fresh account) is monitored only after the watcher is created again.

## macOS: `watch` reports nothing

The notifications arrive through the application's main run loop. A command-line program (such as the `print_settings` example) doesn't run one; an AppKit application — winit, Iced or Tauri — does.

## `watch` returns `Error::Unsupported`

The running platform is not Linux, Windows or macOS.

## Tests: "fuse init failed" and a slow test under `dbus-run-session`

The message comes from `xdg-document-portal` being started on the isolated session bus, where it can't mount its FUSE file system; the read test waits for the portal services on that bus. Neither affects the result.
