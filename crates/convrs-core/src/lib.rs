//! hier befindet sich die core-bibliothek für die konvertierungslogik.

pub mod error;
pub mod format;
pub mod formats;

// re-exports für einfachen zugang
pub use error::FormatError;
pub use format::FileFormat;
