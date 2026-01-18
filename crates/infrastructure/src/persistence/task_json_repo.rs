//! JSON-based implementation of TaskRepository.

use async_trait::async_trait;
use domain::{
    entities::Task,
    repositories::TaskRepository,
    value_objects::{SpecId, TaskId},
    Result as DomainResult,
};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::persistence::{PersistenceError, Result};

/// JSON file-based implementation of TaskRepository.
///
/// Stores tasks as individual JSON files in `.aad/data/tasks/` directory.
pub struct TaskJsonRepo {
    base_dir: PathBuf,
}

impl TaskJsonRepo {
    /// Creates a new TaskJsonRepo.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - Base directory for storing task files (e.g., `.aad/data/tasks`)
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Gets the file path for a task ID.
    fn get_file_path(&self, id: &TaskId) -> PathBuf {
        self.base_dir.join(format!("{}.json", id.as_str()))
    }

    /// Ensures the base directory exists.
    async fn ensure_dir_exists(&self) -> Result<()> {
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir).await?;
        }
        Ok(())
    }

    /// Validates the task ID to prevent path traversal attacks.
    fn validate_id(&self, id: &TaskId) -> Result<()> {
        let id_str = id.as_str();
        if id_str.contains("..") || id_str.contains('/') || id_str.contains('\\') {
            return Err(PersistenceError::PathTraversalError(format!(
                "Invalid TaskId: {}",
                id_str
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl TaskRepository for TaskJsonRepo {
    async fn find_by_id(&self, id: &TaskId) -> DomainResult<Option<Task>> {
        self.validate_id(id)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        let file_path = self.get_file_path(id);

        if !file_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&file_path)
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        let task: Task = serde_json::from_str(&content)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        Ok(Some(task))
    }

    async fn find_by_spec_id(&self, spec_id: &SpecId) -> DomainResult<Vec<Task>> {
        if !self.base_dir.exists() {
            return Ok(Vec::new());
        }

        let mut tasks = Vec::new();
        let mut entries = fs::read_dir(&self.base_dir)
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .await
                    .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

                let task: Task = serde_json::from_str(&content)
                    .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

                if &task.spec_id == spec_id {
                    tasks.push(task);
                }
            }
        }

        Ok(tasks)
    }

    async fn save(&self, task: &Task) -> DomainResult<()> {
        self.validate_id(&task.id)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        self.ensure_dir_exists()
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        let file_path = self.get_file_path(&task.id);
        let content = serde_json::to_string_pretty(task)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        fs::write(&file_path, content)
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        Ok(())
    }

    async fn delete(&self, id: &TaskId) -> DomainResult<()> {
        self.validate_id(id)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        let file_path = self.get_file_path(id);

        if file_path.exists() {
            fs::remove_file(&file_path)
                .await
                .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{entities::Task, value_objects::Priority};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_task_json_repo_save_and_find() {
        let temp_dir = TempDir::new().unwrap();
        let repo = TaskJsonRepo::new(temp_dir.path());

        let spec_id = SpecId::new();
        let task = Task::new(
            spec_id,
            "Test Task".to_string(),
            "Test Description".to_string(),
            Priority::Must,
            "M".to_string(),
        )
        .unwrap();
        let task_id = task.id.clone();

        // Save task
        repo.save(&task).await.unwrap();

        // Find task
        let found = repo.find_by_id(&task_id).await.unwrap();
        assert!(found.is_some());
        let found_task = found.unwrap();
        assert_eq!(found_task.title, "Test Task");
        assert_eq!(found_task.description, "Test Description");
    }

    #[tokio::test]
    async fn test_task_json_repo_find_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let repo = TaskJsonRepo::new(temp_dir.path());

        let nonexistent_id = TaskId::new();
        let result = repo.find_by_id(&nonexistent_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_task_json_repo_find_by_spec_id() {
        let temp_dir = TempDir::new().unwrap();
        let repo = TaskJsonRepo::new(temp_dir.path());

        let spec_id1 = SpecId::new();
        let spec_id2 = SpecId::new();

        let task1 = Task::new(
            spec_id1.clone(),
            "Task 1".to_string(),
            "Desc 1".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let task2 = Task::new(
            spec_id1.clone(),
            "Task 2".to_string(),
            "Desc 2".to_string(),
            Priority::Should,
            "M".to_string(),
        )
        .unwrap();

        let task3 = Task::new(
            spec_id2.clone(),
            "Task 3".to_string(),
            "Desc 3".to_string(),
            Priority::Could,
            "L".to_string(),
        )
        .unwrap();

        repo.save(&task1).await.unwrap();
        repo.save(&task2).await.unwrap();
        repo.save(&task3).await.unwrap();

        // Find tasks by spec_id1
        let tasks_for_spec1 = repo.find_by_spec_id(&spec_id1).await.unwrap();
        assert_eq!(tasks_for_spec1.len(), 2);

        // Find tasks by spec_id2
        let tasks_for_spec2 = repo.find_by_spec_id(&spec_id2).await.unwrap();
        assert_eq!(tasks_for_spec2.len(), 1);
    }

    #[tokio::test]
    async fn test_task_json_repo_delete() {
        let temp_dir = TempDir::new().unwrap();
        let repo = TaskJsonRepo::new(temp_dir.path());

        let spec_id = SpecId::new();
        let task = Task::new(
            spec_id,
            "Test Task".to_string(),
            "Test Description".to_string(),
            Priority::Must,
            "M".to_string(),
        )
        .unwrap();
        let task_id = task.id.clone();

        // Save and verify
        repo.save(&task).await.unwrap();
        assert!(repo.find_by_id(&task_id).await.unwrap().is_some());

        // Delete and verify
        repo.delete(&task_id).await.unwrap();
        assert!(repo.find_by_id(&task_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_task_json_repo_delete_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let repo = TaskJsonRepo::new(temp_dir.path());

        let nonexistent_id = TaskId::new();
        // Should not error even if file doesn't exist
        repo.delete(&nonexistent_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_task_json_repo_path_traversal_protection() {
        let temp_dir = TempDir::new().unwrap();
        let repo = TaskJsonRepo::new(temp_dir.path());

        let malicious_id = std::str::FromStr::from_str("../../../etc/passwd").unwrap();
        let result = repo.find_by_id(&malicious_id).await;
        assert!(result.is_err());
    }
}
