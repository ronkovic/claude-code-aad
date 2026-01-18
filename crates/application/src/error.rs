//! Application layer error types.

use thiserror::Error;

/// Application layer errors.
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// Domain error.
    #[error("Domain error: {0}")]
    Domain(#[from] domain::DomainError),

    /// Workflow transition error.
    #[error("Workflow transition error: {0}")]
    WorkflowTransition(String),

    /// Validation error.
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Application layer result type.
pub type Result<T> = std::result::Result<T, ApplicationError>;
