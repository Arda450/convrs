//! Zentrale Fehlertypen für alle Format-Konvertierungen.

/// Gemeinsamer Fehler-Typ für alle Konvertierungsoperationen.
#[derive(Debug)]
pub enum FormatError {
    /// Fehler beim Lesen/Schreiben von Dateien für CLI.
    IoError(String),
    /// Ungültige Syntax im Input (z.b. kaputtes JSON).
    ParseError(String),
    /// Fehler beim Serialisieren in das Zielformat.
    SerializationError(String),
    /// Unbekanntes oder nicht unterstütztes Format.
    InvalidFormat(String),
    /// Fallback für unerwartete Fehler.
    UnknownError(String),
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::IoError(msg) => write!(f, "IO Error: {}", msg),
            FormatError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            FormatError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            FormatError::InvalidFormat(msg) => write!(f, "Invalid Format: {}", msg),
            FormatError::UnknownError(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}

impl std::error::Error for FormatError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_io_error() {
        let err = FormatError::IoError("file not found".to_string());
        assert_eq!(err.to_string(), "IO Error: file not found");
    }

    #[test]
    fn test_display_parse_error() {
        let err = FormatError::ParseError("Invalid JSON".to_string());
        assert_eq!(err.to_string(), "Parse Error: Invalid JSON");
    }

    #[test]
    fn test_display_serialization_error() {
        let err = FormatError::SerializationError("TOML error".to_string());
        assert_eq!(err.to_string(), "Serialization Error: TOML error");
    }

    #[test]
    fn test_display_invalid_format() {
        let err = FormatError::InvalidFormat("xml".to_string());
        assert_eq!(err.to_string(), "Invalid Format: xml");
    }

    #[test]
    fn test_display_unknown_error() {
        let err = FormatError::UnknownError("unexpected".to_string());
        assert_eq!(err.to_string(), "Unknown Error: unexpected");
    }

    #[test]
    fn test_error_is_debug() {
        let err = FormatError::ParseError("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("ParseError"));
    }
}
