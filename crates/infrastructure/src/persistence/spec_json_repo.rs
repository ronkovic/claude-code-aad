//! JSON-based implementation of SpecRepository.

use async_trait::async_trait;
use domain::{
    entities::Spec, repositories::SpecRepository, value_objects::SpecId, Result as DomainResult,
};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::persistence::{PersistenceError, Result};

/// JSON file-based implementation of SpecRepository.
///
/// Stores specifications as individual JSON files in `.aad/data/specs/` directory.
pub struct SpecJsonRepo {
    base_dir: PathBuf,
}

impl SpecJsonRepo {
    /// Creates a new SpecJsonRepo.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - Base directory for storing spec files (e.g., `.aad/data/specs`)
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Gets the file path for a spec ID.
    fn get_file_path(&self, id: &SpecId) -> PathBuf {
        self.base_dir.join(format!("{}.json", id.as_str()))
    }

    /// Ensures the base directory exists.
    async fn ensure_dir_exists(&self) -> Result<()> {
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir).await?;
        }
        Ok(())
    }

    /// Validates the spec ID to prevent path traversal attacks.
    fn validate_id(&self, id: &SpecId) -> Result<()> {
        let id_str = id.as_str();
        if id_str.contains("..") || id_str.contains('/') || id_str.contains('\\') {
            return Err(PersistenceError::PathTraversalError(format!(
                "Invalid SpecId: {}",
                id_str
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl SpecRepository for SpecJsonRepo {
    async fn find_by_id(&self, id: &SpecId) -> DomainResult<Option<Spec>> {
        // Use PersistenceError internally, convert to DomainError at the end
        let result: Result<Option<Spec>> = async {
            self.validate_id(id)?;

            let file_path = self.get_file_path(id);

            if !file_path.exists() {
                return Ok(None);
            }

            let content = fs::read_to_string(&file_path).await?;
            let spec: Spec = serde_json::from_str(&content)?;

            Ok(Some(spec))
        }
        .await;

        result.map_err(Into::into)
    }

    async fn find_all(&self) -> DomainResult<Vec<Spec>> {
        if !self.base_dir.exists() {
            return Ok(Vec::new());
        }

        let mut specs = Vec::new();
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

                let spec: Spec = serde_json::from_str(&content)
                    .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

                specs.push(spec);
            }
        }

        Ok(specs)
    }

    async fn save(&self, spec: &Spec) -> DomainResult<()> {
        self.validate_id(&spec.id)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        self.ensure_dir_exists()
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        let file_path = self.get_file_path(&spec.id);
        let content = serde_json::to_string_pretty(spec)
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        fs::write(&file_path, content)
            .await
            .map_err(|e| domain::DomainError::RepositoryError(format!("{:?}", e)))?;

        Ok(())
    }

    async fn delete(&self, id: &SpecId) -> DomainResult<()> {
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
    use domain::entities::Spec;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_spec_json_repo_save_and_find() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SpecJsonRepo::new(temp_dir.path());

        let spec = Spec::new("Test Spec".to_string(), "Test Description".to_string()).unwrap();
        let spec_id = spec.id.clone();

        // Save spec
        repo.save(&spec).await.unwrap();

        // Find spec
        let found = repo.find_by_id(&spec_id).await.unwrap();
        assert!(found.is_some());
        let found_spec = found.unwrap();
        assert_eq!(found_spec.name, "Test Spec");
        assert_eq!(found_spec.description, "Test Description");
    }

    #[tokio::test]
    async fn test_spec_json_repo_find_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SpecJsonRepo::new(temp_dir.path());

        let nonexistent_id = SpecId::new();
        let result = repo.find_by_id(&nonexistent_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_spec_json_repo_find_all() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SpecJsonRepo::new(temp_dir.path());

        // Save multiple specs
        let spec1 = Spec::new("Spec 1".to_string(), "Desc 1".to_string()).unwrap();
        let spec2 = Spec::new("Spec 2".to_string(), "Desc 2".to_string()).unwrap();

        repo.save(&spec1).await.unwrap();
        repo.save(&spec2).await.unwrap();

        // Find all
        let all_specs = repo.find_all().await.unwrap();
        assert_eq!(all_specs.len(), 2);
    }

    #[tokio::test]
    async fn test_spec_json_repo_delete() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SpecJsonRepo::new(temp_dir.path());

        let spec = Spec::new("Test Spec".to_string(), "Test Description".to_string()).unwrap();
        let spec_id = spec.id.clone();

        // Save and verify
        repo.save(&spec).await.unwrap();
        assert!(repo.find_by_id(&spec_id).await.unwrap().is_some());

        // Delete and verify
        repo.delete(&spec_id).await.unwrap();
        assert!(repo.find_by_id(&spec_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_spec_json_repo_delete_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SpecJsonRepo::new(temp_dir.path());

        let nonexistent_id = SpecId::new();
        // Should not error even if file doesn't exist
        repo.delete(&nonexistent_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_spec_json_repo_path_traversal_protection() {
        let temp_dir = TempDir::new().unwrap();
        let repo = SpecJsonRepo::new(temp_dir.path());

        let malicious_id = std::str::FromStr::from_str("../../../etc/passwd").unwrap();
        let result = repo.find_by_id(&malicious_id).await;
        assert!(result.is_err());
    }
}
