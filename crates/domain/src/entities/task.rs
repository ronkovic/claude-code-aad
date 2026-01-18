//! Task entity.

use crate::value_objects::{Priority, SpecId, Status, TaskId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Task entity.
///
/// Represents an individual work item within a specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for this task.
    pub id: TaskId,
    /// ID of the specification this task belongs to.
    pub spec_id: SpecId,
    /// Short title of the task.
    pub title: String,
    /// Detailed description of what needs to be done.
    pub description: String,
    /// Current status of the task.
    pub status: Status,
    /// Priority level (MoSCoW).
    pub priority: Priority,
    /// Complexity estimate (S/M/L/XL).
    pub complexity: String,
    /// List of task IDs this task depends on.
    pub dependencies: Vec<TaskId>,
    /// When this task was created.
    pub created_at: DateTime<Utc>,
    /// When this task was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Creates a new task.
    ///
    /// # Errors
    ///
    /// Returns an error if the title is empty.
    pub fn new(
        spec_id: SpecId,
        title: String,
        description: String,
        priority: Priority,
        complexity: String,
    ) -> Result<Self, crate::DomainError> {
        if title.trim().is_empty() {
            return Err(crate::DomainError::ValidationError(
                "Task title cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();
        Ok(Self {
            id: TaskId::new(),
            spec_id,
            title: title.trim().to_string(),
            description,
            status: Status::Pending,
            priority,
            complexity,
            dependencies: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Changes the task status.
    pub fn change_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Adds a dependency to this task.
    ///
    /// # Errors
    ///
    /// Returns an error if adding this dependency would create a circular dependency.
    pub fn add_dependency(&mut self, task_id: TaskId) -> Result<(), crate::DomainError> {
        // Check for self-dependency
        if self.id == task_id {
            return Err(crate::DomainError::ValidationError(
                "Task cannot depend on itself".to_string(),
            ));
        }

        // Check if already exists
        if self.dependencies.contains(&task_id) {
            return Ok(());
        }

        self.dependencies.push(task_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Removes a dependency from this task.
    pub fn remove_dependency(&mut self, task_id: &TaskId) {
        if let Some(pos) = self.dependencies.iter().position(|id| id == task_id) {
            self.dependencies.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// Checks if this task has circular dependencies in the given task set.
    ///
    /// This is a helper method for dependency validation.
    pub fn has_circular_dependency(&self, all_tasks: &[Task]) -> Result<bool, crate::DomainError> {
        let mut visited = HashSet::new();
        self.check_circular_recursive(&self.id, all_tasks, &mut visited)
    }

    fn check_circular_recursive(
        &self,
        target_id: &TaskId,
        all_tasks: &[Task],
        visited: &mut HashSet<TaskId>,
    ) -> Result<bool, crate::DomainError> {
        if visited.contains(&self.id) {
            return Ok(false);
        }

        visited.insert(self.id.clone());

        for dep_id in &self.dependencies {
            if dep_id == target_id {
                return Ok(true);
            }

            if let Some(dep_task) = all_tasks.iter().find(|t| &t.id == dep_id) {
                if dep_task.check_circular_recursive(target_id, all_tasks, visited)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let spec_id = SpecId::new();
        let task = Task::new(
            spec_id.clone(),
            "Test Task".to_string(),
            "Description".to_string(),
            Priority::Must,
            "M".to_string(),
        )
        .unwrap();

        assert_eq!(task.spec_id, spec_id);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, "Description");
        assert_eq!(task.status, Status::Pending);
        assert_eq!(task.priority, Priority::Must);
        assert_eq!(task.complexity, "M");
        assert!(task.dependencies.is_empty());
    }

    #[test]
    fn test_task_empty_title_rejected() {
        let spec_id = SpecId::new();
        let result = Task::new(
            spec_id,
            "".to_string(),
            "Desc".to_string(),
            Priority::Must,
            "S".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_task_title_trimming() {
        let spec_id = SpecId::new();
        let task = Task::new(
            spec_id,
            "  Test  ".to_string(),
            "Desc".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();
        assert_eq!(task.title, "Test");
    }

    #[test]
    fn test_task_change_status() {
        let spec_id = SpecId::new();
        let mut task = Task::new(
            spec_id,
            "Test".to_string(),
            "Desc".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        assert_eq!(task.status, Status::Pending);

        task.change_status(Status::InProgress);
        assert_eq!(task.status, Status::InProgress);

        task.change_status(Status::Completed);
        assert_eq!(task.status, Status::Completed);
    }

    #[test]
    fn test_task_add_dependency() {
        let spec_id = SpecId::new();
        let mut task = Task::new(
            spec_id,
            "Test".to_string(),
            "Desc".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let dep_id = TaskId::new();
        task.add_dependency(dep_id.clone()).unwrap();

        assert_eq!(task.dependencies.len(), 1);
        assert_eq!(task.dependencies[0], dep_id);

        // Adding same dependency again should not duplicate
        task.add_dependency(dep_id.clone()).unwrap();
        assert_eq!(task.dependencies.len(), 1);
    }

    #[test]
    fn test_task_self_dependency_rejected() {
        let spec_id = SpecId::new();
        let mut task = Task::new(
            spec_id,
            "Test".to_string(),
            "Desc".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let result = task.add_dependency(task.id.clone());
        assert!(result.is_err());
    }

    #[test]
    fn test_task_remove_dependency() {
        let spec_id = SpecId::new();
        let mut task = Task::new(
            spec_id,
            "Test".to_string(),
            "Desc".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let dep_id1 = TaskId::new();
        let dep_id2 = TaskId::new();

        task.add_dependency(dep_id1.clone()).unwrap();
        task.add_dependency(dep_id2.clone()).unwrap();
        assert_eq!(task.dependencies.len(), 2);

        task.remove_dependency(&dep_id1);
        assert_eq!(task.dependencies.len(), 1);
        assert_eq!(task.dependencies[0], dep_id2);
    }

    #[test]
    fn test_task_circular_dependency_detection() {
        let spec_id = SpecId::new();

        let mut task1 = Task::new(
            spec_id.clone(),
            "Task1".to_string(),
            "".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let mut task2 = Task::new(
            spec_id.clone(),
            "Task2".to_string(),
            "".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let mut task3 = Task::new(
            spec_id,
            "Task3".to_string(),
            "".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        // Create a circular dependency: task1 -> task2 -> task3 -> task1
        task1.add_dependency(task2.id.clone()).unwrap();
        task2.add_dependency(task3.id.clone()).unwrap();
        task3.add_dependency(task1.id.clone()).unwrap();

        let all_tasks = vec![task1.clone(), task2.clone(), task3.clone()];

        assert!(task1.has_circular_dependency(&all_tasks).unwrap());
        assert!(task2.has_circular_dependency(&all_tasks).unwrap());
        assert!(task3.has_circular_dependency(&all_tasks).unwrap());
    }

    #[test]
    fn test_task_no_circular_dependency() {
        let spec_id = SpecId::new();

        let task1 = Task::new(
            spec_id.clone(),
            "Task1".to_string(),
            "".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let mut task2 = Task::new(
            spec_id,
            "Task2".to_string(),
            "".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        // Linear dependency: task2 -> task1
        task2.add_dependency(task1.id.clone()).unwrap();

        let all_tasks = vec![task1.clone(), task2.clone()];

        assert!(!task1.has_circular_dependency(&all_tasks).unwrap());
        assert!(!task2.has_circular_dependency(&all_tasks).unwrap());
    }

    #[test]
    fn test_task_clone() {
        let spec_id = SpecId::new();
        let task = Task::new(
            spec_id,
            "Test".to_string(),
            "Desc".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let cloned = task.clone();
        assert_eq!(task.id, cloned.id);
        assert_eq!(task.title, cloned.title);
    }
}
