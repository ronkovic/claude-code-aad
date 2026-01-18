//! GitHub integration service.

use crate::{ApplicationError, Result};
use infrastructure::adapters::GhCliAdapter;

/// Service for managing GitHub integrations.
///
/// This service provides high-level operations for creating PRs,
/// merging PRs, and creating issues using the GitHub CLI adapter.
pub struct IntegrationService {
    gh_adapter: GhCliAdapter,
}

impl IntegrationService {
    /// Creates a new IntegrationService.
    pub fn new() -> Self {
        Self {
            gh_adapter: GhCliAdapter::new(),
        }
    }

    /// Creates a new IntegrationService with a custom GhCliAdapter.
    ///
    /// # Arguments
    ///
    /// * `gh_adapter` - GitHub CLI adapter to use
    pub fn with_adapter(gh_adapter: GhCliAdapter) -> Self {
        Self { gh_adapter }
    }

    /// Creates a pull request.
    ///
    /// # Arguments
    ///
    /// * `title` - PR title
    /// * `body` - PR description
    /// * `base` - Base branch (e.g., "main")
    ///
    /// # Returns
    ///
    /// The PR number.
    ///
    /// # Errors
    ///
    /// Returns an error if the PR creation fails.
    pub fn create_pull_request(&self, title: &str, body: &str, base: &str) -> Result<u64> {
        self.gh_adapter
            .create_pr(title, body, base)
            .map_err(|e| ApplicationError::ExternalServiceError(format!("PR creation failed: {}", e)))
    }

    /// Merges a pull request.
    ///
    /// # Arguments
    ///
    /// * `number` - PR number to merge
    /// * `strategy` - Merge strategy ("merge", "squash", or "rebase")
    ///
    /// # Errors
    ///
    /// Returns an error if the PR merge fails.
    pub fn merge_pull_request(&self, number: u64, strategy: &str) -> Result<()> {
        self.gh_adapter
            .merge_pr(number, strategy)
            .map_err(|e| ApplicationError::ExternalServiceError(format!("PR merge failed: {}", e)))
    }

    /// Creates an issue.
    ///
    /// # Arguments
    ///
    /// * `title` - Issue title
    /// * `body` - Issue description
    /// * `labels` - Labels to apply
    ///
    /// # Returns
    ///
    /// The issue number.
    ///
    /// # Errors
    ///
    /// Returns an error if the issue creation fails.
    pub fn create_issue(&self, title: &str, body: &str, labels: &[&str]) -> Result<u64> {
        self.gh_adapter
            .create_issue(title, body, labels)
            .map_err(|e| ApplicationError::ExternalServiceError(format!("Issue creation failed: {}", e)))
    }
}

impl Default for IntegrationService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_service() {
        let _service = IntegrationService::new();
        // Service is created successfully
        assert!(true);
    }

    #[test]
    fn test_default_trait() {
        let _service = IntegrationService::default();
        // Service is created successfully
        assert!(true);
    }

    #[test]
    fn test_with_adapter_creates_service_with_custom_adapter() {
        let adapter = GhCliAdapter::with_path("/custom/gh".to_string());
        let _service = IntegrationService::with_adapter(adapter);
        // Service is created successfully with custom adapter
        assert!(true);
    }
}
