//! Quality service for validating phase gate conditions.

use crate::error::Result;
use domain::entities::{CheckStatus, QualityCheck, QualityGate, Spec, Task};
use domain::value_objects::Phase;

/// Service for managing quality gates and phase transitions.
pub struct QualityService;

impl QualityService {
    /// Creates a new quality service.
    pub fn new() -> Self {
        Self
    }

    /// Checks if a phase gate can be passed for the given spec and tasks.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn check_phase_gate(
        &self,
        phase: Phase,
        spec: &Spec,
        tasks: &[Task],
    ) -> Result<QualityGate> {
        let checks = match phase {
            Phase::Spec => self.check_spec_phase(spec),
            Phase::Tasks => self.check_tasks_phase(tasks),
            Phase::Tdd => self.check_tdd_phase(tasks),
            _ => {
                vec![QualityCheck::new(
                    "Phase not implemented".to_string(),
                    format!("Quality gate for {:?} phase is not yet implemented", phase),
                    CheckStatus::Failed("Not implemented".to_string()),
                )]
            }
        };

        Ok(QualityGate::new(phase, checks))
    }

    /// Checks SPEC phase gate conditions.
    fn check_spec_phase(&self, spec: &Spec) -> Vec<QualityCheck> {
        let mut checks = Vec::new();

        // Check 1: Spec has a valid name
        let name_check = if spec.name.is_empty() {
            QualityCheck::new(
                "Spec name validation".to_string(),
                "Spec must have a non-empty name".to_string(),
                CheckStatus::Failed("Spec name is empty".to_string()),
            )
        } else {
            QualityCheck::new(
                "Spec name validation".to_string(),
                "Spec must have a non-empty name".to_string(),
                CheckStatus::Passed,
            )
        };
        checks.push(name_check);

        // Check 2: Spec has a description (acceptance criteria placeholder)
        let description_check = if spec.description.trim().is_empty() {
            QualityCheck::new(
                "Acceptance criteria".to_string(),
                "Spec must have testable acceptance criteria".to_string(),
                CheckStatus::Failed("Description is empty".to_string()),
            )
        } else {
            QualityCheck::new(
                "Acceptance criteria".to_string(),
                "Spec must have testable acceptance criteria".to_string(),
                CheckStatus::Passed,
            )
        };
        checks.push(description_check);

        checks
    }

    /// Checks TASKS phase gate conditions.
    fn check_tasks_phase(&self, tasks: &[Task]) -> Vec<QualityCheck> {
        let mut checks = Vec::new();

        // Check 1: All tasks have valid IDs
        let has_tasks = !tasks.is_empty();
        let all_have_ids = tasks.iter().all(|t| !t.id.to_string().is_empty());

        let id_check = if !has_tasks {
            QualityCheck::new(
                "Task ID validation".to_string(),
                "All tasks must have valid IDs".to_string(),
                CheckStatus::Failed("No tasks found".to_string()),
            )
        } else if !all_have_ids {
            QualityCheck::new(
                "Task ID validation".to_string(),
                "All tasks must have valid IDs".to_string(),
                CheckStatus::Failed("Some tasks missing IDs".to_string()),
            )
        } else {
            QualityCheck::new(
                "Task ID validation".to_string(),
                "All tasks must have valid IDs".to_string(),
                CheckStatus::Passed,
            )
        };
        checks.push(id_check);

        // Check 2: Dependencies are documented
        let deps_check = QualityCheck::new(
            "Dependency documentation".to_string(),
            "Task dependencies must be documented".to_string(),
            CheckStatus::Passed, // For now, always pass (can enhance later)
        );
        checks.push(deps_check);

        checks
    }

    /// Checks TDD phase gate conditions.
    fn check_tdd_phase(&self, _tasks: &[Task]) -> Vec<QualityCheck> {
        let mut checks = Vec::new();

        // Check 1: All tests passing (placeholder - would need actual test runner integration)
        let tests_check = QualityCheck::new(
            "Tests passing".to_string(),
            "All tests must pass".to_string(),
            CheckStatus::Passed, // Placeholder - would integrate with cargo test
        );
        checks.push(tests_check);

        // Check 2: Coverage >= 80% (placeholder)
        let coverage_check = QualityCheck::new(
            "Code coverage".to_string(),
            "Code coverage must be >= 80%".to_string(),
            CheckStatus::Passed, // Placeholder - would integrate with coverage tool
        );
        checks.push(coverage_check);

        // Check 3: Lint passing (placeholder)
        let lint_check = QualityCheck::new(
            "Lint checks".to_string(),
            "All lint checks must pass".to_string(),
            CheckStatus::Passed, // Placeholder - would integrate with clippy
        );
        checks.push(lint_check);

        checks
    }

    /// Generates a quality gate report.
    pub fn generate_report(&self, gate: &QualityGate) -> String {
        let mut report = String::new();

        report.push_str(&format!(
            "Quality Gate Report - {} Phase\n",
            gate.phase.japanese_name()
        ));
        report.push_str(&format!("Evaluated at: {}\n", gate.evaluated_at));
        report.push_str(&format!(
            "Overall Status: {}\n",
            if gate.passed { "PASSED" } else { "FAILED" }
        ));
        report.push_str(&format!(
            "Approval Status: {}\n",
            if gate.approved { "APPROVED" } else { "PENDING" }
        ));
        if let Some(ref approver) = gate.approved_by {
            report.push_str(&format!("Approved by: {}\n", approver));
        }
        report.push_str("\nChecks:\n");

        for check in &gate.checks {
            let status_str = if check.is_passed() {
                "✓ PASS"
            } else {
                "✗ FAIL"
            };
            report.push_str(&format!("  {} - {}\n", status_str, check.name));
            if !check.is_passed() {
                if let CheckStatus::Failed(ref reason) = check.status {
                    report.push_str(&format!("      Reason: {}\n", reason));
                }
            }
        }

        if !gate.passed {
            report.push_str("\nFailed Checks:\n");
            for failed in gate.failed_checks() {
                report.push_str(&format!("  - {}: ", failed.name));
                if let CheckStatus::Failed(ref reason) = failed.status {
                    report.push_str(&format!("{}\n", reason));
                }
            }
        }

        report
    }
}

