//! Phase enumeration for workflow stages.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Development workflow phase.
///
/// Represents the different stages in the AI-driven development lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Phase {
    /// Specification phase: defining requirements and design.
    Spec,
    /// Task decomposition phase: breaking down work into tasks.
    Tasks,
    /// Test-driven development phase: implementing features.
    Tdd,
    /// Review phase: code review and quality checks.
    Review,
    /// Retrospective phase: learning and improvement.
    Retro,
    /// Merge phase: integrating changes.
    Merge,
}

impl Phase {
    /// Returns all phases in order.
    pub fn all() -> Vec<Phase> {
        vec![
            Phase::Spec,
            Phase::Tasks,
            Phase::Tdd,
            Phase::Review,
            Phase::Retro,
            Phase::Merge,
        ]
    }

    /// Returns the next phase, if any.
    pub fn next(&self) -> Option<Phase> {
        match self {
            Phase::Spec => Some(Phase::Tasks),
            Phase::Tasks => Some(Phase::Tdd),
            Phase::Tdd => Some(Phase::Review),
            Phase::Review => Some(Phase::Retro),
            Phase::Retro => Some(Phase::Merge),
            Phase::Merge => None,
        }
    }

    /// Returns the Japanese display name.
    pub fn japanese_name(&self) -> &str {
        match self {
            Phase::Spec => "仕様",
            Phase::Tasks => "タスク分割",
            Phase::Tdd => "開発",
            Phase::Review => "レビュー",
            Phase::Retro => "振り返り",
            Phase::Merge => "統合",
        }
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Phase::Spec => "SPEC",
            Phase::Tasks => "TASKS",
            Phase::Tdd => "TDD",
            Phase::Review => "REVIEW",
            Phase::Retro => "RETRO",
            Phase::Merge => "MERGE",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Phase {
    type Err = crate::DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SPEC" => Ok(Phase::Spec),
            "TASKS" => Ok(Phase::Tasks),
            "TDD" => Ok(Phase::Tdd),
            "REVIEW" => Ok(Phase::Review),
            "RETRO" => Ok(Phase::Retro),
            "MERGE" => Ok(Phase::Merge),
            _ => Err(crate::DomainError::ValidationError(format!(
                "Invalid phase: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_display() {
        assert_eq!(format!("{}", Phase::Spec), "SPEC");
        assert_eq!(format!("{}", Phase::Tasks), "TASKS");
        assert_eq!(format!("{}", Phase::Tdd), "TDD");
        assert_eq!(format!("{}", Phase::Review), "REVIEW");
        assert_eq!(format!("{}", Phase::Retro), "RETRO");
        assert_eq!(format!("{}", Phase::Merge), "MERGE");
    }

    #[test]
    fn test_phase_from_str() {
        assert_eq!(Phase::from_str("SPEC").unwrap(), Phase::Spec);
        assert_eq!(Phase::from_str("spec").unwrap(), Phase::Spec);
        assert_eq!(Phase::from_str("TASKS").unwrap(), Phase::Tasks);
        assert_eq!(Phase::from_str("TDD").unwrap(), Phase::Tdd);
        assert_eq!(Phase::from_str("REVIEW").unwrap(), Phase::Review);
        assert_eq!(Phase::from_str("RETRO").unwrap(), Phase::Retro);
        assert_eq!(Phase::from_str("MERGE").unwrap(), Phase::Merge);

        assert!(Phase::from_str("INVALID").is_err());
    }

    #[test]
    fn test_phase_japanese_name() {
        assert_eq!(Phase::Spec.japanese_name(), "仕様");
        assert_eq!(Phase::Tasks.japanese_name(), "タスク分割");
        assert_eq!(Phase::Tdd.japanese_name(), "開発");
    }

    #[test]
    fn test_phase_next() {
        assert_eq!(Phase::Spec.next(), Some(Phase::Tasks));
        assert_eq!(Phase::Tasks.next(), Some(Phase::Tdd));
        assert_eq!(Phase::Tdd.next(), Some(Phase::Review));
        assert_eq!(Phase::Review.next(), Some(Phase::Retro));
        assert_eq!(Phase::Retro.next(), Some(Phase::Merge));
        assert_eq!(Phase::Merge.next(), None);
    }

    #[test]
    fn test_phase_all() {
        let phases = Phase::all();
        assert_eq!(phases.len(), 6);
        assert_eq!(phases[0], Phase::Spec);
        assert_eq!(phases[5], Phase::Merge);
    }

    #[test]
    fn test_phase_equality() {
        assert_eq!(Phase::Spec, Phase::Spec);
        assert_ne!(Phase::Spec, Phase::Tasks);
    }

    #[test]
    fn test_phase_clone() {
        let phase = Phase::Spec;
        let cloned = phase;
        assert_eq!(phase, cloned);
    }
}
