//! Session repository trait.

use crate::entities::Session;
use crate::Result;

/// Repository interface for Session entities.
///
/// This trait defines the contract for persisting and retrieving sessions.
/// Concrete implementations belong in the infrastructure layer.
#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    /// Finds a session by its ID.
    ///
    /// Returns `None` if the session does not exist.
    async fn find_by_id(&self, id: &str) -> Result<Option<Session>>;

    /// Finds all active sessions (not ended).
    async fn find_active(&self) -> Result<Vec<Session>>;

    /// Saves a session.
    ///
    /// If the session already exists, it will be updated.
    async fn save(&self, session: &Session) -> Result<()>;

    /// Deletes a session by its ID.
    ///
    /// Returns `Ok(())` even if the session does not exist.
    async fn delete(&self, id: &str) -> Result<()>;
}
