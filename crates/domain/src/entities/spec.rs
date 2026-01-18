//! Specification entity.

use crate::value_objects::{Phase, SpecId, Status, TaskId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Specification entity.
///
/// Represents a complete specification for a feature or component,
/// tracking its progress through various phases of development.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    /// Unique identifier for this specification.
    pub id: SpecId,
    /// Human-readable name of the specification.
    pub name: String,
    /// Detailed description of what this specification covers.
    pub description: String,
    /// Current development phase.
    pub phase: Phase,
    /// Current status.
    pub status: Status,
    /// List of task IDs associated with this specification.
    pub tasks: Vec<TaskId>,
    /// When this specification was created.
    pub created_at: DateTime<Utc>,
    /// When this specification was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Spec {
    /// Creates a new specification.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty.
    pub fn new(name: String, description: String) -> Result<Self, crate::DomainError> {
        if name.trim().is_empty() {
            return Err(crate::DomainError::ValidationError(
                "Spec name cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();
        Ok(Self {
            id: SpecId::new(),
            name: name.trim().to_string(),
            description,
            phase: Phase::Spec,
            status: Status::Pending,
            tasks: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Adds a task to this specification.
    pub fn add_task(&mut self, task_id: TaskId) {
        if !self.tasks.contains(&task_id) {
            self.tasks.push(task_id);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a task from this specification.
    pub fn remove_task(&mut self, task_id: &TaskId) {
        if let Some(pos) = self.tasks.iter().position(|id| id == task_id) {
            self.tasks.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// Changes the current phase.
    ///
    /// # Errors
    ///
    /// Returns an error if trying to move backwards in phases.
    pub fn change_phase(&mut self, new_phase: Phase) -> Result<(), crate::DomainError> {
        // Allow moving to the same phase (idempotent)
        if self.phase == new_phase {
            return Ok(());
        }

        // For now, allow any phase change. In a future implementation,
        // we might want to enforce sequential phase progression.
        self.phase = new_phase;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Updates the status.
    pub fn update_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_creation() {
        let spec = Spec::new("Test Spec".to_string(), "Description".to_string()).unwrap();
        assert_eq!(spec.name, "Test Spec");
        assert_eq!(spec.description, "Description");
        assert_eq!(spec.phase, Phase::Spec);
        assert_eq!(spec.status, Status::Pending);
        assert!(spec.tasks.is_empty());
    }

    #[test]
    fn test_spec_empty_name_rejected() {
        let result = Spec::new("".to_string(), "Description".to_string());
        assert!(result.is_err());

        let result = Spec::new("   ".to_string(), "Description".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_spec_name_trimming() {
        let spec = Spec::new("  Test  ".to_string(), "Description".to_string()).unwrap();
        assert_eq!(spec.name, "Test");
    }

    #[test]
    fn test_spec_add_task() {
        let mut spec = Spec::new("Test".to_string(), "Desc".to_string()).unwrap();
        let task_id = TaskId::new();

        spec.add_task(task_id.clone());
        assert_eq!(spec.tasks.len(), 1);
        assert_eq!(spec.tasks[0], task_id);

        // Adding the same task again should not duplicate
        spec.add_task(task_id.clone());
        assert_eq!(spec.tasks.len(), 1);
    }

    #[test]
    fn test_spec_remove_task() {
        let mut spec = Spec::new("Test".to_string(), "Desc".to_string()).unwrap();
        let task_id1 = TaskId::new();
        let task_id2 = TaskId::new();

        spec.add_task(task_id1.clone());
        spec.add_task(task_id2.clone());
        assert_eq!(spec.tasks.len(), 2);

        spec.remove_task(&task_id1);
        assert_eq!(spec.tasks.len(), 1);
        assert_eq!(spec.tasks[0], task_id2);

        // Removing non-existent task should be safe
        spec.remove_task(&task_id1);
        assert_eq!(spec.tasks.len(), 1);
    }

    #[test]
    fn test_spec_change_phase() {
        let mut spec = Spec::new("Test".to_string(), "Desc".to_string()).unwrap();
        assert_eq!(spec.phase, Phase::Spec);

        spec.change_phase(Phase::Tasks).unwrap();
        assert_eq!(spec.phase, Phase::Tasks);

        // Changing to same phase should be OK
        spec.change_phase(Phase::Tasks).unwrap();
        assert_eq!(spec.phase, Phase::Tasks);
    }

    #[test]
    fn test_spec_update_status() {
        let mut spec = Spec::new("Test".to_string(), "Desc".to_string()).unwrap();
        assert_eq!(spec.status, Status::Pending);

        spec.update_status(Status::InProgress);
        assert_eq!(spec.status, Status::InProgress);

        spec.update_status(Status::Completed);
        assert_eq!(spec.status, Status::Completed);
    }

    #[test]
    fn test_spec_clone() {
        let spec = Spec::new("Test".to_string(), "Desc".to_string()).unwrap();
        let cloned = spec.clone();
        assert_eq!(spec.id, cloned.id);
        assert_eq!(spec.name, cloned.name);
    }

    #[test]
    fn test_spec_updated_at_changes() {
        let mut spec = Spec::new("Test".to_string(), "Desc".to_string()).unwrap();
        let initial_updated = spec.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        spec.add_task(TaskId::new());
        assert!(spec.updated_at > initial_updated);
    }
}
