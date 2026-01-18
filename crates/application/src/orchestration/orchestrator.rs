//! Core orchestrator for managing multiple sessions.

use super::monitor::{MonitorEvent, MonitorProgress, SessionStatus};
use super::{DependencyGraph, OrchestratorConfig};
use crate::error::{ApplicationError, Result};
use domain::entities::Session;
use domain::value_objects::{Phase, SpecId};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// SessionId type alias.
///
/// Represents a unique identifier for a session.
pub type SessionId = String;

/// Core orchestrator for managing multiple sessions.
///
/// The orchestrator is responsible for:
/// - Managing session lifecycle (create, start, stop)
/// - Parallel execution of sessions
/// - Monitoring session progress
/// - Handling escalations
///
/// # Tokio Runtime Integration
///
/// The orchestrator uses tokio for async operations and must be used
/// within a tokio runtime context.
#[derive(Debug)]
pub struct Orchestrator {
    /// Configuration for the orchestrator.
    config: OrchestratorConfig,

    /// Active sessions, keyed by session ID.
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,

    /// Session statuses for monitoring.
    session_statuses: Arc<RwLock<HashMap<SessionId, SessionStatus>>>,

    /// Session start times for timeout tracking.
    session_start_times: Arc<RwLock<HashMap<SessionId, Instant>>>,

    /// Dependency graph for managing spec execution order.
    dependency_graph: Arc<RwLock<DependencyGraph>>,
}

