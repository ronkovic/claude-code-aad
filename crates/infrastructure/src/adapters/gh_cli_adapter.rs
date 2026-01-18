//! GitHub CLI adapter for managing PRs and Issues.

use crate::persistence::{PersistenceError, Result};
use std::process::Command;

/// Adapter for interacting with GitHub CLI (gh).
///
/// This adapter wraps the `gh` command-line tool to provide
/// programmatic access to GitHub operations such as creating PRs,
/// merging PRs, and creating issues.
pub struct GhCliAdapter {
    /// Path to the gh command (defaults to "gh")
    pub(crate) gh_path: String,
}

impl GhCliAdapter {
    /// Creates a new GhCliAdapter with default settings.
    pub fn new() -> Self {
        Self {
            gh_path: "gh".to_string(),
        }
    }

    /// Creates a new GhCliAdapter with a custom gh command path.
    ///
    /// # Arguments
    ///
    /// * `gh_path` - Path to the gh command executable
    pub fn with_path(gh_path: String) -> Self {
        Self { gh_path }
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
    /// The PR number extracted from gh output.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The gh command fails to execute
    /// - The output cannot be parsed
    pub fn create_pr(&self, title: &str, body: &str, base: &str) -> Result<u64> {
        let output = Command::new(&self.gh_path)
            .args(["pr", "create", "--title", title, "--body", body, "--base", base])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PersistenceError::CommandError(format!(
                "gh pr create failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        self.parse_pr_number(&stdout)
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
    /// Returns an error if the gh command fails to execute.
    pub fn merge_pr(&self, number: u64, strategy: &str) -> Result<()> {
        let merge_flag = match strategy {
            "merge" => "--merge",
            "squash" => "--squash",
            "rebase" => "--rebase",
            _ => {
                return Err(PersistenceError::InvalidInput(format!(
                    "Invalid merge strategy: {}",
                    strategy
                )))
            }
        };

        let output = Command::new(&self.gh_path)
            .args(["pr", "merge", &number.to_string(), merge_flag])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PersistenceError::CommandError(format!(
                "gh pr merge failed: {}",
                stderr
            )));
        }

        Ok(())
    }

    /// Creates an issue.
    ///
    /// # Arguments
    ///
    /// * `title` - Issue title
    /// * `body` - Issue description
    /// * `labels` - Labels to apply (comma-separated)
    ///
    /// # Returns
    ///
    /// The issue number extracted from gh output.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The gh command fails to execute
    /// - The output cannot be parsed
    pub fn create_issue(&self, title: &str, body: &str, labels: &[&str]) -> Result<u64> {
        let labels_str = labels.join(",");
        let mut args = vec!["issue", "create", "--title", title, "--body", body];

        if !labels.is_empty() {
            args.push("--label");
            args.push(&labels_str);
        }

        let output = Command::new(&self.gh_path).args(&args).output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PersistenceError::CommandError(format!(
                "gh issue create failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        self.parse_issue_number(&stdout)
    }

    /// Parses PR number from gh pr create output.
    ///
    /// Example output: "https://github.com/owner/repo/pull/123"
    fn parse_pr_number(&self, output: &str) -> Result<u64> {
        output
            .trim()
            .split('/')
            .next_back()
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| {
                PersistenceError::ParseError(format!("Failed to parse PR number from: {}", output))
            })
    }

    /// Parses issue number from gh issue create output.
    ///
    /// Example output: "https://github.com/owner/repo/issues/456"
    fn parse_issue_number(&self, output: &str) -> Result<u64> {
        output
            .trim()
            .split('/')
            .next_back()
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| {
                PersistenceError::ParseError(format!(
                    "Failed to parse issue number from: {}",
                    output
                ))
            })
    }
}

impl Default for GhCliAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_adapter_with_default_path() {
        let adapter = GhCliAdapter::new();
        assert_eq!(adapter.gh_path, "gh");
    }

    #[test]
    fn test_with_path_creates_adapter_with_custom_path() {
        let adapter = GhCliAdapter::with_path("/custom/path/gh".to_string());
        assert_eq!(adapter.gh_path, "/custom/path/gh");
    }

    #[test]
    fn test_parse_pr_number_extracts_number() {
        let adapter = GhCliAdapter::new();
        let output = "https://github.com/owner/repo/pull/123\n";
        let pr_number = adapter.parse_pr_number(output).unwrap();
        assert_eq!(pr_number, 123);
    }

    #[test]
    fn test_parse_pr_number_fails_on_invalid_output() {
        let adapter = GhCliAdapter::new();
        let output = "invalid output";
        let result = adapter.parse_pr_number(output);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_issue_number_extracts_number() {
        let adapter = GhCliAdapter::new();
        let output = "https://github.com/owner/repo/issues/456\n";
        let issue_number = adapter.parse_issue_number(output).unwrap();
        assert_eq!(issue_number, 456);
    }

    #[test]
    fn test_parse_issue_number_fails_on_invalid_output() {
        let adapter = GhCliAdapter::new();
        let output = "invalid output";
        let result = adapter.parse_issue_number(output);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_pr_rejects_invalid_strategy() {
        let adapter = GhCliAdapter::with_path("echo".to_string());
        let result = adapter.merge_pr(123, "invalid");
        assert!(result.is_err());
        if let Err(PersistenceError::InvalidInput(msg)) = result {
            assert!(msg.contains("Invalid merge strategy"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_default_trait() {
        let adapter = GhCliAdapter::default();
        assert_eq!(adapter.gh_path, "gh");
    }
}
