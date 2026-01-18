//! Task execution loop engine.

use crate::error::{ApplicationError, Result};
use domain::entities::{LoopState, Task};
use domain::value_objects::{SpecId, Status, TaskId};
use std::fs;
use std::path::PathBuf;

/// Loop engine for task execution.
///
/// The loop engine manages the execution of tasks for a specification in a loop,
/// with support for pause/resume functionality.
#[derive(Debug)]
pub struct LoopEngine {
    /// Current loop state.
    state: LoopState,
    /// Path to the state file.
    state_file: PathBuf,
    /// Maximum number of retries per task.
    max_retries: u32,
}

impl LoopEngine {
    /// Creates a new loop engine for the given specification.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The specification ID to create the loop for
    ///
    /// # Examples
    ///
    /// ```
    /// use application::loop_engine::LoopEngine;
    /// use domain::value_objects::SpecId;
    ///
    /// let spec_id = SpecId::new();
    /// let engine = LoopEngine::new(spec_id);
    /// ```
    pub fn new(spec_id: SpecId) -> Self {
        let state = LoopState::new(spec_id);
        let state_file = PathBuf::from(".aad/loop-state.json");
        Self {
            state,
            state_file,
            max_retries: 3,
        }
    }

    /// Creates a new loop engine with a custom state file path.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The specification ID to create the loop for
    /// * `state_file` - Path to the state file
    pub fn with_state_file(spec_id: SpecId, state_file: PathBuf) -> Self {
        let state = LoopState::new(spec_id);
        Self {
            state,
            state_file,
            max_retries: 3,
        }
    }

    /// Creates a new loop engine with custom retry limit.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The specification ID to create the loop for
    /// * `max_retries` - Maximum number of retries per task
    pub fn with_max_retries(spec_id: SpecId, max_retries: u32) -> Self {
        let state = LoopState::new(spec_id);
        let state_file = PathBuf::from(".aad/loop-state.json");
        Self {
            state,
            state_file,
            max_retries,
        }
    }

    /// Runs the task execution loop.
    ///
    /// This method starts the loop and processes tasks from the queue until
    /// the queue is empty or the loop is stopped.
    ///
    /// # Arguments
    ///
    /// * `tasks` - List of tasks to execute
    ///
    /// # Returns
    ///
    /// Ok(()) if the loop completes successfully, or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::loop_engine::LoopEngine;
    /// use domain::value_objects::SpecId;
    ///
    /// # async fn example() -> application::Result<()> {
    /// let spec_id = SpecId::new();
    /// let mut engine = LoopEngine::new(spec_id);
    ///
    /// engine.run_loop(vec![]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_loop(&mut self, tasks: Vec<Task>) -> Result<()> {
        // Initialize queue with pending tasks
        let pending_tasks: Vec<TaskId> = tasks
            .iter()
            .filter(|t| t.status == Status::Pending)
            .map(|t| t.id.clone())
            .collect();

        self.state.enqueue_tasks(pending_tasks);
        self.state.start();
        self.save_state()?;

        // Main loop
        while !self.state.is_queue_empty() {
            if !self.state.is_active {
                // Loop is paused
                break;
            }

            // Get next task
            if let Some(task_id) = self.state.dequeue_task() {
                self.state.set_current_task(Some(task_id.clone()));
                self.save_state()?;

                // In a real implementation, this would execute the task
                // For now, we just mark the task as processed
                self.state.set_current_task(None);
                self.save_state()?;
            }
        }

        // If queue is empty, stop the loop
        if self.state.is_queue_empty() {
            self.state.stop();
            self.save_state()?;
        }

