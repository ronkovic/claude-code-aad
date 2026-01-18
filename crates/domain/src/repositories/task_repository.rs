//! Task repository trait.

use crate::entities::Task;
use crate::value_objects::{SpecId, TaskId};
use crate::Result;

/// Repository interface for Task entities.
///
/// This trait defines the contract for persisting and retrieving tasks.
/// Concrete implementations belong in the infrastructure layer.
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    /// Finds a task by its ID.
    ///
    /// Returns `None` if the task does not exist.
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>>;

    /// Finds all tasks belonging to a specification.
    async fn find_by_spec_id(&self, spec_id: &SpecId) -> Result<Vec<Task>>;

    /// Saves a task.
    ///
    /// If the task already exists, it will be updated.
    async fn save(&self, task: &Task) -> Result<()>;

    /// Deletes a task by its ID.
    ///
    /// Returns `Ok(())` even if the task does not exist.
    async fn delete(&self, id: &TaskId) -> Result<()>;
}
