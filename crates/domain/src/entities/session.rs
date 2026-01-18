//! Session entity.

use crate::value_objects::{Phase, SpecId, TaskId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Session entity.
///
/// Represents a work session, tracking context usage and phase progression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique identifier for this session.
    pub id: String,
    /// Associated specification ID.
    pub spec_id: SpecId,
    /// Associated task ID (if working on a specific task).
    pub task_id: Option<TaskId>,
    /// Current phase of work.
    pub phase: Phase,
    /// When this session started.
    pub started_at: DateTime<Utc>,
    /// When this session ended (None if still active).
    pub ended_at: Option<DateTime<Utc>>,
    /// Context usage ratio (0.0 - 1.0).
    pub context_usage: f32,
}

impl Session {
    /// Context usage threshold for warnings (70%).
    pub const CONTEXT_THRESHOLD: f32 = 0.70;

    /// Creates a new session.
    pub fn new(spec_id: SpecId, phase: Phase) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            spec_id,
            task_id: None,
            phase,
            started_at: Utc::now(),
            ended_at: None,
            context_usage: 0.0,
        }
    }

    /// Ends this session.
    pub fn end(&mut self) {
        if self.ended_at.is_none() {
            self.ended_at = Some(Utc::now());
        }
    }

    /// Updates the context usage.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not between 0.0 and 1.0.
    pub fn update_context_usage(&mut self, usage: f32) -> Result<(), crate::DomainError> {
        if !(0.0..=1.0).contains(&usage) {
            return Err(crate::DomainError::ValidationError(
                "Context usage must be between 0.0 and 1.0".to_string(),
            ));
        }
        self.context_usage = usage;
        Ok(())
    }

    /// Checks if context usage exceeds the threshold.
    pub fn is_over_threshold(&self) -> bool {
        self.context_usage >= Self::CONTEXT_THRESHOLD
    }

    /// Gets the duration of this session.
    pub fn duration(&self) -> chrono::Duration {
        let end = self.ended_at.unwrap_or_else(Utc::now);
        end - self.started_at
    }

    /// Checks if this session is still active.
    pub fn is_active(&self) -> bool {
        self.ended_at.is_none()
    }

    /// Sets the task ID for this session.
    pub fn set_task(&mut self, task_id: Option<TaskId>) {
        self.task_id = task_id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let spec_id = SpecId::new();
        let session = Session::new(spec_id.clone(), Phase::Tdd);

        assert_eq!(session.spec_id, spec_id);
        assert_eq!(session.phase, Phase::Tdd);
        assert!(session.task_id.is_none());
        assert!(session.ended_at.is_none());
        assert_eq!(session.context_usage, 0.0);
    }

    #[test]
    fn test_session_end() {
        let spec_id = SpecId::new();
        let mut session = Session::new(spec_id, Phase::Tdd);

        assert!(session.is_active());
        assert!(session.ended_at.is_none());

        session.end();
        assert!(!session.is_active());
        assert!(session.ended_at.is_some());

        // Calling end again should be idempotent
        let first_end = session.ended_at;
        session.end();
        assert_eq!(session.ended_at, first_end);
    }

    #[test]
    fn test_session_update_context_usage() {
        let spec_id = SpecId::new();
        let mut session = Session::new(spec_id, Phase::Tdd);

        session.update_context_usage(0.5).unwrap();
        assert_eq!(session.context_usage, 0.5);

        session.update_context_usage(0.0).unwrap();
        assert_eq!(session.context_usage, 0.0);

        session.update_context_usage(1.0).unwrap();
        assert_eq!(session.context_usage, 1.0);
    }

    #[test]
    fn test_session_update_context_usage_out_of_range() {
        let spec_id = SpecId::new();
        let mut session = Session::new(spec_id, Phase::Tdd);

        assert!(session.update_context_usage(-0.1).is_err());
        assert!(session.update_context_usage(1.1).is_err());
    }

    #[test]
    fn test_session_threshold_check() {
        let spec_id = SpecId::new();
        let mut session = Session::new(spec_id, Phase::Tdd);

        session.update_context_usage(0.6).unwrap();
        assert!(!session.is_over_threshold());

        session.update_context_usage(0.7).unwrap();
        assert!(session.is_over_threshold());

        session.update_context_usage(0.8).unwrap();
        assert!(session.is_over_threshold());
    }

    #[test]
    fn test_session_duration() {
        let spec_id = SpecId::new();
        let session = Session::new(spec_id, Phase::Tdd);

        std::thread::sleep(std::time::Duration::from_millis(10));

        let duration = session.duration();
        assert!(duration.num_milliseconds() >= 10);
    }

    #[test]
    fn test_session_set_task() {
        let spec_id = SpecId::new();
        let mut session = Session::new(spec_id, Phase::Tdd);

        assert!(session.task_id.is_none());

        let task_id = TaskId::new();
        session.set_task(Some(task_id.clone()));
        assert_eq!(session.task_id, Some(task_id));

        session.set_task(None);
        assert!(session.task_id.is_none());
    }

    #[test]
    fn test_session_clone() {
        let spec_id = SpecId::new();
        let session = Session::new(spec_id, Phase::Tdd);

        let cloned = session.clone();
        assert_eq!(session.id, cloned.id);
        assert_eq!(session.spec_id, cloned.spec_id);
    }
}
