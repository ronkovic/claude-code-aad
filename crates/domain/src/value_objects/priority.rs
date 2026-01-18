//! Priority enumeration using MoSCoW method.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Task priority using MoSCoW method.
///
/// - Must: Critical features that must be delivered
/// - Should: Important features that should be included if possible
/// - Could: Desirable features that could be included if resources allow
/// - Wont: Features that won't be included this time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Priority {
    /// Must have - critical requirement.
    Must,
    /// Should have - important but not critical.
    Should,
    /// Could have - desirable if resources allow.
    Could,
    /// Won't have - explicitly out of scope.
    Wont,
}

impl Priority {
    /// Returns the Japanese display name.
    pub fn japanese_name(&self) -> &str {
        match self {
            Priority::Must => "必須",
            Priority::Should => "重要",
            Priority::Could => "あれば良い",
            Priority::Wont => "対象外",
        }
    }

    /// Returns the priority value for ordering (higher is more important).
    fn priority_value(&self) -> u8 {
        match self {
            Priority::Must => 4,
            Priority::Should => 3,
            Priority::Could => 2,
            Priority::Wont => 1,
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Priority::Must => "Must",
            Priority::Should => "Should",
            Priority::Could => "Could",
            Priority::Wont => "Won't",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Priority {
    type Err = crate::DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "must" | "must have" => Ok(Priority::Must),
            "should" | "should have" => Ok(Priority::Should),
            "could" | "could have" => Ok(Priority::Could),
            "wont" | "won't" | "wont have" | "won't have" => Ok(Priority::Wont),
            _ => Err(crate::DomainError::ValidationError(format!(
                "Invalid priority: {}",
                s
            ))),
        }
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority_value().cmp(&other.priority_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_display() {
        assert_eq!(format!("{}", Priority::Must), "Must");
        assert_eq!(format!("{}", Priority::Should), "Should");
        assert_eq!(format!("{}", Priority::Could), "Could");
        assert_eq!(format!("{}", Priority::Wont), "Won't");
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!(Priority::from_str("Must").unwrap(), Priority::Must);
        assert_eq!(Priority::from_str("must").unwrap(), Priority::Must);
        assert_eq!(Priority::from_str("must have").unwrap(), Priority::Must);
        assert_eq!(Priority::from_str("Should").unwrap(), Priority::Should);
        assert_eq!(Priority::from_str("Could").unwrap(), Priority::Could);
        assert_eq!(Priority::from_str("Wont").unwrap(), Priority::Wont);
        assert_eq!(Priority::from_str("won't").unwrap(), Priority::Wont);

        assert!(Priority::from_str("INVALID").is_err());
    }

    #[test]
    fn test_priority_ordering() {
        // Must > Should > Could > Wont
        assert!(Priority::Must > Priority::Should);
        assert!(Priority::Should > Priority::Could);
        assert!(Priority::Could > Priority::Wont);
        assert!(Priority::Must > Priority::Wont);

        // Equality
        assert_eq!(Priority::Must.cmp(&Priority::Must), Ordering::Equal);
    }

    #[test]
    fn test_priority_japanese_name() {
        assert_eq!(Priority::Must.japanese_name(), "必須");
        assert_eq!(Priority::Should.japanese_name(), "重要");
        assert_eq!(Priority::Could.japanese_name(), "あれば良い");
        assert_eq!(Priority::Wont.japanese_name(), "対象外");
    }

    #[test]
    fn test_priority_equality() {
        assert_eq!(Priority::Must, Priority::Must);
        assert_ne!(Priority::Must, Priority::Should);
    }

    #[test]
    fn test_priority_clone() {
        let priority = Priority::Must;
        let cloned = priority;
        assert_eq!(priority, cloned);
    }

    #[test]
    fn test_priority_sorting() {
        let mut priorities = vec![
            Priority::Could,
            Priority::Must,
            Priority::Wont,
            Priority::Should,
        ];
        priorities.sort();

        assert_eq!(
            priorities,
            vec![
                Priority::Wont,
                Priority::Could,
                Priority::Should,
                Priority::Must
            ]
        );
    }
}
