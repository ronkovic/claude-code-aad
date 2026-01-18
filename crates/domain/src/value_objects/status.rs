//! Status enumeration for tracking entity states.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Entity status.
///
/// Represents the current state of a specification, task, or other entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    /// Not yet started.
    Pending,
    /// Currently being worked on.
    InProgress,
    /// Successfully completed.
    Completed,
    /// Blocked or failed.
    Blocked,
}

impl Status {
    /// Returns the Japanese display name.
    pub fn japanese_name(&self) -> &str {
        match self {
            Status::Pending => "未着手",
            Status::InProgress => "進行中",
            Status::Completed => "完了",
            Status::Blocked => "ブロック中",
        }
    }

    /// Checks if the status is terminal (cannot transition further).
    pub fn is_terminal(&self) -> bool {
        matches!(self, Status::Completed)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Pending => "pending",
            Status::InProgress => "in_progress",
            Status::Completed => "completed",
            Status::Blocked => "blocked",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Status {
    type Err = crate::DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Status::Pending),
            "in_progress" | "in-progress" => Ok(Status::InProgress),
            "completed" | "done" => Ok(Status::Completed),
            "blocked" | "failed" => Ok(Status::Blocked),
            _ => Err(crate::DomainError::ValidationError(format!(
                "Invalid status: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", Status::Pending), "pending");
        assert_eq!(format!("{}", Status::InProgress), "in_progress");
        assert_eq!(format!("{}", Status::Completed), "completed");
        assert_eq!(format!("{}", Status::Blocked), "blocked");
    }

    #[test]
    fn test_status_from_str() {
        assert_eq!(Status::from_str("pending").unwrap(), Status::Pending);
        assert_eq!(Status::from_str("PENDING").unwrap(), Status::Pending);
        assert_eq!(Status::from_str("in_progress").unwrap(), Status::InProgress);
        assert_eq!(Status::from_str("in-progress").unwrap(), Status::InProgress);
        assert_eq!(Status::from_str("completed").unwrap(), Status::Completed);
        assert_eq!(Status::from_str("done").unwrap(), Status::Completed);
        assert_eq!(Status::from_str("blocked").unwrap(), Status::Blocked);
        assert_eq!(Status::from_str("failed").unwrap(), Status::Blocked);

        assert!(Status::from_str("INVALID").is_err());
    }

    #[test]
    fn test_status_japanese_name() {
        assert_eq!(Status::Pending.japanese_name(), "未着手");
        assert_eq!(Status::InProgress.japanese_name(), "進行中");
        assert_eq!(Status::Completed.japanese_name(), "完了");
        assert_eq!(Status::Blocked.japanese_name(), "ブロック中");
    }

    #[test]
    fn test_status_is_terminal() {
        assert!(!Status::Pending.is_terminal());
        assert!(!Status::InProgress.is_terminal());
        assert!(Status::Completed.is_terminal());
        assert!(!Status::Blocked.is_terminal());
    }

    #[test]
    fn test_status_equality() {
        assert_eq!(Status::Pending, Status::Pending);
        assert_ne!(Status::Pending, Status::InProgress);
    }

    #[test]
    fn test_status_clone() {
        let status = Status::Pending;
        let cloned = status;
        assert_eq!(status, cloned);
    }
}