        Ok(())
    }

    /// Pauses the loop.
    ///
    /// The loop can be resumed later by calling `resume()`.
    pub fn pause(&mut self) -> Result<()> {
        self.state.pause();
        self.save_state()
    }

    /// Resumes the loop.
    pub fn resume(&mut self) -> Result<()> {
        self.state.resume();
        self.save_state()
    }

    /// Stops the loop.
    ///
    /// This clears the current task and marks the loop as inactive.
    pub fn stop(&mut self) -> Result<()> {
        self.state.stop();
        self.save_state()
    }

    /// Gets the current loop state.
    pub fn state(&self) -> &LoopState {
        &self.state
    }

    /// Gets a mutable reference to the current loop state.
    ///
    /// This is primarily for testing purposes.
    #[cfg(test)]
    pub(crate) fn state_mut(&mut self) -> &mut LoopState {
        &mut self.state
    }

    /// Gets the next task to execute, considering dependencies and retry limits.
    ///
    /// This method dequeues tasks from the queue and checks:
    /// 1. If the task has exceeded the retry limit
    /// 2. If all dependencies are completed
    ///
    /// If a task cannot be executed (dependencies not met), it is re-enqueued
    /// at the back of the queue.
    ///
    /// # Arguments
    ///
    /// * `tasks` - List of all tasks for this specification
    ///
    /// # Returns
    ///
    /// Ok(Some(task_id)) if a task is ready to execute,
    /// Ok(None) if no tasks are ready (queue empty or all blocked),
    /// Err if there's an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use application::loop_engine::LoopEngine;
    /// use domain::entities::Task;
    /// use domain::value_objects::{SpecId, Priority};
    ///
    /// # fn example() -> application::Result<()> {
    /// let spec_id = SpecId::new();
    /// let mut engine = LoopEngine::new(spec_id.clone());
    ///
    /// let task = Task::new(
    ///     spec_id,
    ///     "Task 1".to_string(),
    ///     "Description".to_string(),
    ///     Priority::Must,
    ///     "S".to_string(),
    /// )?;
    ///
    /// // Queue the task for execution
    /// // engine.enqueue_task(task.id.clone());
    /// // Get the next task to execute
    /// let next = engine.next_task(&[task])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn next_task(&mut self, tasks: &[Task]) -> Result<Option<TaskId>> {
        let mut checked_count = 0;
        let queue_size = self.state.pending_count();

        // Try to find a task that can be executed
        while checked_count < queue_size {
            if let Some(task_id) = self.state.dequeue_task() {
                // Check retry limit
                let retry_count = self.state.get_retry_count(&task_id);
                if retry_count >= self.max_retries {
                    // Task has exceeded retry limit, skip it
                    checked_count += 1;
                    continue;
                }

                // Find the task
                if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
                    // Check if task is already completed
                    if task.status == Status::Completed {
                        checked_count += 1;
                        continue;
                    }

                    // Check dependencies
                    let all_deps_completed = task.dependencies.iter().all(|dep_id| {
                        tasks
                            .iter()
                            .find(|t| &t.id == dep_id)
                            .is_some_and(|t| t.status == Status::Completed)
                    });

                    if all_deps_completed {
                        // All dependencies are met, return this task
                        return Ok(Some(task_id));
                    } else {
                        // Dependencies not met, re-enqueue at the back
                        self.state.enqueue_task(task_id);
                        checked_count += 1;
                    }
                } else {
                    // Task not found in the list, skip it
                    checked_count += 1;
                }
            } else {
                // Queue is empty
                break;
            }
        }

        // No task is ready to execute
        Ok(None)
    }

    /// Marks a task as failed and increments its retry count.
    ///
    /// # Arguments
    ///
    /// * `task_id` - ID of the task that failed
    pub fn mark_task_failed(&mut self, task_id: &TaskId) {
        self.state.increment_retry(task_id);
    }

    /// Loads the loop state from the state file.
    ///
    /// # Returns
    ///
    /// Ok(LoopEngine) if the state was loaded successfully, or an error.
    pub fn load(state_file: PathBuf) -> Result<Self> {
        if !state_file.exists() {
            return Err(ApplicationError::Validation(format!(
                "State file does not exist: {}",
                state_file.display()
            )));
        }

        let content = fs::read_to_string(&state_file).map_err(|e| {
            ApplicationError::Validation(format!("Failed to read state file: {}", e))
        })?;

        let state: LoopState = serde_json::from_str(&content).map_err(|e| {
            ApplicationError::Validation(format!("Failed to parse state file: {}", e))
        })?;

        Ok(Self {
            state,
            state_file,
            max_retries: 3,
        })
    }

    /// Saves the loop state to the state file.
    fn save_state(&self) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.state_file.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ApplicationError::Validation(format!("Failed to create directory: {}", e))
            })?;
        }

        let content = serde_json::to_string_pretty(&self.state).map_err(|e| {
            ApplicationError::Validation(format!("Failed to serialize state: {}", e))
        })?;

        fs::write(&self.state_file, content).map_err(|e| {
            ApplicationError::Validation(format!("Failed to write state file: {}", e))
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_objects::Priority;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_loop_engine_creation() {
        let spec_id = SpecId::new();
        let engine = LoopEngine::new(spec_id.clone());

        assert_eq!(engine.state().spec_id, spec_id);
        assert!(engine.state().is_queue_empty());
        assert!(!engine.state().is_active);
    }

    #[tokio::test]
    async fn test_loop_engine_run_loop_empty() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id);

        let result = engine.run_loop(vec![]).await;
        assert!(result.is_ok());
        assert!(engine.state().is_queue_empty());
        assert!(!engine.state().is_active);
    }

    #[tokio::test]
    async fn test_loop_engine_run_loop_with_tasks() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id.clone());

        let task1 = Task::new(
            spec_id.clone(),
            "Task 1".to_string(),
            "Description 1".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let task2 = Task::new(
            spec_id.clone(),
            "Task 2".to_string(),
            "Description 2".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let result = engine.run_loop(vec![task1, task2]).await;
        assert!(result.is_ok());
        assert!(engine.state().is_queue_empty());
        assert!(!engine.state().is_active);
    }

    #[tokio::test]
    async fn test_loop_engine_pause_resume() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id);

        // Start the engine (without actually running)
        engine.state_mut().start();
        assert!(engine.state().is_active);

        engine.pause().unwrap();
        assert!(!engine.state().is_active);

        engine.resume().unwrap();
        assert!(engine.state().is_active);
    }

    #[tokio::test]
    async fn test_loop_engine_stop() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id);

        engine.state_mut().start();
        engine.state_mut().set_current_task(Some(TaskId::new()));

        engine.stop().unwrap();
        assert!(!engine.state().is_active);
        assert!(engine.state().current_task.is_none());
    }

    #[tokio::test]
    async fn test_loop_engine_save_load_state() {
        let temp_dir = TempDir::new().unwrap();
        let state_file = temp_dir.path().join("loop-state.json");

        let spec_id = SpecId::new();
        let mut engine = LoopEngine::with_state_file(spec_id.clone(), state_file.clone());

        // Add some tasks to the queue
        let task_id1 = TaskId::new();
        let task_id2 = TaskId::new();
        engine.state_mut().enqueue_task(task_id1.clone());
        engine.state_mut().enqueue_task(task_id2.clone());
        engine.state_mut().start();

        // Save state
        engine.save_state().unwrap();
        assert!(state_file.exists());

        // Load state
        let loaded_engine = LoopEngine::load(state_file).unwrap();
        assert_eq!(loaded_engine.state().spec_id, spec_id);
        assert_eq!(loaded_engine.state().pending_count(), 2);
        assert!(loaded_engine.state().is_active);
    }

    #[tokio::test]
    async fn test_loop_engine_load_nonexistent_file() {
        let state_file = PathBuf::from("/nonexistent/path/loop-state.json");
        let result = LoopEngine::load(state_file);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_loop_engine_state_access() {
        let spec_id = SpecId::new();
        let engine = LoopEngine::new(spec_id.clone());

        let state = engine.state();
        assert_eq!(state.spec_id, spec_id);
        assert!(state.is_queue_empty());
    }

    #[tokio::test]
    async fn test_loop_engine_filters_non_pending_tasks() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id.clone());

        let mut task1 = Task::new(
            spec_id.clone(),
            "Task 1".to_string(),
            "Description 1".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let task2 = Task::new(
            spec_id.clone(),
            "Task 2".to_string(),
            "Description 2".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        // Mark task1 as completed
        task1.change_status(Status::Completed);

        let result = engine.run_loop(vec![task1, task2]).await;
        assert!(result.is_ok());

        // Only task2 should have been processed (task1 was completed)
        assert!(engine.state().is_queue_empty());
    }

    #[tokio::test]
    async fn test_next_task_no_dependencies() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id.clone());

        let task1 = Task::new(
            spec_id.clone(),
            "Task 1".to_string(),
            "Description 1".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let task2 = Task::new(
            spec_id.clone(),
            "Task 2".to_string(),
            "Description 2".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let tasks = vec![task1.clone(), task2.clone()];

        // Enqueue both tasks
        engine.state_mut().enqueue_task(task1.id.clone());
        engine.state_mut().enqueue_task(task2.id.clone());

        // Get next task
        let next = engine.next_task(&tasks).unwrap();
        assert_eq!(next, Some(task1.id));
    }

    #[tokio::test]
    async fn test_next_task_with_dependencies_completed() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id.clone());

        let task1 = Task::new(
            spec_id.clone(),
            "Task 1".to_string(),
            "Description 1".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let mut task2 = Task::new(
            spec_id.clone(),
            "Task 2".to_string(),
            "Description 2".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        // task2 depends on task1
        task2.add_dependency(task1.id.clone()).unwrap();

        // Mark task1 as completed
        let mut task1_completed = task1.clone();
        task1_completed.change_status(Status::Completed);

        let tasks = vec![task1_completed, task2.clone()];

        // Enqueue task2
        engine.state_mut().enqueue_task(task2.id.clone());

        // Get next task (should be task2 since task1 is completed)
        let next = engine.next_task(&tasks).unwrap();
        assert_eq!(next, Some(task2.id));
    }

    #[tokio::test]
    async fn test_next_task_with_dependencies_not_completed() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id.clone());

        let task1 = Task::new(
            spec_id.clone(),
            "Task 1".to_string(),
            "Description 1".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let mut task2 = Task::new(
            spec_id.clone(),
            "Task 2".to_string(),
            "Description 2".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        // task2 depends on task1
        task2.add_dependency(task1.id.clone()).unwrap();

        let tasks = vec![task1.clone(), task2.clone()];

        // Enqueue task2 (but task1 is not completed)
        engine.state_mut().enqueue_task(task2.id.clone());

        // Get next task (should skip task2 and return None)
        let next = engine.next_task(&tasks).unwrap();
        assert_eq!(next, None);

        // task2 should be re-enqueued at the back
        assert!(engine.state().contains_task(&task2.id));
    }

    #[tokio::test]
    async fn test_next_task_all_completed() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id.clone());

        let mut task1 = Task::new(
            spec_id.clone(),
            "Task 1".to_string(),
            "Description 1".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        task1.change_status(Status::Completed);

        let tasks = vec![task1.clone()];

        // Enqueue task1 (even though it's completed)
        engine.state_mut().enqueue_task(task1.id.clone());

        // Get next task (should return None as all are completed)
        let next = engine.next_task(&tasks).unwrap();
        assert_eq!(next, None);
    }

    #[tokio::test]
    async fn test_next_task_retry_limit() {
        let spec_id = SpecId::new();
        let mut engine = LoopEngine::new(spec_id.clone());

        let task1 = Task::new(
            spec_id.clone(),
            "Task 1".to_string(),
            "Description 1".to_string(),
            Priority::Must,
            "S".to_string(),
        )
        .unwrap();

        let tasks = vec![task1.clone()];

        // Enqueue task1
        engine.state_mut().enqueue_task(task1.id.clone());

        // Simulate 3 retries
        for _ in 0..3 {
            let next = engine.next_task(&tasks).unwrap();
            assert_eq!(next, Some(task1.id.clone()));

            // Mark as failed and re-enqueue
            engine.mark_task_failed(&task1.id);
            engine.state_mut().enqueue_task(task1.id.clone());
        }

        // 4th attempt should fail (exceeded retry limit)
        let next = engine.next_task(&tasks).unwrap();
        assert_eq!(next, None);
    }
}
