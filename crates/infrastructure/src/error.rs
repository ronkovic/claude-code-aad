//! Infrastructure layer error types.

use thiserror::Error;

/// Infrastructure layer errors.
#[derive(Debug, Error)]
pub enum InfrastructureError {
    /// Domain error.
    #[error("Domain error: {0}")]
    Domain(#[from] domain::DomainError),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// File I/O error.
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parsing error.
    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// Validation error.
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Infrastructure layer result type.
pub type Result<T> = std::result::Result<T, InfrastructureError>;
