//! Gate command implementation for quality gate validation.

use anyhow::Result;
use application::quality::QualityService;
use domain::entities::{Spec, Task};
use domain::repositories::{SpecRepository, TaskRepository};
use domain::value_objects::Phase;
use infrastructure::persistence::{SpecJsonRepo, TaskJsonRepo};
use std::str::FromStr;

/// Executes the gate command to validate quality gate conditions for a given phase.
///
/// # Arguments
///
/// * `phase_str` - The phase name (e.g., "SPEC", "TASKS", "TDD")
///
/// # Errors
///
/// Returns an error if:
/// - Phase name is invalid
/// - Quality gate validation fails
/// - Repository operations fail
///
/// # Returns
///
/// Returns `Ok(())` if gate passes, `Err` with exit code 1 if gate fails
pub fn execute(phase_str: &str) -> Result<()> {
    // Parse phase from string
    let phase = Phase::from_str(phase_str).map_err(|e| {
        anyhow::anyhow!("Invalid phase '{}': {}", phase_str, e)
    })?;

    // Initialize repositories
    let spec_repo = SpecJsonRepo::new(".aad/data/specs");
    let task_repo = TaskJsonRepo::new(".aad/data/tasks");

    // Initialize quality service
    let quality_service = QualityService::new();

    // Load specs and tasks
    let rt = tokio::runtime::Runtime::new()?;
    let (spec, tasks) = rt.block_on(async {
        let specs = spec_repo.find_all().await?;

        // For simplicity, we use the first spec if available
        // In a real scenario, this would be spec-specific
        let spec = specs.first().ok_or_else(|| {
            anyhow::anyhow!("No specifications found. Please run 'aad spec' first.")
        })?;

        let tasks = task_repo.find_by_spec_id(&spec.id).await?;

        Ok::<(Spec, Vec<Task>), anyhow::Error>((spec.clone(), tasks))
    })?;

    // Check quality gate
    let gate = quality_service
        .check_phase_gate(phase, &spec, &tasks)
        .map_err(|e| anyhow::anyhow!("Quality gate check failed: {}", e))?;

    // Generate and display report with colored output
    let report = quality_service.generate_report(&gate);

    // Add colors: green for PASSED/✓, red for FAILED/✗
    let colored_report = colorize_report(&report, gate.passed);
    println!("{}", colored_report);

    // Exit with error code 1 if gate failed
    if !gate.passed {
        anyhow::bail!("Quality gate FAILED for {} phase", phase_str);
    }

    println!("\n✅ Quality gate PASSED for {} phase", phase_str);
    Ok(())
}

/// Adds color codes to the report output.
fn colorize_report(report: &str, passed: bool) -> String {
    let mut colored = report.to_string();

    // Replace status indicators with colored versions
    colored = colored.replace("✓ PASS", "\x1b[32m✅ PASS\x1b[0m"); // Green
    colored = colored.replace("✗ FAIL", "\x1b[31m❌ FAIL\x1b[0m"); // Red

    // Color the overall status
    if passed {
        colored = colored.replace("Overall Status: PASSED", "\x1b[32mOverall Status: PASSED\x1b[0m");
    } else {
        colored = colored.replace("Overall Status: FAILED", "\x1b[31mOverall Status: FAILED\x1b[0m");
    }

    colored
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_objects::{Priority, SpecId};
    use std::fs;
    use std::str::FromStr;
    use tempfile::TempDir;

    fn setup_test_env() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let aad_dir = temp_dir.path().join(".aad");
        let data_dir = aad_dir.join("data");
        let specs_dir = data_dir.join("specs");
        let tasks_dir = data_dir.join("tasks");

        fs::create_dir_all(&specs_dir).unwrap();
        fs::create_dir_all(&tasks_dir).unwrap();

        temp_dir
    }

    fn create_test_spec(temp_dir: &TempDir, spec_id: &str, description: &str) {
        let spec = Spec::new(spec_id.to_string(), description.to_string()).unwrap();
        let spec_file = temp_dir
            .path()
            .join(".aad/data/specs")
            .join(format!("{}.json", spec_id));

        let spec_json = serde_json::to_string_pretty(&spec).unwrap();
        fs::write(spec_file, spec_json).unwrap();
    }

    fn create_test_task(temp_dir: &TempDir, spec_id: &str, task_id: &str, title: &str) {
        let spec_id_obj = SpecId::from_str(spec_id).unwrap();
        let task = Task::new(
            spec_id_obj,
            title.to_string(),
            "Test description".to_string(),
            Priority::Must,
            "M".to_string(),
        )
        .unwrap();

        let tasks_dir = temp_dir
            .path()
            .join(".aad/data/tasks")
            .join(spec_id);
        fs::create_dir_all(&tasks_dir).unwrap();

        let task_file = tasks_dir.join(format!("{}.json", task_id));
        let task_json = serde_json::to_string_pretty(&task).unwrap();
        fs::write(task_file, task_json).unwrap();
    }

    #[test]
    fn test_execute_spec_phase_passes() {
        let temp_dir = setup_test_env();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        create_test_spec(&temp_dir, "SPEC-001", "Valid description for testing");

        let result = execute("SPEC");

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_spec_phase_fails_empty_description() {
        let temp_dir = setup_test_env();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        create_test_spec(&temp_dir, "SPEC-001", "");

        let result = execute("SPEC");

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("FAILED"));
    }

    #[test]
    fn test_execute_tasks_phase_passes() {
        let temp_dir = setup_test_env();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        create_test_spec(&temp_dir, "SPEC-001", "Valid spec");
        create_test_task(&temp_dir, "SPEC-001", "SPEC-001-T01", "Task 1");
        create_test_task(&temp_dir, "SPEC-001", "SPEC-001-T02", "Task 2");

        let result = execute("TASKS");

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_tasks_phase_fails_no_tasks() {
        let temp_dir = setup_test_env();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        create_test_spec(&temp_dir, "SPEC-001", "Valid spec");

        let result = execute("TASKS");

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("FAILED"));
    }

    #[test]
    fn test_execute_tdd_phase_passes() {
        let temp_dir = setup_test_env();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        create_test_spec(&temp_dir, "SPEC-001", "Valid spec");

        let result = execute("TDD");

        std::env::set_current_dir(original_dir).unwrap();

        // TDD phase has placeholder checks that always pass
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_invalid_phase() {
        let temp_dir = setup_test_env();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = execute("INVALID");

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid phase"));
    }

    #[test]
    fn test_execute_no_specs_found() {
        let temp_dir = setup_test_env();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = execute("SPEC");

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No specifications found"));
    }

    #[test]
    fn test_colorize_report_passed() {
        let report = "Overall Status: PASSED\n✓ PASS - Test check";
        let colored = colorize_report(report, true);

        // Should contain green color code for PASSED
        assert!(colored.contains("\x1b[32m"));
        // Should contain reset code
        assert!(colored.contains("\x1b[0m"));
        // Should replace ✓ with ✅
        assert!(colored.contains("✅"));
    }

    #[test]
    fn test_colorize_report_failed() {
        let report = "Overall Status: FAILED\n✗ FAIL - Test check";
        let colored = colorize_report(report, false);

        // Should contain red color code for FAILED
        assert!(colored.contains("\x1b[31m"));
        // Should contain reset code
        assert!(colored.contains("\x1b[0m"));
        // Should replace ✗ with ❌
        assert!(colored.contains("❌"));
    }
}
