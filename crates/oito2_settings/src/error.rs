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

//! The errors of this crate.

/// An error of the settings readers and watchers.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The requested operation is not supported on this platform.
    #[error("unsupported: {0}")]
    Unsupported(String),
    /// An internal failure, such as a watcher thread that stopped.
    #[error("internal error: {0}")]
    Internal(String),
}

/// Serializes as `{"kind": "unsupported" | "internal", "message": "…"}`, the
/// shape the OITO2 Tauri plugins report to their webview.
#[cfg(feature = "serde")]
impl serde::Serialize for Error {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let kind = match self {
            Self::Unsupported(_) => "unsupported",
            Self::Internal(_) => "internal",
        };
        let mut error = serializer.serialize_struct("Error", 2)?;
        error.serialize_field("kind", kind)?;
        error.serialize_field("message", &self.to_string())?;
        error.end()
    }
}

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_carry_the_kind() {
        assert_eq!(
            Error::Unsupported("watching".into()).to_string(),
            "unsupported: watching"
        );
        assert_eq!(
            Error::Internal("thread stopped".into()).to_string(),
            "internal error: thread stopped"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serializes_as_kind_and_message() {
        let json = serde_json::to_value(Error::Unsupported("watching".into())).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "kind": "unsupported", "message": "unsupported: watching" })
        );
        let json = serde_json::to_value(Error::Internal("x".into())).unwrap();
        assert_eq!(json["kind"], "internal");
    }
}
