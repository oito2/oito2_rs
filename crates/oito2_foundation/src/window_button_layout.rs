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

//! The window control buttons and their order in a title bar.

use core::fmt;

use crate::WindowControlStyle;

/// A button that controls the native window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum WindowButton {
    /// Minimizes the window.
    Minimize,
    /// Maximizes or restores the window.
    Maximize,
    /// Closes the window.
    Close,
}

impl WindowButton {
    /// The lowercase name: `minimize`, `maximize` or `close`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Minimize => "minimize",
            Self::Maximize => "maximize",
            Self::Close => "close",
        }
    }
}

impl fmt::Display for WindowButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The order and side of the window control buttons in a title bar.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default, rename_all = "camelCase"))]
pub struct WindowButtonLayout {
    /// The buttons placed before the title, in display order.
    pub leading: Vec<WindowButton>,
    /// The buttons placed after the title, in display order.
    pub trailing: Vec<WindowButton>,
}

/// Collects the buttons of one side: maps each token with `button`, skipping
/// unknown tokens and buttons already placed (recorded in `seen`).
fn side<T>(
    tokens: impl IntoIterator<Item = T>,
    button: impl Fn(T) -> Option<WindowButton>,
    seen: &mut Vec<WindowButton>,
) -> Vec<WindowButton> {
    let mut buttons = Vec::new();
    for token in tokens {
        if let Some(button) = button(token)
            && !seen.contains(&button)
        {
            seen.push(button);
            buttons.push(button);
        }
    }
    buttons
}

/// Maps a GNOME `button-layout` entry to its button.
fn gnome_name(name: &str) -> Option<WindowButton> {
    match name.trim() {
        "minimize" => Some(WindowButton::Minimize),
        "maximize" => Some(WindowButton::Maximize),
        "close" => Some(WindowButton::Close),
        _ => None,
    }
}

/// Maps a KWin `ButtonsOnLeft`/`ButtonsOnRight` letter to its button.
fn kde_letter(letter: char) -> Option<WindowButton> {
    match letter {
        'I' => Some(WindowButton::Minimize),
        'A' => Some(WindowButton::Maximize),
        'X' => Some(WindowButton::Close),
        _ => None,
    }
}

/// Maps an xfwm4 `button_layout` letter to its button.
fn xfwm_letter(letter: char) -> Option<WindowButton> {
    match letter {
        'H' => Some(WindowButton::Minimize),
        'M' => Some(WindowButton::Maximize),
        'C' => Some(WindowButton::Close),
        _ => None,
    }
}

impl WindowButtonLayout {
    /// A layout with `leading` buttons before the title and `trailing`
    /// buttons after it.
    pub fn new(leading: Vec<WindowButton>, trailing: Vec<WindowButton>) -> Self {
        Self { leading, trailing }
    }

    /// Parses the GNOME `button-layout` format used by GNOME, Budgie,
    /// Cinnamon, MATE and Pantheon, such as `appmenu:minimize,maximize,close`.
    ///
    /// Names before the first colon are leading, names between it and the
    /// next colon trailing; anything after a second colon is ignored. Entries
    /// other than `minimize`, `maximize` and `close` are ignored, and each
    /// button is kept only at its first position.
    pub fn parse_gnome(value: &str) -> Self {
        let mut sides = value.split(':');
        let left = sides.next().unwrap_or_default();
        let right = sides.next();
        let mut seen = Vec::new();
        let leading = side(left.split(','), gnome_name, &mut seen);
        let trailing = right
            .map(|right| side(right.split(','), gnome_name, &mut seen))
            .unwrap_or_default();
        Self { leading, trailing }
    }