impl Orchestrator {
    /// Creates a new orchestrator with the given configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::orchestration::{Orchestrator, OrchestratorConfig};
    ///
    /// let config = OrchestratorConfig::default();
    /// let orchestrator = Orchestrator::new(config);
    /// ```
    pub fn new(config: OrchestratorConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_statuses: Arc::new(RwLock::new(HashMap::new())),
            session_start_times: Arc::new(RwLock::new(HashMap::new())),
            dependency_graph: Arc::new(RwLock::new(DependencyGraph::new())),
        }
    }

    /// Adds a new session to the orchestrator.
    ///
    /// # Arguments
    ///
    /// * `session` - The session to add
    ///
    /// # Returns
    ///
    /// The session ID if successful, or an error if a session with the same ID already exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::orchestration::{Orchestrator, OrchestratorConfig};
    /// use domain::entities::Session;
    /// use domain::value_objects::{SpecId, Phase};
    ///
    /// let config = OrchestratorConfig::default();
    /// let mut orchestrator = Orchestrator::new(config);
    ///
    /// let session = Session::new(SpecId::new(), Phase::Tdd);
    /// let session_id = orchestrator.add_session(session);
    /// ```
    pub async fn add_session(&self, session: Session) -> Result<SessionId> {
        let session_id = session.id.clone();
        let mut sessions = self.sessions.write().await;

        if sessions.contains_key(&session_id) {
            return Err(ApplicationError::SessionAlreadyExists(session_id));
        }

        sessions.insert(session_id.clone(), session);
        drop(sessions);

        // Register session status as Pending
        let mut statuses = self.session_statuses.write().await;
        statuses.insert(session_id.clone(), SessionStatus::Pending);

        Ok(session_id)
    }

    /// Removes a session from the orchestrator.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The ID of the session to remove
    ///
    /// # Returns
    ///
    /// The removed session if it existed, or None if it did not.
    pub async fn remove_session(&self, session_id: &SessionId) -> Option<Session> {
        let mut sessions = self.sessions.write().await;
        let removed_session = sessions.remove(session_id);
        drop(sessions);

        // Also remove from statuses and start times
        let mut statuses = self.session_statuses.write().await;
        statuses.remove(session_id);
        drop(statuses);

        let mut start_times = self.session_start_times.write().await;
        start_times.remove(session_id);

        removed_session
    }

    /// Gets a session by its ID.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The ID of the session to retrieve
    ///
    /// # Returns
    ///
    /// A clone of the session if it exists, or None if it does not.
    pub async fn get_session(&self, session_id: &SessionId) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Gets all active sessions.
    ///
    /// # Returns
    ///
    /// A vector of cloned sessions.
    pub async fn get_all_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }

    /// Gets the number of active sessions.
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }

    /// Gets the orchestrator configuration.
    pub fn config(&self) -> &OrchestratorConfig {
        &self.config
    }

    /// Registers a Spec with the orchestrator.
    ///
    /// Creates a session for the spec and adds it to the dependency graph.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The ID of the spec to register
    /// * `phase` - The initial phase for the session
    ///
    /// # Returns
    ///
    /// The session ID if successful.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use application::orchestration::{Orchestrator, OrchestratorConfig};
    /// use domain::value_objects::{SpecId, Phase};
    ///
    /// # async {
    /// let config = OrchestratorConfig::default();
    /// let orchestrator = Orchestrator::new(config);
    ///
    /// let spec_id = SpecId::new();
    /// let session_id = orchestrator.register_spec(&spec_id, Phase::Tdd).await.unwrap();
    /// # };
    /// ```
    pub async fn register_spec(&self, spec_id: &SpecId, phase: Phase) -> Result<SessionId> {
        // Create a new session for the spec
        let session = Session::new(spec_id.clone(), phase);
        let session_id = session.id.clone();

        // Add session to the orchestrator
        self.add_session(session).await?;

        // Add spec to dependency graph (with no dependencies initially)
        // We add a dummy dependency then remove it to ensure the node exists
        let mut graph = self.dependency_graph.write().await;
        let spec_id_str = spec_id.to_string();
        graph.add_dependency(&spec_id_str, "")?;
        graph.remove_dependency(&spec_id_str, "");

        Ok(session_id)
    }

    /// Registers a Spec with dependencies.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The ID of the spec to register
    /// * `phase` - The initial phase for the session
    /// * `depends_on` - List of spec IDs this spec depends on
    ///
    /// # Returns
    ///
    /// The session ID if successful, or an error if a cyclic dependency is detected.
    pub async fn register_spec_with_dependencies(
        &self,
        spec_id: &SpecId,
        phase: Phase,
        depends_on: &[SpecId],
    ) -> Result<SessionId> {
        // Create a new session for the spec
        let session = Session::new(spec_id.clone(), phase);
        let session_id = session.id.clone();

        // Add session to the orchestrator
        self.add_session(session).await?;

        // Add spec and dependencies to dependency graph
        let mut graph = self.dependency_graph.write().await;
        for dependency in depends_on {
            graph.add_dependency(&spec_id.to_string(), &dependency.to_string())?;
        }

        Ok(session_id)
    }

    /// Starts a session.
    ///
    /// In a full implementation, this would launch a Child Session process.
    /// For now, this is a placeholder that validates the session exists.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The ID of the session to start
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, or an error if the session doesn't exist.
    pub async fn start_session(&self, session_id: &SessionId) -> Result<()> {
        let sessions = self.sessions.read().await;

        if !sessions.contains_key(session_id) {
            return Err(ApplicationError::Validation(format!(
                "Session not found: {}",
                session_id
            )));
        }
        drop(sessions);

        // Mark session as Running and record start time
        let mut statuses = self.session_statuses.write().await;
        statuses.insert(session_id.clone(), SessionStatus::Running);
        drop(statuses);

        let mut start_times = self.session_start_times.write().await;
        start_times.insert(session_id.clone(), Instant::now());

        // TODO: Launch Child Session process
        // For now, we just validate that the session exists

        Ok(())
    }

    /// Starts all registered sessions in dependency order.
    ///
    /// Uses the dependency graph to determine execution order via topological sort.
    /// Sessions with no dependencies are started first, followed by dependent sessions.
    ///
    /// # Returns
    ///
    /// A vector of session IDs in the order they were started.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use application::orchestration::{Orchestrator, OrchestratorConfig};
    /// use domain::value_objects::{SpecId, Phase};
    ///
    /// # async {
    /// let config = OrchestratorConfig::default();
    /// let orchestrator = Orchestrator::new(config);
    ///
    /// let spec1 = SpecId::new();
    /// let spec2 = SpecId::new();
    ///
    /// orchestrator.register_spec(&spec1, Phase::Tdd).await.unwrap();
    /// orchestrator.register_spec_with_dependencies(&spec2, Phase::Tdd, &[spec1.clone()]).await.unwrap();
    ///
    /// let order = orchestrator.start_all_sessions().await.unwrap();
    /// // spec1 should be started before spec2
    /// # };
    /// ```
    pub async fn start_all_sessions(&self) -> Result<Vec<SessionId>> {
        // Get topological sort order from dependency graph
        let graph = self.dependency_graph.read().await;
        let execution_order = graph.topological_sort()?;
        drop(graph);

        let mut started_sessions = Vec::new();

        // Start sessions in order
        for spec_id in execution_order {
            // Find the session for this spec
            let sessions = self.sessions.read().await;
            let session_id = sessions
                .values()
                .find(|s| s.spec_id.to_string() == spec_id)
                .map(|s| s.id.clone());
            drop(sessions);

            if let Some(session_id) = session_id {
                self.start_session(&session_id).await?;
                started_sessions.push(session_id);
            }
        }

        Ok(started_sessions)
    }

    /// Gets parallel execution groups from the dependency graph.
    ///
    /// Returns "waves" of execution where each wave contains specs
    /// that can run in parallel.
    ///
    /// # Returns
    ///
    /// A vector of waves, where each wave is a vector of spec IDs.
    pub async fn get_parallel_execution_groups(&self) -> Result<Vec<Vec<String>>> {
        let graph = self.dependency_graph.read().await;
        graph.get_parallel_groups()
    }

    /// Marks a session as completed.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The ID of the session to mark as completed
    pub async fn mark_session_completed(&self, session_id: &SessionId) {
        let mut statuses = self.session_statuses.write().await;
        statuses.insert(session_id.clone(), SessionStatus::Completed);
    }

    /// Marks a session as failed.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The ID of the session to mark as failed
    pub async fn mark_session_failed(&self, session_id: &SessionId) {
        let mut statuses = self.session_statuses.write().await;
        statuses.insert(session_id.clone(), SessionStatus::Failed);
    }

    /// Gets the status of a session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The ID of the session
    ///
    /// # Returns
    ///
    /// The session status, or None if the session doesn't exist.
    pub async fn get_session_status(&self, session_id: &SessionId) -> Option<SessionStatus> {
        let statuses = self.session_statuses.read().await;
        statuses.get(session_id).copied()
    }

    /// Determines the current status of a session based on its runtime.
    ///
    /// Checks if the session has timed out based on the configured timeout.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The ID of the session
    ///
    /// # Returns
    ///
    /// The determined session status.
    async fn determine_session_status(&self, session_id: &SessionId) -> SessionStatus {
        let statuses = self.session_statuses.read().await;
        let current_status = statuses.get(session_id).copied();
        drop(statuses);

        // If already in a terminal state, return it
        if let Some(status) = current_status {
            if status.is_terminal() {
                return status;
            }
        }

        // Check for timeout
        let start_times = self.session_start_times.read().await;
        if let Some(start_time) = start_times.get(session_id) {
            let elapsed = start_time.elapsed();
            let timeout_duration = Duration::from_secs(self.config.session_timeout_secs);

            if elapsed >= timeout_duration {
                drop(start_times);
                // Mark as timed out
                let mut statuses = self.session_statuses.write().await;
                statuses.insert(session_id.clone(), SessionStatus::TimedOut);
                return SessionStatus::TimedOut;
            }
        }

        current_status.unwrap_or(SessionStatus::Pending)
    }

    /// Calculates overall progress for all sessions.
    ///
    /// # Returns
    ///
    /// A `MonitorProgress` struct containing progress statistics.
    pub async fn calculate_progress(&self) -> MonitorProgress {
        let statuses = self.session_statuses.read().await;

        let mut progress = MonitorProgress::new();
        progress.total_sessions = statuses.len();

        for status in statuses.values() {
            match status {
                SessionStatus::Pending => progress.pending_sessions += 1,
                SessionStatus::Running => progress.running_sessions += 1,
                SessionStatus::Completed => progress.completed_sessions += 1,
                SessionStatus::TimedOut => progress.timed_out_sessions += 1,
                SessionStatus::Failed => progress.failed_sessions += 1,
            }
        }

        progress
    }

    /// Handles a monitor event by logging it.
    ///
    /// # Arguments
    ///
    /// * `event` - The monitor event to handle
    fn handle_monitor_event(&self, event: &MonitorEvent) {
        super::monitor::log::event(event);
    }

    /// Checks all sessions and returns a list of events for status changes.
    ///
    /// # Returns
    ///
    /// A vector of monitor events for sessions that changed status.
    async fn check_all_sessions(&self) -> Vec<MonitorEvent> {
        let mut events = Vec::new();

        let sessions = self.sessions.read().await;
        let session_ids: Vec<SessionId> = sessions.keys().cloned().collect();
        drop(sessions);

        for session_id in session_ids {
            let old_status = self.get_session_status(&session_id).await;
            let new_status = self.determine_session_status(&session_id).await;

            // If status changed to a terminal state, generate an event
            if old_status != Some(new_status) && new_status.is_terminal() {
                match new_status {
                    SessionStatus::Completed => {
                        let start_times = self.session_start_times.read().await;
                        let duration_secs = start_times
                            .get(&session_id)
                            .map(|start| start.elapsed().as_secs())
                            .unwrap_or(0);
                        events.push(MonitorEvent::SessionCompleted {
                            session_id,
                            duration_secs,
                        });
                    }
                    SessionStatus::TimedOut => {
                        events.push(MonitorEvent::SessionTimedOut {
                            session_id,
                            timeout_secs: self.config.session_timeout_secs,
                        });
                    }
                    SessionStatus::Failed => {
                        events.push(MonitorEvent::SessionFailed {
                            session_id,
                            reason: "Unknown error".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }

        events
    }

    /// Main monitoring loop that tracks session progress.
    ///
    /// This loop runs continuously, checking session status every
    /// `monitor_interval_secs` seconds. It logs events and progress updates.
    ///
    /// The loop exits when all sessions reach a terminal state.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use application::orchestration::{Orchestrator, OrchestratorConfig};
    /// use domain::value_objects::{SpecId, Phase};
    ///
    /// # async {
    /// let config = OrchestratorConfig::default();
    /// let orchestrator = Orchestrator::new(config);
    ///
    /// // Register and start sessions
    /// let spec = SpecId::new();
    /// orchestrator.register_spec(&spec, Phase::Tdd).await.unwrap();
    /// orchestrator.start_all_sessions().await.unwrap();
    ///
    /// // Start monitoring loop (blocks until all sessions complete)
    /// orchestrator.monitor_loop().await;
    /// # };
    /// ```
    pub async fn monitor_loop(&self) {
        use super::monitor::log;

        log::info("Starting monitor loop");

        let interval_duration = Duration::from_secs(self.config.monitor_interval_secs);
        let mut interval = tokio::time::interval(interval_duration);

        loop {
            interval.tick().await;

            // Check all sessions for status changes
            let events = self.check_all_sessions().await;

            // Handle each event
            for event in events {
                self.handle_monitor_event(&event);
            }

            // Calculate and log progress
            let progress = self.calculate_progress().await;

            if progress.total_sessions > 0 {
                let progress_event = MonitorEvent::ProgressUpdate {
                    completed: progress.completed_sessions
                        + progress.failed_sessions
                        + progress.timed_out_sessions,
                    total: progress.total_sessions,
                    percent: progress.progress_percent(),
                };
                self.handle_monitor_event(&progress_event);
            }

            // Exit loop if all sessions are in terminal state
            if progress.all_terminal() {
                log::info("All sessions completed, exiting monitor loop");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_objects::{Phase, SpecId};

    #[tokio::test]
    async fn test_orchestrator_creation() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        assert_eq!(orchestrator.session_count().await, 0);
    }

    #[tokio::test]
    async fn test_add_session() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session = Session::new(SpecId::new(), Phase::Tdd);
        let expected_id = session.id.clone();

        let session_id = orchestrator.add_session(session).await.unwrap();
        assert_eq!(session_id, expected_id);
        assert_eq!(orchestrator.session_count().await, 1);
    }

    #[tokio::test]
    async fn test_add_duplicate_session() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session1 = Session::new(SpecId::new(), Phase::Tdd);
        let session2 = session1.clone();

        orchestrator.add_session(session1).await.unwrap();
        let result = orchestrator.add_session(session2).await;

        assert!(result.is_err());
        assert_eq!(orchestrator.session_count().await, 1);
    }

    #[tokio::test]
    async fn test_get_session() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session = Session::new(SpecId::new(), Phase::Tdd);
        let session_id = session.id.clone();

        orchestrator.add_session(session.clone()).await.unwrap();

        let retrieved = orchestrator.get_session(&session_id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, session_id);
    }

    #[tokio::test]
    async fn test_get_nonexistent_session() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let result = orchestrator.get_session(&"nonexistent".to_string()).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_remove_session() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session = Session::new(SpecId::new(), Phase::Tdd);
        let session_id = session.id.clone();

        orchestrator.add_session(session).await.unwrap();
        assert_eq!(orchestrator.session_count().await, 1);

        let removed = orchestrator.remove_session(&session_id).await;
        assert!(removed.is_some());
        assert_eq!(orchestrator.session_count().await, 0);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_session() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let removed = orchestrator
            .remove_session(&"nonexistent".to_string())
            .await;
        assert!(removed.is_none());
    }

    #[tokio::test]
    async fn test_get_all_sessions() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session1 = Session::new(SpecId::new(), Phase::Tdd);
        let session2 = Session::new(SpecId::new(), Phase::Review);

        orchestrator.add_session(session1).await.unwrap();
        orchestrator.add_session(session2).await.unwrap();

        let all_sessions = orchestrator.get_all_sessions().await;
        assert_eq!(all_sessions.len(), 2);
    }

    #[tokio::test]
    async fn test_config_access() {
        let config = OrchestratorConfig {
            max_parallel_sessions: 8,
            session_timeout_secs: 7200,
            monitor_interval_secs: 2,
        };

        let orchestrator = Orchestrator::new(config.clone());

        assert_eq!(
            orchestrator.config().max_parallel_sessions,
            config.max_parallel_sessions
        );
        assert_eq!(
            orchestrator.config().session_timeout_secs,
            config.session_timeout_secs
        );
        assert_eq!(
            orchestrator.config().monitor_interval_secs,
            config.monitor_interval_secs
        );
    }

    #[tokio::test]
    async fn test_tokio_integration() {
        // This test verifies that the orchestrator can be used within a tokio context
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        // Add a session asynchronously
        let session = Session::new(SpecId::new(), Phase::Tdd);
        let session_id = orchestrator.add_session(session).await.unwrap();

        // Retrieve it asynchronously
        let retrieved = orchestrator.get_session(&session_id).await;
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_register_spec() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let spec_id = SpecId::new();
        let session_id = orchestrator
            .register_spec(&spec_id, Phase::Tdd)
            .await
            .unwrap();

        assert!(!session_id.is_empty());
        assert_eq!(orchestrator.session_count().await, 1);
    }

    #[tokio::test]
    async fn test_register_spec_with_dependencies() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let spec1 = SpecId::new();
        let spec2 = SpecId::new();

        orchestrator
            .register_spec(&spec1, Phase::Tdd)
            .await
            .unwrap();
        orchestrator
            .register_spec_with_dependencies(&spec2, Phase::Tdd, &[spec1.clone()])
            .await
            .unwrap();

        assert_eq!(orchestrator.session_count().await, 2);
    }

    #[tokio::test]
    async fn test_register_spec_cyclic_dependency() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let spec1 = SpecId::new();
        let spec2 = SpecId::new();

        orchestrator
            .register_spec(&spec1, Phase::Tdd)
            .await
            .unwrap();
        orchestrator
            .register_spec_with_dependencies(&spec2, Phase::Tdd, &[spec1.clone()])
            .await
            .unwrap();

        // Try to create a cycle: spec1 depends on spec2
        let result = orchestrator
            .register_spec_with_dependencies(&spec1, Phase::Tdd, &[spec2.clone()])
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_start_session() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let spec_id = SpecId::new();
        let session_id = orchestrator
            .register_spec(&spec_id, Phase::Tdd)
            .await
            .unwrap();

        let result = orchestrator.start_session(&session_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_nonexistent_session() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let result = orchestrator
            .start_session(&"nonexistent".to_string())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_start_all_sessions_simple() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let spec1 = SpecId::new();
        let spec2 = SpecId::new();

        orchestrator
            .register_spec(&spec1, Phase::Tdd)
            .await
            .unwrap();
        orchestrator
            .register_spec(&spec2, Phase::Tdd)
            .await
            .unwrap();

        let started = orchestrator.start_all_sessions().await.unwrap();
        assert_eq!(started.len(), 2);
    }

    #[tokio::test]
    async fn test_start_all_sessions_with_dependencies() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let spec1 = SpecId::new();
        let spec2 = SpecId::new();
        let spec3 = SpecId::new();

        orchestrator
            .register_spec(&spec1, Phase::Tdd)
            .await
            .unwrap();
        orchestrator
            .register_spec_with_dependencies(&spec2, Phase::Tdd, &[spec1.clone()])
            .await
            .unwrap();
        orchestrator
            .register_spec_with_dependencies(&spec3, Phase::Tdd, &[spec1.clone()])
            .await
            .unwrap();

        let started = orchestrator.start_all_sessions().await.unwrap();
        assert_eq!(started.len(), 3);

        // spec1 should be started first
        let sessions = orchestrator.get_all_sessions().await;
        let spec1_session = sessions
            .iter()
            .find(|s| s.spec_id == spec1)
            .unwrap();
        assert_eq!(started[0], spec1_session.id);
    }

    #[tokio::test]
    async fn test_get_parallel_execution_groups() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let spec1 = SpecId::new();
        let spec2 = SpecId::new();
        let spec3 = SpecId::new();

        orchestrator
            .register_spec(&spec1, Phase::Tdd)
            .await
            .unwrap();
        orchestrator
            .register_spec_with_dependencies(&spec2, Phase::Tdd, &[spec1.clone()])
            .await
            .unwrap();
        orchestrator
            .register_spec_with_dependencies(&spec3, Phase::Tdd, &[spec1.clone()])
            .await
            .unwrap();

        let groups = orchestrator.get_parallel_execution_groups().await.unwrap();

        // Filter out empty string nodes (used for specs with no dependencies)
        let filtered_groups: Vec<Vec<String>> = groups
            .iter()
            .map(|wave| {
                wave.iter()
                    .filter(|spec_id| !spec_id.is_empty())
                    .cloned()
                    .collect()
            })
            .filter(|wave: &Vec<String>| !wave.is_empty())
            .collect();

        // Wave 0: spec1
        assert_eq!(filtered_groups[0].len(), 1);
        assert_eq!(filtered_groups[0][0], spec1.to_string());

        // Wave 1: spec2 and spec3 (can run in parallel)
        assert_eq!(filtered_groups[1].len(), 2);
        assert!(filtered_groups[1].contains(&spec2.to_string()));
        assert!(filtered_groups[1].contains(&spec3.to_string()));
    }

    #[tokio::test]
    async fn test_session_status_pending_on_add() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session = Session::new(SpecId::new(), Phase::Tdd);
        let session_id = orchestrator.add_session(session).await.unwrap();

        let status = orchestrator.get_session_status(&session_id).await;
        assert_eq!(status, Some(SessionStatus::Pending));
    }

    #[tokio::test]
    async fn test_session_status_running_on_start() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let spec_id = SpecId::new();
        let session_id = orchestrator
            .register_spec(&spec_id, Phase::Tdd)
            .await
            .unwrap();

        orchestrator.start_session(&session_id).await.unwrap();

        let status = orchestrator.get_session_status(&session_id).await;
        assert_eq!(status, Some(SessionStatus::Running));
    }

    #[tokio::test]
    async fn test_mark_session_completed() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session = Session::new(SpecId::new(), Phase::Tdd);
        let session_id = orchestrator.add_session(session).await.unwrap();

        orchestrator.mark_session_completed(&session_id).await;

        let status = orchestrator.get_session_status(&session_id).await;
        assert_eq!(status, Some(SessionStatus::Completed));
    }

    #[tokio::test]
    async fn test_mark_session_failed() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session = Session::new(SpecId::new(), Phase::Tdd);
        let session_id = orchestrator.add_session(session).await.unwrap();

        orchestrator.mark_session_failed(&session_id).await;

        let status = orchestrator.get_session_status(&session_id).await;
        assert_eq!(status, Some(SessionStatus::Failed));
    }

    #[tokio::test]
    async fn test_calculate_progress() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        // Add sessions
        let session1 = Session::new(SpecId::new(), Phase::Tdd);
        let session2 = Session::new(SpecId::new(), Phase::Tdd);
        let session3 = Session::new(SpecId::new(), Phase::Tdd);

        let id1 = orchestrator.add_session(session1).await.unwrap();
        let id2 = orchestrator.add_session(session2).await.unwrap();
        let _id3 = orchestrator.add_session(session3).await.unwrap();

        // Mark different statuses
        orchestrator.mark_session_completed(&id1).await;
        orchestrator.mark_session_failed(&id2).await;
        // id3 remains Pending

        let progress = orchestrator.calculate_progress().await;
        assert_eq!(progress.total_sessions, 3);
        assert_eq!(progress.completed_sessions, 1);
        assert_eq!(progress.failed_sessions, 1);
        assert_eq!(progress.pending_sessions, 1);
        assert_eq!(progress.progress_percent(), 66); // (1 + 1) / 3 * 100 = 66
    }

    #[tokio::test]
    async fn test_session_timeout_detection() {
        let config = OrchestratorConfig {
            max_parallel_sessions: 4,
            session_timeout_secs: 1, // 1 second timeout for testing
            monitor_interval_secs: 1,
        };
        let orchestrator = Orchestrator::new(config);

        let spec_id = SpecId::new();
        let session_id = orchestrator
            .register_spec(&spec_id, Phase::Tdd)
            .await
            .unwrap();

        orchestrator.start_session(&session_id).await.unwrap();

        // Wait for timeout
        tokio::time::sleep(Duration::from_secs(2)).await;

        let status = orchestrator.determine_session_status(&session_id).await;
        assert_eq!(status, SessionStatus::TimedOut);
    }

    #[tokio::test]
    async fn test_monitor_loop_exits_when_all_terminal() {
        let config = OrchestratorConfig {
            max_parallel_sessions: 4,
            session_timeout_secs: 3600,
            monitor_interval_secs: 1,
        };
        let orchestrator = Orchestrator::new(config);

        // Add a session
        let session = Session::new(SpecId::new(), Phase::Tdd);
        let session_id = orchestrator.add_session(session).await.unwrap();

        // Mark as completed immediately
        orchestrator.mark_session_completed(&session_id).await;

        // Monitor loop should exit immediately
        let start = Instant::now();
        orchestrator.monitor_loop().await;
        let elapsed = start.elapsed();

        // Should exit quickly (within 2 seconds)
        assert!(elapsed < Duration::from_secs(2));
    }

    #[tokio::test]
    async fn test_remove_session_clears_status() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config);

        let session = Session::new(SpecId::new(), Phase::Tdd);
        let session_id = orchestrator.add_session(session).await.unwrap();

        // Verify status exists
        assert_eq!(
            orchestrator.get_session_status(&session_id).await,
            Some(SessionStatus::Pending)
        );

        // Remove session
        orchestrator.remove_session(&session_id).await;

        // Status should be gone
        assert_eq!(orchestrator.get_session_status(&session_id).await, None);
    }
}