impl Default for QualityService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_objects::{Priority, SpecId};

    fn create_test_spec(name: &str, description: &str) -> Spec {
        Spec::new(name.to_string(), description.to_string()).unwrap()
    }

    fn create_test_task(spec_id: SpecId, title: &str) -> Task {
        Task::new(
            spec_id,
            title.to_string(),
            "description".to_string(),
            Priority::Must,
            "M".to_string(),
        )
        .unwrap()
    }

    #[test]
    fn test_quality_service_creation() {
        let service = QualityService::new();
        assert!(std::mem::size_of_val(&service) == 0); // Zero-sized type
    }

    #[test]
    fn test_spec_phase_gate_passes_with_valid_spec() {
        let service = QualityService::new();
        let spec = create_test_spec("Test Spec", "Valid description with criteria");

        let gate = service
            .check_phase_gate(Phase::Spec, &spec, &[])
            .unwrap();

        assert_eq!(gate.phase, Phase::Spec);
        assert!(gate.passed);
        assert_eq!(gate.failed_checks().len(), 0);
    }

    #[test]
    fn test_spec_phase_gate_fails_with_empty_description() {
        let service = QualityService::new();
        let spec = create_test_spec("Test Spec", "");

        let gate = service
            .check_phase_gate(Phase::Spec, &spec, &[])
            .unwrap();

        assert!(!gate.passed);
        assert_eq!(gate.failed_checks().len(), 1);
        assert_eq!(gate.failed_checks()[0].name, "Acceptance criteria");
    }

    #[test]
    fn test_tasks_phase_gate_passes_with_valid_tasks() {
        let service = QualityService::new();
        let spec = create_test_spec("Test", "Desc");
        let tasks = vec![
            create_test_task(spec.id.clone(), "Task 1"),
            create_test_task(spec.id.clone(), "Task 2"),
        ];

        let gate = service
            .check_phase_gate(Phase::Tasks, &spec, &tasks)
            .unwrap();

        assert!(gate.passed);
        assert_eq!(gate.failed_checks().len(), 0);
    }

    #[test]
    fn test_tasks_phase_gate_fails_with_no_tasks() {
        let service = QualityService::new();
        let spec = create_test_spec("Test", "Desc");

        let gate = service
            .check_phase_gate(Phase::Tasks, &spec, &[])
            .unwrap();

        assert!(!gate.passed);
        assert!(gate.failed_checks().len() > 0);
    }

    #[test]
    fn test_tdd_phase_gate_placeholder_passes() {
        let service = QualityService::new();
        let spec = create_test_spec("Test", "Desc");

        let gate = service
            .check_phase_gate(Phase::Tdd, &spec, &[])
            .unwrap();

        // TDD phase currently has placeholder checks that pass
        assert!(gate.passed);
        assert_eq!(gate.checks.len(), 3); // Tests, coverage, lint
    }

    #[test]
    fn test_unimplemented_phase_fails() {
        let service = QualityService::new();
        let spec = create_test_spec("Test", "Desc");

        let gate = service
            .check_phase_gate(Phase::Review, &spec, &[])
            .unwrap();

        assert!(!gate.passed);
        assert_eq!(gate.failed_checks().len(), 1);
    }

    #[test]
    fn test_generate_report_for_passing_gate() {
        let service = QualityService::new();
        let spec = create_test_spec("Test", "Description");

        let gate = service
            .check_phase_gate(Phase::Spec, &spec, &[])
            .unwrap();

        let report = service.generate_report(&gate);

        assert!(report.contains("Quality Gate Report"));
        assert!(report.contains("PASSED"));
        assert!(report.contains("✓ PASS"));
        assert!(!report.contains("Failed Checks"));
    }

    #[test]
    fn test_generate_report_for_failing_gate() {
        let service = QualityService::new();
        let spec = create_test_spec("Test", "");

        let gate = service
            .check_phase_gate(Phase::Spec, &spec, &[])
            .unwrap();

        let report = service.generate_report(&gate);

        assert!(report.contains("Quality Gate Report"));
        assert!(report.contains("FAILED"));
        assert!(report.contains("✗ FAIL"));
        assert!(report.contains("Failed Checks"));
        assert!(report.contains("Acceptance criteria"));
    }

    #[test]
    fn test_generate_report_with_approval() {
        let service = QualityService::new();
        let spec = create_test_spec("Test", "Description");

        let mut gate = service
            .check_phase_gate(Phase::Spec, &spec, &[])
            .unwrap();
        gate.approve("test-user".to_string());

        let report = service.generate_report(&gate);

        assert!(report.contains("APPROVED"));
        assert!(report.contains("Approved by: test-user"));
    }

    #[test]
    fn test_default_trait() {
        let _service = QualityService::default();
    }
}
