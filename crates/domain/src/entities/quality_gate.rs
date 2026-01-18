//! Quality gate entity for validating phase transitions.

use crate::value_objects::Phase;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Result of a quality check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    /// Check passed.
    Passed,
    /// Check failed with reason.
    Failed(String),
}

/// Individual quality check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCheck {
    /// Name of the check.
    pub name: String,
    /// Description of what this check validates.
    pub description: String,
    /// Result of the check.
    pub status: CheckStatus,
    /// When this check was performed.
    pub checked_at: DateTime<Utc>,
}

impl QualityCheck {
    /// Creates a new quality check result.
    pub fn new(name: String, description: String, status: CheckStatus) -> Self {
        Self {
            name,
            description,
            status,
            checked_at: Utc::now(),
        }
    }

    /// Returns true if this check passed.
    pub fn is_passed(&self) -> bool {
        matches!(self.status, CheckStatus::Passed)
    }
}

/// Quality gate for a specific phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    /// The phase this gate validates.
    pub phase: Phase,
    /// List of checks performed.
    pub checks: Vec<QualityCheck>,
    /// Overall gate status.
    pub passed: bool,
    /// Human approval status.
    pub approved: bool,
    /// Who approved (if any).
    pub approved_by: Option<String>,
    /// When the gate was evaluated.
    pub evaluated_at: DateTime<Utc>,
}

impl QualityGate {
    /// Creates a new quality gate for a phase.
    pub fn new(phase: Phase, checks: Vec<QualityCheck>) -> Self {
        let passed = checks.iter().all(|c| c.is_passed());
        Self {
            phase,
            checks,
            passed,
            approved: false,
            approved_by: None,
            evaluated_at: Utc::now(),
        }
    }

    /// Records human approval.
    pub fn approve(&mut self, approved_by: String) {
        self.approved = true;
        self.approved_by = Some(approved_by);
    }

    /// Returns true if the gate can be passed (all checks passed and approved).
    pub fn can_proceed(&self) -> bool {
        self.passed && self.approved
    }

    /// Returns failed checks.
    pub fn failed_checks(&self) -> Vec<&QualityCheck> {
        self.checks.iter().filter(|c| !c.is_passed()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_check_creation() {
        let check = QualityCheck::new(
            "Test Check".to_string(),
            "Test description".to_string(),
            CheckStatus::Passed,
        );

        assert_eq!(check.name, "Test Check");
        assert_eq!(check.description, "Test description");
        assert!(check.is_passed());
    }

    #[test]
    fn test_quality_check_is_passed() {
        let passed = QualityCheck::new(
            "Pass".to_string(),
            "".to_string(),
            CheckStatus::Passed,
        );
        assert!(passed.is_passed());

        let failed = QualityCheck::new(
            "Fail".to_string(),
            "".to_string(),
            CheckStatus::Failed("reason".to_string()),
        );
        assert!(!failed.is_passed());
    }

    #[test]
    fn test_quality_gate_all_checks_pass() {
        let checks = vec![
            QualityCheck::new("Check1".to_string(), "".to_string(), CheckStatus::Passed),
            QualityCheck::new("Check2".to_string(), "".to_string(), CheckStatus::Passed),
        ];

        let gate = QualityGate::new(Phase::Spec, checks);
        assert!(gate.passed);
        assert_eq!(gate.failed_checks().len(), 0);
    }

    #[test]
    fn test_quality_gate_some_checks_fail() {
        let checks = vec![
            QualityCheck::new("Check1".to_string(), "".to_string(), CheckStatus::Passed),
            QualityCheck::new(
                "Check2".to_string(),
                "".to_string(),
                CheckStatus::Failed("Error".to_string()),
            ),
        ];

        let gate = QualityGate::new(Phase::Spec, checks);
        assert!(!gate.passed);
        assert_eq!(gate.failed_checks().len(), 1);
        assert_eq!(gate.failed_checks()[0].name, "Check2");
    }

    #[test]
    fn test_quality_gate_approval() {
        let checks = vec![QualityCheck::new(
            "Check".to_string(),
            "".to_string(),
            CheckStatus::Passed,
        )];

        let mut gate = QualityGate::new(Phase::Spec, checks);
        assert!(!gate.approved);
        assert!(!gate.can_proceed());

        gate.approve("human".to_string());
        assert!(gate.approved);
        assert_eq!(gate.approved_by, Some("human".to_string()));
        assert!(gate.can_proceed());
    }

    #[test]
    fn test_quality_gate_cannot_proceed_if_checks_fail() {
        let checks = vec![QualityCheck::new(
            "Check".to_string(),
            "".to_string(),
            CheckStatus::Failed("error".to_string()),
        )];

        let mut gate = QualityGate::new(Phase::Spec, checks);
        gate.approve("human".to_string());

        // Even with approval, cannot proceed if checks failed
        assert!(!gate.can_proceed());
    }

    #[test]
    fn test_quality_gate_phase_preservation() {
        let checks = vec![QualityCheck::new(
            "Check".to_string(),
            "".to_string(),
            CheckStatus::Passed,
        )];

        let gate = QualityGate::new(Phase::Tasks, checks);
        assert_eq!(gate.phase, Phase::Tasks);
    }
}
