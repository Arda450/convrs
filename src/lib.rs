pub mod error;
pub mod formats;
pub mod detect;

// Re-exports für einfachen Zugriff
pub use error::FormatError;

#[cfg(feature = "web")]
pub mod ui;

#[cfg(feature = "cli")]
pub mod cli;