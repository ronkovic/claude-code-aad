//! Session monitoring and progress tracking for the orchestrator.
//!
//! This module provides types and utilities for monitoring session status,
//! tracking progress, and logging orchestration events.

use super::orchestrator::SessionId;

/// Status of a session in the orchestrator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    /// Session is waiting to be started.
    Pending,
    /// Session is currently running.
    Running,
    /// Session completed successfully.
    Completed,
    /// Session timed out.
    TimedOut,
    /// Session failed with an error.
    Failed,
}

impl SessionStatus {
    /// Returns true if this status is a terminal state (session is done).
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            SessionStatus::Completed | SessionStatus::TimedOut | SessionStatus::Failed
        )
    }
}

/// Events that occur during session monitoring.
#[derive(Debug, Clone)]
pub enum MonitorEvent {
    /// A session was started.
    SessionStarted {
        /// ID of the started session.
        session_id: SessionId,
        /// Spec ID associated with the session.
        spec_id: String,
    },
    /// A session completed successfully.
    SessionCompleted {
        /// ID of the completed session.
        session_id: SessionId,
        /// Duration in seconds.
        duration_secs: u64,
    },
    /// A session timed out.
    SessionTimedOut {
        /// ID of the timed-out session.
        session_id: SessionId,
        /// Timeout threshold in seconds.
        timeout_secs: u64,
    },
    /// A session failed.
    SessionFailed {
        /// ID of the failed session.
        session_id: SessionId,
        /// Failure reason.
        reason: String,
    },
    /// Progress update for all sessions.
    ProgressUpdate {
        /// Number of completed sessions.
        completed: usize,
        /// Total number of sessions.
        total: usize,
        /// Completion percentage (0-100).
        percent: u8,
    },
}

/// Progress statistics for all sessions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorProgress {
    /// Total number of sessions.
    pub total_sessions: usize,
    /// Number of completed sessions.
    pub completed_sessions: usize,
    /// Number of failed sessions.
    pub failed_sessions: usize,
    /// Number of timed-out sessions.
    pub timed_out_sessions: usize,
    /// Number of currently running sessions.
    pub running_sessions: usize,
    /// Number of pending sessions.
    pub pending_sessions: usize,
}

impl MonitorProgress {
    /// Creates a new progress tracker with all counters set to zero.
    pub fn new() -> Self {
        Self {
            total_sessions: 0,
            completed_sessions: 0,
            failed_sessions: 0,
            timed_out_sessions: 0,
            running_sessions: 0,
            pending_sessions: 0,
        }
    }

    /// Calculates the overall progress percentage (0-100).
    ///
    /// Progress is based on terminal states (completed + failed + timed-out).
    pub fn progress_percent(&self) -> u8 {
        if self.total_sessions == 0 {
            return 100;
        }

        let terminal_count =
            self.completed_sessions + self.failed_sessions + self.timed_out_sessions;
        ((terminal_count * 100) / self.total_sessions) as u8
    }

    /// Returns true if all sessions are in a terminal state.
    pub fn all_terminal(&self) -> bool {
        self.total_sessions > 0
            && (self.completed_sessions + self.failed_sessions + self.timed_out_sessions
                == self.total_sessions)
    }
}

impl Default for MonitorProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Logging utilities for orchestration events.
pub mod log {
    use super::MonitorEvent;

    /// Logs an informational message.
    pub fn info(message: &str) {
        println!("[INFO] {}", message);
    }

    /// Logs a warning message.
    pub fn warn(message: &str) {
        eprintln!("[WARN] {}", message);
    }

    /// Logs an error message.
    pub fn error(message: &str) {
        eprintln!("[ERROR] {}", message);
    }

    /// Logs a progress message.
    pub fn progress(message: &str) {
        println!("[PROGRESS] {}", message);
    }

    /// Logs a monitor event with appropriate formatting.
    pub fn event(event: &MonitorEvent) {
        match event {
            MonitorEvent::SessionStarted {
                session_id,
                spec_id,
            } => {
                info(&format!(
                    "Session started: {} (spec: {})",
                    session_id, spec_id
                ));
            }
            MonitorEvent::SessionCompleted {
                session_id,
                duration_secs,
            } => {
                info(&format!(
                    "Session completed: {} (duration: {}s)",
                    session_id, duration_secs
                ));
            }
            MonitorEvent::SessionTimedOut {
                session_id,
                timeout_secs,
            } => {
                warn(&format!(
                    "Session timed out: {} (timeout: {}s)",
                    session_id, timeout_secs
                ));
            }
            MonitorEvent::SessionFailed { session_id, reason } => {
                error(&format!(
                    "Session failed: {} (reason: {})",
                    session_id, reason
                ));
            }
            MonitorEvent::ProgressUpdate {
                completed,
                total,
                percent,
            } => {
                progress(&format!(
                    "Overall progress: {}/{} ({}%)",
                    completed, total, percent
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_status_is_terminal() {
        assert!(!SessionStatus::Pending.is_terminal());
        assert!(!SessionStatus::Running.is_terminal());
        assert!(SessionStatus::Completed.is_terminal());
        assert!(SessionStatus::TimedOut.is_terminal());
        assert!(SessionStatus::Failed.is_terminal());
    }

    #[test]
    fn test_monitor_progress_new() {
        let progress = MonitorProgress::new();
        assert_eq!(progress.total_sessions, 0);
        assert_eq!(progress.completed_sessions, 0);
        assert_eq!(progress.failed_sessions, 0);
        assert_eq!(progress.timed_out_sessions, 0);
        assert_eq!(progress.running_sessions, 0);
        assert_eq!(progress.pending_sessions, 0);
    }

    #[test]
    fn test_monitor_progress_percent_empty() {
        let progress = MonitorProgress::new();
        assert_eq!(progress.progress_percent(), 100);
    }

    #[test]
    fn test_monitor_progress_percent() {
        let progress = MonitorProgress {
            total_sessions: 10,
            completed_sessions: 5,
            failed_sessions: 2,
            timed_out_sessions: 1,
            running_sessions: 2,
            pending_sessions: 0,
        };
        // (5 + 2 + 1) / 10 * 100 = 80%
        assert_eq!(progress.progress_percent(), 80);
    }

    #[test]
    fn test_monitor_progress_all_terminal() {
        let progress = MonitorProgress {
            total_sessions: 5,
            completed_sessions: 3,
            failed_sessions: 1,
            timed_out_sessions: 1,
            running_sessions: 0,
            pending_sessions: 0,
        };
        assert!(progress.all_terminal());
    }

    #[test]
    fn test_monitor_progress_not_all_terminal() {
        let progress = MonitorProgress {
            total_sessions: 5,
            completed_sessions: 3,
            failed_sessions: 1,
            timed_out_sessions: 0,
            running_sessions: 1,
            pending_sessions: 0,
        };
        assert!(!progress.all_terminal());
    }

    #[test]
    fn test_monitor_progress_empty_not_terminal() {
        let progress = MonitorProgress::new();
        assert!(!progress.all_terminal());
    }
}
