//! Type-safe ID value objects.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Specification ID.
///
/// A type-safe wrapper around a UUID to uniquely identify a specification.
///
/// # Examples
///
/// ```
/// use domain::value_objects::ids::SpecId;
///
/// let id = SpecId::new();
/// println!("Spec ID: {}", id);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpecId(String);

impl SpecId {
    /// Creates a new random SpecId.
    pub fn new() -> Self {
        Self(format!("SPEC-{}", Uuid::new_v4()))
    }

    /// Returns the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SpecId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SpecId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SpecId {
    type Err = crate::DomainError;

    /// Creates a SpecId from an existing string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is empty.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(crate::DomainError::ValidationError(
                "SpecId cannot be empty".to_string(),
            ));
        }
        Ok(Self(s.to_string()))
    }
}

/// Task ID.
///
/// A type-safe wrapper around a UUID to uniquely identify a task.
///
/// # Examples
///
/// ```
/// use domain::value_objects::ids::TaskId;
///
/// let id = TaskId::new();
/// println!("Task ID: {}", id);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(String);

impl TaskId {
    /// Creates a new random TaskId.
    pub fn new() -> Self {
        Self(format!("TASK-{}", Uuid::new_v4()))
    }

    /// Returns the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TaskId {
    type Err = crate::DomainError;

    /// Creates a TaskId from an existing string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is empty.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(crate::DomainError::ValidationError(
                "TaskId cannot be empty".to_string(),
            ));
        }
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_id_creation() {
        let id = SpecId::new();
        assert!(id.as_str().starts_with("SPEC-"));
    }

    #[test]
    fn test_spec_id_from_str() {
        let id = SpecId::from_str("SPEC-001").unwrap();
        assert_eq!(id.as_str(), "SPEC-001");

        let empty_result = SpecId::from_str("");
        assert!(empty_result.is_err());
    }

    #[test]
    fn test_spec_id_equality() {
        let id1 = SpecId::from_str("SPEC-001").unwrap();
        let id2 = SpecId::from_str("SPEC-001").unwrap();
        let id3 = SpecId::from_str("SPEC-002").unwrap();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_spec_id_display() {
        let id = SpecId::from_str("SPEC-001").unwrap();
        assert_eq!(format!("{}", id), "SPEC-001");
    }

    #[test]
    fn test_spec_id_clone() {
        let id = SpecId::new();
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_spec_id_default() {
        let id = SpecId::default();
        assert!(id.as_str().starts_with("SPEC-"));
    }

    #[test]
    fn test_task_id_creation() {
        let id = TaskId::new();
        assert!(id.as_str().starts_with("TASK-"));
    }

    #[test]
    fn test_task_id_from_str() {
        let id = TaskId::from_str("TASK-001").unwrap();
        assert_eq!(id.as_str(), "TASK-001");

        let empty_result = TaskId::from_str("");
        assert!(empty_result.is_err());
    }

    #[test]
    fn test_task_id_equality() {
        let id1 = TaskId::from_str("TASK-001").unwrap();
        let id2 = TaskId::from_str("TASK-001").unwrap();
        let id3 = TaskId::from_str("TASK-002").unwrap();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_task_id_display() {
        let id = TaskId::from_str("TASK-001").unwrap();
        assert_eq!(format!("{}", id), "TASK-001");
    }

    #[test]
    fn test_task_id_clone() {
        let id = TaskId::new();
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_task_id_default() {
        let id = TaskId::default();
        assert!(id.as_str().starts_with("TASK-"));
    }
}