    /// Parses the KDE KWin `ButtonsOnLeft` and `ButtonsOnRight` values, such
    /// as `MS` and `HIAX`.
    ///
    /// Recognizes `I` (minimize), `A` (maximize) and `X` (close); other
    /// letters are ignored, and each button is kept only at its first
    /// position.
    pub fn parse_kde(buttons_on_left: &str, buttons_on_right: &str) -> Self {
        let mut seen = Vec::new();
        let leading = side(buttons_on_left.chars(), kde_letter, &mut seen);
        let trailing = side(buttons_on_right.chars(), kde_letter, &mut seen);
        Self { leading, trailing }
    }

    /// Parses the Xfce xfwm4 `button_layout` format, such as `O|HMC`.
    ///
    /// Letters before the first `|` title marker are leading, letters between
    /// it and the next marker trailing. Recognizes `H` (minimize), `M` (maximize) and `C` (close);
    /// other letters are ignored, and each button is kept only at its first
    /// position.
    pub fn parse_xfwm(value: &str) -> Self {
        let mut sides = value.split('|');
        let left = sides.next().unwrap_or_default();
        let right = sides.next();
        let mut seen = Vec::new();
        let leading = side(left.chars(), xfwm_letter, &mut seen);
        let trailing = right
            .map(|right| side(right.chars(), xfwm_letter, &mut seen))
            .unwrap_or_default();
        Self { leading, trailing }
    }

    /// The default layout for `style`: leading close, minimize and maximize
    /// for [`WindowControlStyle::Macos`], no buttons for
    /// [`WindowControlStyle::None`], and trailing minimize, maximize and
    /// close otherwise.
    pub fn default_for(style: WindowControlStyle) -> Self {
        match style {
            WindowControlStyle::Macos => Self::new(
                vec![
                    WindowButton::Close,
                    WindowButton::Minimize,
                    WindowButton::Maximize,
                ],
                Vec::new(),
            ),
            WindowControlStyle::None => Self::default(),
            _ => Self::new(
                Vec::new(),
                vec![
                    WindowButton::Minimize,
                    WindowButton::Maximize,
                    WindowButton::Close,
                ],
            ),
        }
    }

    /// Whether the layout has no buttons at all.
    pub fn is_empty(&self) -> bool {
        self.leading.is_empty() && self.trailing.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_each_button_at_its_first_position() {
        let layout = WindowButtonLayout::parse_gnome("close,close:close,minimize");
        assert_eq!(layout.leading, [WindowButton::Close]);
        assert_eq!(layout.trailing, [WindowButton::Minimize]);
        let layout = WindowButtonLayout::parse_kde("XX", "XIA");
        assert_eq!(layout.leading, [WindowButton::Close]);
        assert_eq!(
            layout.trailing,
            [WindowButton::Minimize, WindowButton::Maximize]
        );
    }

    #[test]
    fn a_missing_separator_means_leading_only() {
        assert_eq!(
            WindowButtonLayout::parse_gnome("close").leading,
            [WindowButton::Close]
        );
        assert!(WindowButtonLayout::parse_gnome("close").trailing.is_empty());
        assert_eq!(
            WindowButtonLayout::parse_xfwm("HMC").leading,
            [
                WindowButton::Minimize,
                WindowButton::Maximize,
                WindowButton::Close
            ]
        );
    }

    #[test]
    fn empty_layouts() {
        assert!(WindowButtonLayout::default().is_empty());
        assert!(WindowButtonLayout::parse_gnome(":").is_empty());
        assert!(WindowButtonLayout::parse_xfwm("|").is_empty());
        assert!(!WindowButtonLayout::default_for(WindowControlStyle::Gnome).is_empty());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serializes_as_leading_and_trailing() {
        let layout = WindowButtonLayout::default_for(WindowControlStyle::Macos);
        let json = serde_json::to_value(&layout).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "leading": ["close", "minimize", "maximize"], "trailing": [] })
        );
        let back: WindowButtonLayout = serde_json::from_value(json).unwrap();
        assert_eq!(back, layout);
        let partial: WindowButtonLayout =
            serde_json::from_str(r#"{"trailing":["close"]}"#).unwrap();
        assert_eq!(partial.trailing, [WindowButton::Close]);
    }
}
