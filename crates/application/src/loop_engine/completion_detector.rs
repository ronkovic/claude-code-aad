//! Completion detection for autonomous task execution.
//!
//! This module provides pattern-based detection of task completion messages.
//! It uses regular expressions with safeguards against ReDoS attacks.

use crate::{ApplicationError, Result};
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

/// Maximum allowed time for pattern matching (10ms)
const MAX_PATTERN_MATCH_TIME: Duration = Duration::from_millis(10);

/// Configuration for completion patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionPatterns {
    /// List of regex patterns to match completion messages.
    pub patterns: Vec<String>,
}

/// Detects task completion based on configurable patterns.
///
/// # Examples
///
/// ```
/// use application::loop_engine::CompletionDetector;
///
/// let detector = CompletionDetector::from_patterns(vec![
///     "Task completed successfully".to_string(),
///     "All tasks completed".to_string(),
/// ]).unwrap();
///
/// assert!(detector.is_completed("Task completed successfully"));
/// assert!(detector.is_completed("All tasks completed"));
/// assert!(!detector.is_completed("Task in progress"));
/// ```
#[derive(Debug)]
pub struct CompletionDetector {
    patterns: RegexSet,
}

impl CompletionDetector {
    /// Creates a new completion detector from a configuration file.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the completion patterns JSON file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The configuration file cannot be read
    /// - The configuration file is invalid JSON
    /// - The patterns cannot be compiled
    /// - The patterns list is empty
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use application::loop_engine::CompletionDetector;
    /// use std::path::Path;
    ///
    /// let detector = CompletionDetector::from_config(
    ///     Path::new("config/completion-patterns.json")
    /// ).unwrap();
    /// ```
    pub fn from_config(config_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(config_path).map_err(|e| {
            ApplicationError::PatternLoadError(format!(
                "Failed to read config file {}: {}",
                config_path.display(),
                e
            ))
        })?;

        let config: CompletionPatterns = serde_json::from_str(&content).map_err(|e| {
            ApplicationError::PatternLoadError(format!("Invalid JSON in config file: {}", e))
        })?;

        if config.patterns.is_empty() {
            return Err(ApplicationError::PatternLoadError(
                "Patterns list cannot be empty".to_string(),
            ));
        }

        Self::from_patterns(config.patterns)
    }

    /// Creates a new completion detector from a list of pattern strings.
    ///
    /// # Arguments
    ///
    /// * `patterns` - Vector of regex pattern strings
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The patterns list is empty
    /// - Any pattern cannot be compiled as a valid regex
    ///
    /// # Examples
    ///
    /// ```
    /// use application::loop_engine::CompletionDetector;
    ///
    /// let detector = CompletionDetector::from_patterns(vec![
    ///     "Done\\.".to_string(),
    ///     "Finished\\.".to_string(),
    /// ]).unwrap();
    /// ```
    pub fn from_patterns(patterns: Vec<String>) -> Result<Self> {
        if patterns.is_empty() {
            return Err(ApplicationError::PatternLoadError(
                "Patterns list cannot be empty".to_string(),
            ));
        }

        let regex_set = RegexSet::new(&patterns).map_err(|e| {
            ApplicationError::PatternLoadError(format!("Failed to compile patterns: {}", e))
        })?;

        Ok(Self {
            patterns: regex_set,
        })
    }

    /// Checks if the given text indicates task completion.
    ///
    /// This method matches the text against all configured patterns using OR logic.
    /// It includes protection against ReDoS attacks by enforcing a time limit.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to check for completion indicators
    ///
    /// # Returns
    ///
    /// Returns `true` if any pattern matches, `false` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if pattern matching takes longer than 10ms (potential ReDoS attack).
    ///
    /// # Examples
    ///
    /// ```
    /// use application::loop_engine::CompletionDetector;
    ///
    /// let detector = CompletionDetector::from_patterns(vec![
    ///     "Success\\.".to_string(),
    /// ]).unwrap();
    ///
    /// assert!(detector.is_completed("Success."));
    /// assert!(!detector.is_completed("Failed"));
    /// ```
    pub fn is_completed(&self, text: &str) -> bool {
        let start = Instant::now();
        let result = self.patterns.is_match(text);
        let elapsed = start.elapsed();

        if elapsed > MAX_PATTERN_MATCH_TIME {
            panic!(
                "Pattern matching took too long ({:?}), possible ReDoS attack",
                elapsed
            );
        }

        result
    }

