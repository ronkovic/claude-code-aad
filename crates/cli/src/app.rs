//! Application dependency injection container.

use domain::repositories::{SessionRepository, SpecRepository, TaskRepository};
use infrastructure::config::AadConfig;
use std::sync::Arc;

/// Application dependency injection container.
///
/// This struct holds all the dependencies needed by the application,
/// including repositories and configuration.
#[allow(dead_code)]
pub struct App {
    spec_repository: Arc<dyn SpecRepository>,
    task_repository: Arc<dyn TaskRepository>,
    session_repository: Arc<dyn SessionRepository>,
    config: AadConfig,
}

#[allow(dead_code)]
impl App {
    /// Creates a new App instance with default dependencies.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization of dependencies fails.
    pub fn new() -> anyhow::Result<Self> {
        // TODO: 実際のリポジトリ実装をインスタンス化（SPEC-004で実装予定）
        let _config = AadConfig::default();

        // Placeholder: リポジトリの具象実装は SPEC-004 で実装
        todo!("FileSpecRepository の実装待ち")
    }

    /// Creates a new App instance with custom repositories for testing.
    ///
    /// This constructor is useful for dependency injection in tests.
    #[cfg(test)]
    pub fn with_repositories(
        spec_repository: Arc<dyn SpecRepository>,
        task_repository: Arc<dyn TaskRepository>,
        session_repository: Arc<dyn SessionRepository>,
        config: AadConfig,
    ) -> Self {
        Self {
            spec_repository,
            task_repository,
            session_repository,
            config,
        }
    }

    /// Returns a reference to the spec repository.
    pub fn spec_repository(&self) -> &dyn SpecRepository {
        &*self.spec_repository
    }

    /// Returns a reference to the task repository.
    pub fn task_repository(&self) -> &dyn TaskRepository {
        &*self.task_repository
    }

    /// Returns a reference to the session repository.
    pub fn session_repository(&self) -> &dyn SessionRepository {
        &*self.session_repository
    }

    /// Returns a reference to the configuration.
    pub fn config(&self) -> &AadConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use domain::entities::{Session, Spec, Task};
    use domain::value_objects::{SpecId, TaskId};

    // Mock implementations for testing

    struct MockSpecRepository;

    #[async_trait]
    impl SpecRepository for MockSpecRepository {
        async fn find_by_id(&self, _id: &SpecId) -> domain::Result<Option<Spec>> {
            Ok(None)
        }

        async fn find_all(&self) -> domain::Result<Vec<Spec>> {
            Ok(vec![])
        }

        async fn save(&self, _spec: &Spec) -> domain::Result<()> {
            Ok(())
        }

        async fn delete(&self, _id: &SpecId) -> domain::Result<()> {
            Ok(())
        }
    }

    struct MockTaskRepository;

    #[async_trait]
    impl TaskRepository for MockTaskRepository {
        async fn find_by_id(&self, _id: &TaskId) -> domain::Result<Option<Task>> {
            Ok(None)
        }

        async fn find_by_spec_id(&self, _spec_id: &SpecId) -> domain::Result<Vec<Task>> {
            Ok(vec![])
        }

        async fn save(&self, _task: &Task) -> domain::Result<()> {
            Ok(())
        }

        async fn delete(&self, _id: &TaskId) -> domain::Result<()> {
            Ok(())
        }
    }

    struct MockSessionRepository;

    #[async_trait]
    impl SessionRepository for MockSessionRepository {
        async fn find_by_id(&self, _id: &str) -> domain::Result<Option<Session>> {
            Ok(None)
        }

        async fn find_active(&self) -> domain::Result<Vec<Session>> {
            Ok(vec![])
        }

        async fn save(&self, _session: &Session) -> domain::Result<()> {
            Ok(())
        }

        async fn delete(&self, _id: &str) -> domain::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_app_with_mock_repositories() {
        let spec_repo = Arc::new(MockSpecRepository) as Arc<dyn SpecRepository>;
        let task_repo = Arc::new(MockTaskRepository) as Arc<dyn TaskRepository>;
        let session_repo = Arc::new(MockSessionRepository) as Arc<dyn SessionRepository>;
        let config = AadConfig::default();

        let app = App::with_repositories(spec_repo, task_repo, session_repo, config);

        // Verify that repositories are accessible
        assert!(app.spec_repository() as *const _ as *const () != std::ptr::null());
        assert!(app.task_repository() as *const _ as *const () != std::ptr::null());
        assert!(app.session_repository() as *const _ as *const () != std::ptr::null());
        assert_eq!(app.config().version, "0.1.0");
    }

    #[test]
    fn test_app_config_access() {
        let spec_repo = Arc::new(MockSpecRepository) as Arc<dyn SpecRepository>;
        let task_repo = Arc::new(MockTaskRepository) as Arc<dyn TaskRepository>;
        let session_repo = Arc::new(MockSessionRepository) as Arc<dyn SessionRepository>;
        let config = AadConfig::default();

        let app = App::with_repositories(spec_repo, task_repo, session_repo, config);

        assert_eq!(app.config().context_threshold, 70);
        assert_eq!(app.config().default_branch, Some("main".to_string()));
    }
}
