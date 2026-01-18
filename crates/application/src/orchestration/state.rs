//! Orchestration state management for resume and dry-run functionality.
//!
//! This module provides functionality to:
//! - Save orchestration state to disk for resume capability
//! - Restore orchestration state from disk
//! - Print execution plan for dry-run mode

use crate::error::{ApplicationError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the state of an orchestration session.
///
/// This can be serialized to JSON and saved to disk for resume functionality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorState {
    /// Specification IDs being orchestrated
    pub spec_ids: Vec<String>,

    /// Phase for each spec
    pub spec_phases: HashMap<String, String>,

    /// Dependencies between specs (spec_id -> list of dependencies)
    pub dependencies: HashMap<String, Vec<String>>,

    /// Completed spec IDs
    pub completed: Vec<String>,

    /// Failed spec IDs
    pub failed: Vec<String>,

    /// Running spec IDs
    pub running: Vec<String>,

    /// Pending spec IDs
    pub pending: Vec<String>,

    /// Timestamp when state was saved
    pub saved_at: String,
}

impl OrchestratorState {
    /// Creates a new orchestrator state.
    pub fn new(spec_ids: Vec<String>) -> Self {
        let pending = spec_ids.clone();
        Self {
            spec_ids,
            spec_phases: HashMap::new(),
            dependencies: HashMap::new(),
            completed: Vec::new(),
            failed: Vec::new(),
            running: Vec::new(),
            pending,
            saved_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Adds a dependency between specs.
    pub fn add_dependency(&mut self, spec_id: &str, depends_on: &str) {
        self.dependencies
            .entry(spec_id.to_string())
            .or_default()
            .push(depends_on.to_string());
    }

    /// Marks a spec as completed.
    pub fn mark_completed(&mut self, spec_id: &str) {
        self.pending.retain(|id| id != spec_id);
        self.running.retain(|id| id != spec_id);
        if !self.completed.contains(&spec_id.to_string()) {
            self.completed.push(spec_id.to_string());
        }
    }

    /// Marks a spec as failed.
    pub fn mark_failed(&mut self, spec_id: &str) {
        self.pending.retain(|id| id != spec_id);
        self.running.retain(|id| id != spec_id);
        if !self.failed.contains(&spec_id.to_string()) {
            self.failed.push(spec_id.to_string());
        }
    }

    /// Marks a spec as running.
    pub fn mark_running(&mut self, spec_id: &str) {
        self.pending.retain(|id| id != spec_id);
        if !self.running.contains(&spec_id.to_string()) {
            self.running.push(spec_id.to_string());
        }
    }

    /// Gets remaining specs (pending + running).
    pub fn remaining_specs(&self) -> Vec<String> {
        let mut remaining = self.pending.clone();
        remaining.extend(self.running.clone());
        remaining
    }

    /// Checks if orchestration is complete.
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty() && self.running.is_empty()
    }

    /// Gets progress percentage.
    pub fn progress_percent(&self) -> u8 {
        let total = self.spec_ids.len();
        if total == 0 {
            return 100;
        }
        let done = self.completed.len() + self.failed.len();
        ((done as f64 / total as f64) * 100.0) as u8
    }
}

/// Saves orchestrator state to disk.
///
/// # Arguments
///
/// * `state` - The state to save
/// * `state_file` - Path to the state file (default: .aad/orchestration/state.json)
///
/// # Examples
///
/// ```
/// use application::orchestration::state::{OrchestratorState, save_state};
///
/// let state = OrchestratorState::new(vec!["SPEC-001".to_string()]);
/// save_state(&state, None).unwrap();
/// ```
pub fn save_state(state: &OrchestratorState, state_file: Option<&Path>) -> Result<()> {
    let default_path = PathBuf::from(".aad/orchestration/state.json");
    let path = state_file.unwrap_or(&default_path);

    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            ApplicationError::Validation(format!("Failed to create directory {:?}: {}", parent, e))
        })?;
    }

    // Serialize state to JSON
    let json = serde_json::to_string_pretty(state)
        .map_err(|e| ApplicationError::Validation(format!("Failed to serialize state: {}", e)))?;

    // Write to file
    fs::write(path, json).map_err(|e| {
        ApplicationError::Validation(format!("Failed to write state file {:?}: {}", path, e))
    })?;

    Ok(())
}

