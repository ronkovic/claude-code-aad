//! Loop state entity.

use crate::value_objects::{SpecId, TaskId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Loop state entity.
///
/// Represents the state of the task execution loop for a specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopState {
    /// ID of the specification this loop is for.
    pub spec_id: SpecId,
    /// Queue of pending task IDs.
    pub task_queue: VecDeque<TaskId>,
    /// Currently executing task ID.
    pub current_task: Option<TaskId>,
    /// Whether the loop is currently active.
    pub is_active: bool,
    /// Retry counts for each task.
    pub retry_counts: HashMap<TaskId, u32>,
    /// When this loop state was created.
    pub created_at: DateTime<Utc>,
    /// When this loop state was last updated.
    pub updated_at: DateTime<Utc>,
}

impl LoopState {
    /// Creates a new loop state.
    pub fn new(spec_id: SpecId) -> Self {
        let now = Utc::now();
        Self {
            spec_id,
            task_queue: VecDeque::new(),
            current_task: None,
            is_active: false,
            retry_counts: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Increments the retry count for a task.
    ///
    /// Returns the new retry count.
    pub fn increment_retry(&mut self, task_id: &TaskId) -> u32 {
        let count = self.retry_counts.entry(task_id.clone()).or_insert(0);
        *count += 1;
        self.updated_at = Utc::now();
        *count
    }

    /// Gets the retry count for a task.
    pub fn get_retry_count(&self, task_id: &TaskId) -> u32 {
        self.retry_counts.get(task_id).copied().unwrap_or(0)
    }

    /// Clears the retry count for a task.
    pub fn clear_retry(&mut self, task_id: &TaskId) {
        self.retry_counts.remove(task_id);
        self.updated_at = Utc::now();
    }

    /// Enqueues a task.
    pub fn enqueue_task(&mut self, task_id: TaskId) {
        // Only add if not already in queue
        if !self.task_queue.contains(&task_id) {
            self.task_queue.push_back(task_id);
            self.updated_at = Utc::now();
        }
    }

    /// Enqueues multiple tasks.
    pub fn enqueue_tasks(&mut self, task_ids: Vec<TaskId>) {
        for task_id in task_ids {
            self.enqueue_task(task_id);
        }
    }

    /// Dequeues the next task.
    pub fn dequeue_task(&mut self) -> Option<TaskId> {
        let task_id = self.task_queue.pop_front();
        if task_id.is_some() {
            self.updated_at = Utc::now();
        }
        task_id
    }

    /// Sets the current task.
    pub fn set_current_task(&mut self, task_id: Option<TaskId>) {
        self.current_task = task_id;
        self.updated_at = Utc::now();
    }

    /// Starts the loop.
    pub fn start(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Stops the loop.
    pub fn stop(&mut self) {
        self.is_active = false;
        self.current_task = None;
        self.updated_at = Utc::now();
    }

    /// Pauses the loop.
    pub fn pause(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Resumes the loop.
    pub fn resume(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Checks if the queue is empty.
    pub fn is_queue_empty(&self) -> bool {
        self.task_queue.is_empty()
    }

    /// Gets the number of pending tasks.
    pub fn pending_count(&self) -> usize {
        self.task_queue.len()
    }

    /// Clears the queue.
    pub fn clear_queue(&mut self) {
        self.task_queue.clear();
        self.updated_at = Utc::now();
    }

    /// Checks if a task is in the queue.
    pub fn contains_task(&self, task_id: &TaskId) -> bool {
        self.task_queue.contains(task_id)
    }

    /// Gets the next task without dequeuing.
    pub fn peek_next_task(&self) -> Option<&TaskId> {
        self.task_queue.front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_state_creation() {
        let spec_id = SpecId::new();
        let state = LoopState::new(spec_id.clone());

        assert_eq!(state.spec_id, spec_id);
        assert!(state.task_queue.is_empty());
        assert!(state.current_task.is_none());
        assert!(!state.is_active);
    }

    #[test]
    fn test_loop_state_enqueue_task() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        let task_id = TaskId::new();
        state.enqueue_task(task_id.clone());

        assert_eq!(state.task_queue.len(), 1);
        assert_eq!(state.task_queue[0], task_id);

        // Duplicate enqueue should be ignored
        state.enqueue_task(task_id.clone());
        assert_eq!(state.task_queue.len(), 1);
    }

    #[test]
    fn test_loop_state_enqueue_tasks() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        let task_ids = vec![TaskId::new(), TaskId::new(), TaskId::new()];
        state.enqueue_tasks(task_ids.clone());

        assert_eq!(state.task_queue.len(), 3);
        assert_eq!(state.task_queue[0], task_ids[0]);
        assert_eq!(state.task_queue[1], task_ids[1]);
        assert_eq!(state.task_queue[2], task_ids[2]);
    }

    #[test]
    fn test_loop_state_dequeue_task() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        let task_id1 = TaskId::new();
        let task_id2 = TaskId::new();

        state.enqueue_task(task_id1.clone());
        state.enqueue_task(task_id2.clone());

        assert_eq!(state.dequeue_task(), Some(task_id1));
        assert_eq!(state.dequeue_task(), Some(task_id2));
        assert_eq!(state.dequeue_task(), None);
    }

    #[test]
    fn test_loop_state_set_current_task() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        assert!(state.current_task.is_none());

        let task_id = TaskId::new();
        state.set_current_task(Some(task_id.clone()));
        assert_eq!(state.current_task, Some(task_id));

        state.set_current_task(None);
        assert!(state.current_task.is_none());
    }

    #[test]
    fn test_loop_state_start_stop() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        assert!(!state.is_active);

        state.start();
        assert!(state.is_active);

        state.stop();
        assert!(!state.is_active);
        assert!(state.current_task.is_none());
    }

    #[test]
    fn test_loop_state_pause_resume() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        state.start();
        assert!(state.is_active);

        state.pause();
        assert!(!state.is_active);

        state.resume();
        assert!(state.is_active);
    }

