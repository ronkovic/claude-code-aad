//! Configuration validation logic.

use crate::error::{InfrastructureError, Result};

/// Validation trait for configuration types.
pub trait Validate {
    /// Validates the configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    fn validate(&self) -> Result<()>;
}

/// Validation error with user-friendly Japanese messages.
#[derive(Debug)]
pub enum ValidationError {
    /// Required field is missing.
    MissingField { field: String },

    /// Value is out of valid range.
    OutOfRange {
        field: String,
        value: i64,
        min: i64,
        max: i64,
    },

    /// Path does not exist.
    PathNotFound { path: String },

    /// File read error.
    FileReadError { file: String, reason: String },

    /// Custom validation error.
    Custom(String),
}

impl ValidationError {
    /// Converts the validation error to a user-friendly Japanese error message.
    pub fn to_japanese_message(&self) -> String {
        match self {
            ValidationError::MissingField { field } => {
                format!("必須フィールド '{}' が設定されていません", field)
            }
            ValidationError::OutOfRange {
                field,
                value,
                min,
                max,
            } => {
                format!(
                    "'{}' の値 {} は範囲外です（{}〜{}）",
                    field, value, min, max
                )
            }
            ValidationError::PathNotFound { path } => {
                format!("パス '{}' が見つかりません", path)
            }
            ValidationError::FileReadError { file, reason } => {
                format!(
                    "設定ファイル '{}' の読み込みに失敗しました: {}",
                    file, reason
                )
            }
            ValidationError::Custom(msg) => msg.clone(),
        }
    }
}

impl From<ValidationError> for InfrastructureError {
    fn from(err: ValidationError) -> Self {
        InfrastructureError::Validation(err.to_japanese_message())
    }
}

/// Validates that a required field is present.
pub fn validate_required<T>(field_name: &str, value: &Option<T>) -> Result<()> {
    if value.is_none() {
        return Err(ValidationError::MissingField {
            field: field_name.to_string(),
        }
        .into());
    }
    Ok(())
}

/// Validates that a value is within a specified range.
pub fn validate_range(field_name: &str, value: i64, min: i64, max: i64) -> Result<()> {
    if value < min || value > max {
        return Err(ValidationError::OutOfRange {
            field: field_name.to_string(),
            value,
            min,
            max,
        }
        .into());
    }
    Ok(())
}

/// Validates that a path exists.
pub fn validate_path_exists(path: &std::path::Path) -> Result<()> {
    if !path.exists() {
        return Err(ValidationError::PathNotFound {
            path: path.display().to_string(),
        }
        .into());
    }
    Ok(())
}

/// Validates that a string is not empty after trimming.
pub fn validate_not_empty(field_name: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(ValidationError::MissingField {
            field: field_name.to_string(),
        }
        .into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_validate_required_present() {
        let value = Some("test");
        assert!(validate_required("field", &value).is_ok());
    }

    #[test]
    fn test_validate_required_missing() {
        let value: Option<String> = None;
        let result = validate_required("field", &value);
        assert!(result.is_err());

        if let Err(InfrastructureError::Validation(msg)) = result {
            assert!(msg.contains("必須フィールド"));
            assert!(msg.contains("field"));
        } else {
            panic!("Expected validation error");
        }
    }

    #[test]
    fn test_validate_range_valid() {
        assert!(validate_range("threshold", 50, 0, 100).is_ok());
        assert!(validate_range("threshold", 0, 0, 100).is_ok());
        assert!(validate_range("threshold", 100, 0, 100).is_ok());
    }

    #[test]
    fn test_validate_range_too_low() {
        let result = validate_range("threshold", -1, 0, 100);
        assert!(result.is_err());

        if let Err(InfrastructureError::Validation(msg)) = result {
            assert!(msg.contains("範囲外"));
            assert!(msg.contains("0〜100"));
        } else {
            panic!("Expected validation error");
        }
    }

    #[test]
    fn test_validate_range_too_high() {
        let result = validate_range("threshold", 101, 0, 100);
        assert!(result.is_err());

        if let Err(InfrastructureError::Validation(msg)) = result {
            assert!(msg.contains("範囲外"));
            assert!(msg.contains("101"));
        } else {
            panic!("Expected validation error");
        }
    }

    #[test]
    fn test_validate_path_exists_valid() {
        let path = Path::new(".");
        assert!(validate_path_exists(path).is_ok());
    }

    #[test]
    fn test_validate_path_exists_missing() {
        let path = Path::new("/nonexistent/path/to/file");
        let result = validate_path_exists(path);
        assert!(result.is_err());

        if let Err(InfrastructureError::Validation(msg)) = result {
            assert!(msg.contains("パス"));
            assert!(msg.contains("見つかりません"));
        } else {
            panic!("Expected validation error");
        }
    }

    #[test]
    fn test_validate_not_empty_valid() {
        assert!(validate_not_empty("name", "test").is_ok());
        assert!(validate_not_empty("name", "  test  ").is_ok());
    }

    #[test]
    fn test_validate_not_empty_invalid() {
        assert!(validate_not_empty("name", "").is_err());
        assert!(validate_not_empty("name", "   ").is_err());
    }

    #[test]
    fn test_validation_error_to_japanese_message() {
        let err = ValidationError::MissingField {
            field: "version".to_string(),
        };
        assert!(err.to_japanese_message().contains("必須フィールド"));
        assert!(err.to_japanese_message().contains("version"));

        let err = ValidationError::OutOfRange {
            field: "threshold".to_string(),
            value: 150,
            min: 0,
            max: 100,
        };
        assert!(err.to_japanese_message().contains("範囲外"));

        let err = ValidationError::PathNotFound {
            path: "/test/path".to_string(),
        };
        assert!(err.to_japanese_message().contains("見つかりません"));

        let err = ValidationError::FileReadError {
            file: "config.toml".to_string(),
            reason: "permission denied".to_string(),
        };
        assert!(err.to_japanese_message().contains("読み込みに失敗"));
    }
}
