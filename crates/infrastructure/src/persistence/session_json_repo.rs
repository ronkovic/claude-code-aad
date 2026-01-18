//! JSON-based implementation of SessionRepository.

use async_trait::async_trait;
use domain::{entities::Session, repositories::SessionRepository, Result as DomainResult};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::persistence::{PersistenceError, Result};

/// JSON file-based implementation of SessionRepository.
///
/// Stores sessions as individual JSON files in `.aad/data/sessions/` directory.
pub struct SessionJsonRepo {
    base_dir: PathBuf,
}

impl SessionJsonRepo {
    /// Creates a new SessionJsonRepo.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - Base directory for storing session files (e.g., `.aad/data/sessions`)
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Gets the file path for a session ID.
    fn get_file_path(&self, id: &str) -> PathBuf {
        self.base_dir.join(format!("{}.json", id))
    }

    /// Ensures the base directory exists.
    async fn ensure_dir_exists(&self) -> Result<()> {
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir).await?;
        }
        Ok(())
    }

    /// Validates the session ID to prevent path traversal attacks.
    fn validate_id(&self, id: &str) -> Result<()> {
        if id.contains("..") || id.contains('/') || id.contains('\\') {
            return Err(PersistenceError::PathTraversalError(format!(
                "Invalid SessionId: {}",
                id
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl SessionRepository for SessionJsonRepo {
    async fn find_by_id(&self, id: &str) -> DomainResult<Option<Session>> {
        self.validate_id(id)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        let file_path = self.get_file_path(id);

        if !file_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&file_path)
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        let session: Session = serde_json::from_str(&content)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        Ok(Some(session))
    }

    async fn find_active(&self) -> DomainResult<Vec<Session>> {
        if !self.base_dir.exists() {
            return Ok(Vec::new());
        }

        let mut sessions = Vec::new();
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

                let session: Session = serde_json::from_str(&content)
                    .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

                if session.is_active() {
                    sessions.push(session);
                }
            }
        }

        Ok(sessions)
    }

    async fn save(&self, session: &Session) -> DomainResult<()> {
        self.validate_id(&session.id)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        self.ensure_dir_exists()
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        let file_path = self.get_file_path(&session.id);
        let content = serde_json::to_string_pretty(session)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        fs::write(&file_path, content)
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> DomainResult<()> {
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
    use domain::{entities::Session, value_objects::{Phase, SpecId}};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_session_json_repo_save_and_find() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SessionJsonRepo::new(temp_dir.path());

        let spec_id = SpecId::new();
        let session = Session::new(spec_id, Phase::Tdd);
        let session_id = session.id.clone();

        // Save session
        repo.save(&session).await.unwrap();

        // Find session
        let found = repo.find_by_id(&session_id).await.unwrap();
        assert!(found.is_some());
        let found_session = found.unwrap();
        assert_eq!(found_session.id, session_id);
        assert_eq!(found_session.phase, Phase::Tdd);
    }

    #[tokio::test]
    async fn test_session_json_repo_find_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SessionJsonRepo::new(temp_dir.path());

        let result = repo.find_by_id("nonexistent-id").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_session_json_repo_find_active() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SessionJsonRepo::new(temp_dir.path());

        let spec_id1 = SpecId::new();
        let spec_id2 = SpecId::new();

        let mut session1 = Session::new(spec_id1, Phase::Tdd);
        let session2 = Session::new(spec_id2.clone(), Phase::Review);
        let mut session3 = Session::new(spec_id2, Phase::Merge);

        // session1: active
        // session2: active
        // session3: ended

        session3.end();

        repo.save(&session1).await.unwrap();
        repo.save(&session2).await.unwrap();
        repo.save(&session3).await.unwrap();

        // Find active sessions
        let active_sessions = repo.find_active().await.unwrap();
        assert_eq!(active_sessions.len(), 2);

        // Verify that session3 is not in active sessions
        assert!(!active_sessions.iter().any(|s| s.id == session3.id));
    }

    #[tokio::test]
    async fn test_session_json_repo_delete() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SessionJsonRepo::new(temp_dir.path());

        let spec_id = SpecId::new();
        let session = Session::new(spec_id, Phase::Tdd);
        let session_id = session.id.clone();

        // Save and verify
        repo.save(&session).await.unwrap();
        assert!(repo.find_by_id(&session_id).await.unwrap().is_some());

        // Delete and verify
        repo.delete(&session_id).await.unwrap();
        assert!(repo.find_by_id(&session_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_session_json_repo_delete_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SessionJsonRepo::new(temp_dir.path());

        // Should not error even if file doesn't exist
        repo.delete("nonexistent-id").await.unwrap();
    }

    #[tokio::test]
    async fn test_session_json_repo_path_traversal_protection() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SessionJsonRepo::new(temp_dir.path());

        let malicious_id = "../../../etc/passwd";
        let result = repo.find_by_id(malicious_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_session_json_repo_update() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SessionJsonRepo::new(temp_dir.path());

        let spec_id = SpecId::new();
        let mut session = Session::new(spec_id, Phase::Tdd);
        let session_id = session.id.clone();

        // Save initial session
        repo.save(&session).await.unwrap();

        // Update session
        session.update_context_usage(0.75).unwrap();
        repo.save(&session).await.unwrap();

        // Verify update
        let found = repo.find_by_id(&session_id).await.unwrap();
        assert!(found.is_some());
        let found_session = found.unwrap();
        assert_eq!(found_session.context_usage, 0.75);
    }

    #[tokio::test]
    async fn test_session_json_repo_find_active_empty() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SessionJsonRepo::new(temp_dir.path());

        // Before any sessions are saved
        let active_sessions = repo.find_active().await.unwrap();
        assert!(active_sessions.is_empty());
    }
}