    #[test]
    fn test_loop_state_is_queue_empty() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        assert!(state.is_queue_empty());

        state.enqueue_task(TaskId::new());
        assert!(!state.is_queue_empty());

        state.dequeue_task();
        assert!(state.is_queue_empty());
    }

    #[test]
    fn test_loop_state_pending_count() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        assert_eq!(state.pending_count(), 0);

        state.enqueue_task(TaskId::new());
        assert_eq!(state.pending_count(), 1);

        state.enqueue_task(TaskId::new());
        assert_eq!(state.pending_count(), 2);

        state.dequeue_task();
        assert_eq!(state.pending_count(), 1);
    }

    #[test]
    fn test_loop_state_clear_queue() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        state.enqueue_task(TaskId::new());
        state.enqueue_task(TaskId::new());
        assert_eq!(state.pending_count(), 2);

        state.clear_queue();
        assert!(state.is_queue_empty());
    }

    #[test]
    fn test_loop_state_contains_task() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        let task_id1 = TaskId::new();
        let task_id2 = TaskId::new();

        state.enqueue_task(task_id1.clone());
        assert!(state.contains_task(&task_id1));
        assert!(!state.contains_task(&task_id2));
    }

    #[test]
    fn test_loop_state_peek_next_task() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        assert!(state.peek_next_task().is_none());

        let task_id = TaskId::new();
        state.enqueue_task(task_id.clone());
        assert_eq!(state.peek_next_task(), Some(&task_id));

        // Peek should not remove the task
        assert_eq!(state.peek_next_task(), Some(&task_id));
        assert_eq!(state.pending_count(), 1);
    }

    #[test]
    fn test_loop_state_clone() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);
        state.enqueue_task(TaskId::new());

        let cloned = state.clone();
        assert_eq!(state.spec_id, cloned.spec_id);
        assert_eq!(state.task_queue, cloned.task_queue);
        assert_eq!(state.is_active, cloned.is_active);
    }

    #[test]
    fn test_loop_state_retry_count() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);
        let task_id = TaskId::new();

        assert_eq!(state.get_retry_count(&task_id), 0);

        let count1 = state.increment_retry(&task_id);
        assert_eq!(count1, 1);
        assert_eq!(state.get_retry_count(&task_id), 1);

        let count2 = state.increment_retry(&task_id);
        assert_eq!(count2, 2);
        assert_eq!(state.get_retry_count(&task_id), 2);

        state.clear_retry(&task_id);
        assert_eq!(state.get_retry_count(&task_id), 0);
    }

    #[test]
    fn test_loop_state_multiple_task_retries() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);
        let task_id1 = TaskId::new();
        let task_id2 = TaskId::new();

        state.increment_retry(&task_id1);
        state.increment_retry(&task_id1);
        state.increment_retry(&task_id2);

        assert_eq!(state.get_retry_count(&task_id1), 2);
        assert_eq!(state.get_retry_count(&task_id2), 1);
    }
}
