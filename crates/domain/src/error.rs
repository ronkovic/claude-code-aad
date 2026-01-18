//! Domain error types.

use thiserror::Error;

/// Result type alias for domain operations.
pub type Result<T> = std::result::Result<T, DomainError>;

/// Domain-level errors.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    /// Entity or value object not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Validation failed.
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Repository operation failed.
    #[error("Repository error: {0}")]
    RepositoryError(String),

    /// General error.
    #[error("Error: {0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DomainError::NotFound("test".to_string());
        assert_eq!(err.to_string(), "Not found: test");

        let err = DomainError::ValidationError("invalid".to_string());
        assert_eq!(err.to_string(), "Validation error: invalid");

        let err = DomainError::RepositoryError("failed".to_string());
        assert_eq!(err.to_string(), "Repository error: failed");
    }

    #[test]
    fn test_error_equality() {
        let err1 = DomainError::NotFound("test".to_string());
        let err2 = DomainError::NotFound("test".to_string());
        assert_eq!(err1, err2);

        let err3 = DomainError::NotFound("other".to_string());
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_error_clone() {
        let err = DomainError::ValidationError("test".to_string());
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }
}