/// Restores orchestrator state from disk.
///
/// # Arguments
///
/// * `state_file` - Path to the state file (default: .aad/orchestration/state.json)
///
/// # Returns
///
/// The restored state, or an error if the file doesn't exist or is invalid.
///
/// # Examples
///
/// ```no_run
/// use application::orchestration::state::restore_state;
///
/// let state = restore_state(None).unwrap();
/// println!("Restored {} specs", state.spec_ids.len());
/// ```
pub fn restore_state(state_file: Option<&Path>) -> Result<OrchestratorState> {
    let default_path = PathBuf::from(".aad/orchestration/state.json");
    let path = state_file.unwrap_or(&default_path);

    // Check if file exists
    if !path.exists() {
        return Err(ApplicationError::Validation(format!(
            "State file not found: {:?}",
            path
        )));
    }

    // Read file
    let json = fs::read_to_string(path).map_err(|e| {
        ApplicationError::Validation(format!("Failed to read state file {:?}: {}", path, e))
    })?;

    // Deserialize state
    let state: OrchestratorState = serde_json::from_str(&json)
        .map_err(|e| ApplicationError::Validation(format!("Failed to deserialize state: {}", e)))?;

    Ok(state)
}

/// Prints an execution plan for dry-run mode.
///
/// Shows the dependency graph and execution order without actually running anything.
///
/// # Arguments
///
/// * `state` - The orchestrator state containing specs and dependencies
///
/// # Examples
///
/// ```
/// use application::orchestration::state::{OrchestratorState, print_execution_plan};
/// use std::collections::HashMap;
///
/// let mut state = OrchestratorState::new(vec!["SPEC-001".to_string(), "SPEC-002".to_string()]);
/// state.add_dependency("SPEC-002", "SPEC-001");
/// print_execution_plan(&state);
/// ```
pub fn print_execution_plan(state: &OrchestratorState) {
    println!("\n📋 実行計画 (Dry Run)\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 1. Show all specs
    println!("\n📦 対象仕様 ({} specs):", state.spec_ids.len());
    for spec_id in &state.spec_ids {
        let phase = state
            .spec_phases
            .get(spec_id)
            .map(|p| p.as_str())
            .unwrap_or("TDD");
        println!("  • {} [Phase: {}]", spec_id, phase);
    }

    // 2. Show dependencies
    if !state.dependencies.is_empty() {
        println!("\n🔗 依存関係:");
        for (spec_id, deps) in &state.dependencies {
            if !deps.is_empty() {
                println!("  {} depends on:", spec_id);
                for dep in deps {
                    println!("    ↳ {}", dep);
                }
            }
        }
    } else {
        println!("\n🔗 依存関係: なし（全て並列実行可能）");
    }

    // 3. Show execution waves (parallel groups)
    println!("\n🌊 実行ウェーブ:");
    let waves = calculate_execution_waves(state);
    for (i, wave) in waves.iter().enumerate() {
        println!("  Wave {}: {} spec(s)", i + 1, wave.len());
        for spec_id in wave {
            println!("    ├─ {}", spec_id);
        }
    }

    // 4. Show estimated parallelism
    let max_parallelism = waves.iter().map(|w| w.len()).max().unwrap_or(0);
    println!("\n⚡ 最大並列度: {} specs", max_parallelism);

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n💡 Tip: --dry-run を外して実際に実行してください\n");
}

/// Calculates execution waves based on dependencies.
///
/// Returns a vector of waves, where each wave contains specs that can run in parallel.
fn calculate_execution_waves(state: &OrchestratorState) -> Vec<Vec<String>> {
    let mut waves: Vec<Vec<String>> = Vec::new();
    let mut completed: Vec<String> = Vec::new();
    let mut remaining: Vec<String> = state.spec_ids.clone();

    while !remaining.is_empty() {
        let mut current_wave = Vec::new();

        // Find specs that can run (all dependencies completed)
        for spec_id in &remaining {
            let deps = state.dependencies.get(spec_id);
            let can_run = if let Some(deps) = deps {
                deps.iter().all(|dep| completed.contains(dep))
            } else {
                true // No dependencies
            };

            if can_run {
                current_wave.push(spec_id.clone());
            }
        }

        if current_wave.is_empty() {
            // Deadlock - circular dependency or missing dependency
            eprintln!("⚠️  警告: 依存関係の循環または欠落を検出");
            break;
        }

        // Mark current wave as completed
        for spec_id in &current_wave {
            completed.push(spec_id.clone());
            remaining.retain(|id| id != spec_id);
        }

        waves.push(current_wave);
    }

    waves
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_orchestrator_state_new() {
        let spec_ids = vec!["SPEC-001".to_string(), "SPEC-002".to_string()];
        let state = OrchestratorState::new(spec_ids.clone());

        assert_eq!(state.spec_ids, spec_ids);
        assert_eq!(state.pending, spec_ids);
        assert!(state.completed.is_empty());
        assert!(state.failed.is_empty());
        assert!(state.running.is_empty());
    }

    #[test]
    fn test_add_dependency() {
        let mut state = OrchestratorState::new(vec!["SPEC-001".to_string()]);
        state.add_dependency("SPEC-002", "SPEC-001");

        assert_eq!(
            state.dependencies.get("SPEC-002"),
            Some(&vec!["SPEC-001".to_string()])
        );
    }

    #[test]
    fn test_mark_completed() {
        let mut state = OrchestratorState::new(vec!["SPEC-001".to_string()]);
        state.mark_completed("SPEC-001");

        assert!(state.completed.contains(&"SPEC-001".to_string()));
        assert!(state.pending.is_empty());
    }

    #[test]
    fn test_mark_failed() {
        let mut state = OrchestratorState::new(vec!["SPEC-001".to_string()]);
        state.mark_failed("SPEC-001");

        assert!(state.failed.contains(&"SPEC-001".to_string()));
        assert!(state.pending.is_empty());
    }

    #[test]
    fn test_mark_running() {
        let mut state = OrchestratorState::new(vec!["SPEC-001".to_string()]);
        state.mark_running("SPEC-001");

        assert!(state.running.contains(&"SPEC-001".to_string()));
        assert!(state.pending.is_empty());
    }

    #[test]
    fn test_is_complete() {
        let mut state = OrchestratorState::new(vec!["SPEC-001".to_string()]);
        assert!(!state.is_complete());

        state.mark_completed("SPEC-001");
        assert!(state.is_complete());
    }

    #[test]
    fn test_progress_percent() {
        let mut state = OrchestratorState::new(vec![
            "SPEC-001".to_string(),
            "SPEC-002".to_string(),
            "SPEC-003".to_string(),
            "SPEC-004".to_string(),
        ]);

        assert_eq!(state.progress_percent(), 0);

        state.mark_completed("SPEC-001");
        assert_eq!(state.progress_percent(), 25);

        state.mark_completed("SPEC-002");
        assert_eq!(state.progress_percent(), 50);

        state.mark_failed("SPEC-003");
        assert_eq!(state.progress_percent(), 75);

        state.mark_completed("SPEC-004");
        assert_eq!(state.progress_percent(), 100);
    }

    #[test]
    fn test_save_and_restore_state() {
        let temp_dir = std::env::temp_dir();
        let state_file = temp_dir.join("test_orchestrator_state.json");

        // Clean up if exists
        let _ = fs::remove_file(&state_file);

        let mut state = OrchestratorState::new(vec!["SPEC-001".to_string()]);
        state.mark_completed("SPEC-001");

        // Save state
        save_state(&state, Some(&state_file)).unwrap();
        assert!(state_file.exists());

        // Restore state
        let restored = restore_state(Some(&state_file)).unwrap();
        assert_eq!(restored.spec_ids, state.spec_ids);
        assert_eq!(restored.completed, state.completed);

        // Clean up
        fs::remove_file(&state_file).unwrap();
    }

    #[test]
    fn test_restore_nonexistent_state() {
        let result = restore_state(Some(Path::new("/nonexistent/state.json")));
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_execution_waves() {
        let mut state = OrchestratorState::new(vec![
            "SPEC-001".to_string(),
            "SPEC-002".to_string(),
            "SPEC-003".to_string(),
        ]);

        // SPEC-002 depends on SPEC-001
        // SPEC-003 depends on SPEC-001
        state.add_dependency("SPEC-002", "SPEC-001");
        state.add_dependency("SPEC-003", "SPEC-001");

        let waves = calculate_execution_waves(&state);

        // Wave 0: SPEC-001
        assert_eq!(waves.len(), 2);
        assert_eq!(waves[0].len(), 1);
        assert!(waves[0].contains(&"SPEC-001".to_string()));

        // Wave 1: SPEC-002 and SPEC-003 (parallel)
        assert_eq!(waves[1].len(), 2);
        assert!(waves[1].contains(&"SPEC-002".to_string()));
        assert!(waves[1].contains(&"SPEC-003".to_string()));
    }

    #[test]
    fn test_print_execution_plan() {
        let mut state =
            OrchestratorState::new(vec!["SPEC-001".to_string(), "SPEC-002".to_string()]);
        state.add_dependency("SPEC-002", "SPEC-001");

        // This test just ensures the function doesn't panic
        print_execution_plan(&state);
    }

    #[test]
    fn test_remaining_specs() {
        let mut state = OrchestratorState::new(vec![
            "SPEC-001".to_string(),
            "SPEC-002".to_string(),
            "SPEC-003".to_string(),
        ]);

        state.mark_running("SPEC-001");
        state.mark_completed("SPEC-002");

        let remaining = state.remaining_specs();
        assert_eq!(remaining.len(), 2); // SPEC-001 (running) + SPEC-003 (pending)
        assert!(remaining.contains(&"SPEC-001".to_string()));
        assert!(remaining.contains(&"SPEC-003".to_string()));
    }
}
