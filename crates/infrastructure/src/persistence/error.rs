//! Persistence layer error types.

use thiserror::Error;

/// Errors that can occur during persistence operations.
#[derive(Debug, Error)]
pub enum PersistenceError {
    /// File not found error.
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Deserialization error.
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Path traversal attempt detected.
    #[error("Path traversal attempt detected: {0}")]
    PathTraversalError(String),

    /// Invalid file name.
    #[error("Invalid file name: {0}")]
    InvalidFileName(String),

    /// Backup error.
    #[error("Backup error: {0}")]
    BackupError(String),

    /// Token replacement error.
    #[error("Token replacement error: {0}")]
    TokenReplacementError(String),

    /// Domain error wrapper.
    #[error("Domain error: {0}")]
    DomainError(#[from] domain::DomainError),
}

/// Result type for persistence operations.
pub type Result<T> = std::result::Result<T, PersistenceError>;
