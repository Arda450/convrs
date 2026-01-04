pub mod error;
pub mod formats;
pub mod detect;
pub mod format;

// Re-exports für einfachen Zugriff
pub use error::FormatError;
pub use format::FileFormat;

#[cfg(feature = "cli")]
pub mod cli;