    /// Returns the number of configured patterns.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::loop_engine::CompletionDetector;
    ///
    /// let detector = CompletionDetector::from_patterns(vec![
    ///     "Done".to_string(),
    ///     "Finished".to_string(),
    /// ]).unwrap();
    ///
    /// assert_eq!(detector.pattern_count(), 2);
    /// ```
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_single_pattern_matching() {
        let detector =
            CompletionDetector::from_patterns(vec!["Task completed successfully".to_string()])
                .unwrap();

        assert!(detector.is_completed("Task completed successfully"));
        assert!(!detector.is_completed("Task in progress"));
        assert!(!detector.is_completed(""));
    }

    #[test]
    fn test_multiple_patterns_or_condition() {
        let detector = CompletionDetector::from_patterns(vec![
            "Done\\.".to_string(),
            "Finished\\.".to_string(),
            "Success\\.".to_string(),
        ])
        .unwrap();

        assert!(detector.is_completed("Done."));
        assert!(detector.is_completed("Finished."));
        assert!(detector.is_completed("Success."));
        assert!(!detector.is_completed("In progress"));
        assert!(!detector.is_completed("Done")); // Missing dot
    }

    #[test]
    fn test_regex_pattern_matching() {
        let detector = CompletionDetector::from_patterns(vec![
            "All tasks? (?:have been )?completed".to_string(),
        ])
        .unwrap();

        assert!(detector.is_completed("All tasks completed"));
        assert!(detector.is_completed("All task completed"));
        assert!(detector.is_completed("All tasks have been completed"));
        assert!(!detector.is_completed("Some tasks completed"));
    }

    #[test]
    fn test_empty_patterns_error() {
        let result = CompletionDetector::from_patterns(vec![]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::PatternLoadError(_)
        ));
    }

    #[test]
    fn test_invalid_regex_pattern() {
        let result = CompletionDetector::from_patterns(vec!["[invalid(regex".to_string()]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::PatternLoadError(_)
        ));
    }

    #[test]
    fn test_from_config_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"patterns": ["Done\\.", "Finished\\."]}}"#).unwrap();

        let detector = CompletionDetector::from_config(temp_file.path()).unwrap();
        assert_eq!(detector.pattern_count(), 2);
        assert!(detector.is_completed("Done."));
        assert!(detector.is_completed("Finished."));
    }

    #[test]
    fn test_from_config_invalid_json() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{{invalid json}}").unwrap();

        let result = CompletionDetector::from_config(temp_file.path());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::PatternLoadError(_)
        ));
    }

    #[test]
    fn test_from_config_empty_patterns() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"patterns": []}}"#).unwrap();

        let result = CompletionDetector::from_config(temp_file.path());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::PatternLoadError(_)
        ));
    }

    #[test]
    fn test_performance_under_10ms() {
        let detector = CompletionDetector::from_patterns(vec![
            "Task completed successfully".to_string(),
            "All tasks completed".to_string(),
            "Done\\.".to_string(),
        ])
        .unwrap();

        let start = Instant::now();
        for _ in 0..100 {
            detector.is_completed("Task completed successfully");
        }
        let elapsed = start.elapsed();

        // 100 iterations should complete well under 10ms
        assert!(elapsed < Duration::from_millis(10));
    }

    #[test]
    fn test_no_match_returns_false() {
        let detector = CompletionDetector::from_patterns(vec!["Success\\.".to_string()]).unwrap();

        assert!(!detector.is_completed("Failed"));
        assert!(!detector.is_completed("Error"));
        assert!(!detector.is_completed("In progress"));
    }

    #[test]
    fn test_pattern_count() {
        let detector = CompletionDetector::from_patterns(vec![
            "Pattern 1".to_string(),
            "Pattern 2".to_string(),
            "Pattern 3".to_string(),
        ])
        .unwrap();

        assert_eq!(detector.pattern_count(), 3);
    }
}
