//! Core orchestrator for managing multiple sessions.

use super::OrchestratorConfig;
use crate::error::{ApplicationError, Result};
use domain::entities::Session;
use std::collections::HashMap;
use std::sync::Arc;
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
        sessions.remove(session_id)
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
}
