// Gemeinsamer Fehler-Typen und Fehlerbehandlung hier implementieren

// dynmaisch generischer Fehler-Typ
#[derive(Debug)] // ermöglicht das Debugging des Fehlers
pub enum FormatError { // pub macht es für anderen modulen benutzbar
     IoError(String),
     ParseError(String),
     SerializationError(String), // Umbenannt von FormatError, um Namenskonflikt zu vermeiden
     InvalidFormat(String), // Für ungültige/nicht unterstützte Formate
     UnknownError(String),
}

// implementiert Display für FormatError, damit der Fehler als String ausgegeben werden kann
impl std::fmt::Display for FormatError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { // rückgabe ist ein std::fmt::Result, also Ok(()) oder Err(e)
      match self { // pattern matching, um den Fehler zu handeln
          FormatError::IoError(msg) => write!(f, "IO Error: {}", msg), // write! ist eine makro, die eine string in die formatierte ausgabe schreibt
          FormatError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
          FormatError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
          FormatError::InvalidFormat(msg) => write!(f, "Invalid Format: {}", msg),
          FormatError::UnknownError(msg) => write!(f, "Unknown Error: {}", msg), // Fallback für unbekannte Fehler
      }
  } // end of match
} // end of impl

impl std::error::Error for FormatError {} // implementiert Error für FormatError, damit der Fehler als Error ausgegeben werden kann