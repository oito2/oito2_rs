// Copyright (C) 2026  OITO2
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Reads the desktop configuration files under `XDG_CONFIG_HOME`.

use std::path::{Path, PathBuf};

use oito2_foundation::raw::files;

use crate::ConfigValues;
use crate::parsers::{cosmic_bool, ini_value, kde_color, xfconf_property};

/// The largest configuration file read, in bytes; larger files are ignored.
const MAX_FILE_SIZE: u64 = 1024 * 1024;

/// Returns the configuration directory: `XDG_CONFIG_HOME` when it is set to
/// an absolute path, else `HOME/.config`, or `None` without either.
pub fn config_home(xdg_config_home: Option<&str>, home: Option<&str>) -> Option<PathBuf> {
    match (xdg_config_home, home) {
        (Some(xdg), _) if !xdg.is_empty() && Path::new(xdg).is_absolute() => {
            Some(PathBuf::from(xdg))
        }
        (_, Some(home)) if !home.is_empty() => Some(Path::new(home).join(".config")),
        _ => None,
    }
}

/// Reads `relative` under `home` as text, or `None` when it is missing,
/// unreadable or larger than 1 MiB.
fn read(home: &Path, relative: &str) -> Option<String> {
    let path = home.join(relative);
    let metadata = std::fs::metadata(&path).ok()?;
    if !metadata.is_file() || metadata.len() > MAX_FILE_SIZE {
        return None;
    }
    std::fs::read_to_string(path).ok()
}

/// Extracts the values the settings need from the KDE, Xfce and COSMIC files
/// under `home`; missing files leave their values empty.
pub fn read_config(home: &Path) -> ConfigValues {
    let kwinrc = read(home, files::KWINRC).unwrap_or_default();
    let kdeglobals = read(home, files::KDEGLOBALS).unwrap_or_default();
    let xfwm = read(home, files::XFWM).unwrap_or_default();
    let cosmic_flag = |relative| read(home, relative).and_then(|content| cosmic_bool(&content));
    let decoration = "org.kde.kdecoration2";
    ConfigValues {
        kwin_buttons_on_left: ini_value(&kwinrc, decoration, "ButtonsOnLeft"),
        kwin_buttons_on_right: ini_value(&kwinrc, decoration, "ButtonsOnRight"),
        kde_accent_color: ini_value(&kdeglobals, "General", "AccentColor")
            .and_then(|value| kde_color(&value)),
        kde_color_scheme: ini_value(&kdeglobals, "General", "ColorScheme"),
        kde_look_and_feel_package: ini_value(&kdeglobals, "KDE", "LookAndFeelPackage"),
        xfwm_button_layout: xfconf_property(&xfwm, "button_layout"),
        cosmic_show_minimize: cosmic_flag(files::COSMIC_MINIMIZE),
        cosmic_show_maximize: cosmic_flag(files::COSMIC_MAXIMIZE),
        cosmic_is_dark: cosmic_flag(files::COSMIC_DARK),
    }
}

#[cfg(test)]
/// The directories whose files hold settings, with the file names that matter
/// in each, in the order of [`files::ALL`].
pub fn watched_files(home: &Path) -> Vec<(PathBuf, Vec<String>)> {
    let mut by_directory: Vec<(PathBuf, Vec<String>)> = Vec::new();
    for relative in files::ALL {
        let path = home.join(relative);
        let (Some(directory), Some(name)) = (path.parent(), path.file_name()) else {
            continue;
        };
        let name = name.to_string_lossy().into_owned();
        match by_directory.iter_mut().find(|(dir, _)| dir == directory) {
            Some((_, names)) => names.push(name),
            None => by_directory.push((directory.to_path_buf(), vec![name])),
        }
    }
    by_directory
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates an empty temporary configuration directory for one test.
    fn temp_home(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("oito2-settings-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write(home: &Path, relative: &str, content: &str) {
        let path = home.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn resolves_the_config_home() {
        assert_eq!(
            config_home(Some("/x"), Some("/home/a")),
            Some(PathBuf::from("/x"))
        );
        assert_eq!(
            config_home(Some(""), Some("/home/a")),
            Some(PathBuf::from("/home/a/.config"))
        );
        assert_eq!(
            config_home(Some("relative/config"), Some("/home/a")),
            Some(PathBuf::from("/home/a/.config"))
        );
        assert_eq!(config_home(Some("relative/config"), None), None);
        assert_eq!(config_home(None, None), None);
    }

    #[test]
    fn reads_every_desktop_file() {
        let home = temp_home("read");
        write(
            &home,
            files::KWINRC,
            "[org.kde.kdecoration2]\nButtonsOnLeft=X\nButtonsOnRight=IA\n",
        );
        write(
            &home,
            files::KDEGLOBALS,
            "[General]\nAccentColor=61,174,233\nColorScheme=BreezeHighContrast\n\n[KDE]\nLookAndFeelPackage=org.kde.breezedark.desktop\n",
        );
        write(
            &home,
            files::XFWM,
            r#"<channel name="xfwm4"><property name="general" type="empty"><property name="button_layout" type="string" value="CMH|"/></property></channel>"#,
        );
        write(&home, files::COSMIC_MINIMIZE, "false");
        write(&home, files::COSMIC_MAXIMIZE, "true\n");
        write(&home, files::COSMIC_DARK, "true");
        let values = read_config(&home);
        assert_eq!(values.kwin_buttons_on_left.as_deref(), Some("X"));
        assert_eq!(values.kwin_buttons_on_right.as_deref(), Some("IA"));
        assert_eq!(values.kde_accent_color.as_deref(), Some("#3DAEE9"));
        assert_eq!(
            values.kde_color_scheme.as_deref(),
            Some("BreezeHighContrast")
        );
        assert_eq!(
            values.kde_look_and_feel_package.as_deref(),
            Some("org.kde.breezedark.desktop")
        );
        assert_eq!(values.xfwm_button_layout.as_deref(), Some("CMH|"));
        assert_eq!(values.cosmic_show_minimize, Some(false));
        assert_eq!(values.cosmic_show_maximize, Some(true));
        assert_eq!(values.cosmic_is_dark, Some(true));
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn leaves_missing_files_empty() {
        let home = temp_home("missing");
        assert_eq!(read_config(&home), ConfigValues::default());
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn groups_the_watched_files_by_directory() {
        let home = PathBuf::from("/c");
        let watched = watched_files(&home);
        assert_eq!(watched.len(), 4);
        assert_eq!(
            watched[0],
            (
                PathBuf::from("/c"),
                vec!["kwinrc".to_owned(), "kdeglobals".to_owned()]
            )
        );
        assert_eq!(watched[1].1, vec!["xfwm4.xml".to_owned()]);
        assert_eq!(
            watched[2].1,
            vec!["show_minimize".to_owned(), "show_maximize".to_owned()]
        );
        assert_eq!(
            watched[3],
            (
                PathBuf::from("/c/cosmic/com.system76.CosmicTheme.Mode/v1"),
                vec!["is_dark".to_owned()]
            )
        );
    }
}
