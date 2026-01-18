//! Style-related value objects for output formatting.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Style name for output formatting.
///
/// Represents a named style configuration (e.g., "default", "minimal", "verbose").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StyleName(String);

impl StyleName {
    /// Maximum length for a style name.
    pub const MAX_LENGTH: usize = 64;

    /// Creates a new StyleName.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty or exceeds MAX_LENGTH.
    pub fn new(name: &str) -> Result<Self, crate::DomainError> {
        let trimmed = name.trim();

        if trimmed.is_empty() {
            return Err(crate::DomainError::ValidationError(
                "StyleName cannot be empty".to_string(),
            ));
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(crate::DomainError::ValidationError(format!(
                "StyleName cannot exceed {} characters",
                Self::MAX_LENGTH
            )));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StyleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Token replacement map for output formatting.
///
/// Maps template tokens (e.g., `{{feature}}`) to their replacement values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMap {
    tokens: HashMap<String, String>,
}

impl TokenMap {
    /// Maximum replacement depth to prevent infinite loops.
    const MAX_DEPTH: usize = 10;

    /// Creates a new empty TokenMap.
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    /// Creates a TokenMap with default tokens.
    pub fn default_tokens() -> Self {
        let mut map = Self::new();
        map.insert("date", &chrono::Utc::now().format("%Y-%m-%d").to_string());
        map.insert("author", "Claude Code");
        map
    }

    /// Inserts a token mapping.
    pub fn insert(&mut self, key: &str, value: &str) {
        self.tokens.insert(key.to_string(), value.to_string());
    }

    /// Gets a token value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.tokens.get(key).map(|s| s.as_str())
    }

    /// Replaces all tokens in the input string.
    ///
    /// Tokens are in the format `{{token_name}}`.
    ///
    /// # Errors
    ///
    /// Returns an error if circular references are detected.
    pub fn replace_tokens(&self, input: &str) -> Result<String, crate::DomainError> {
        self.replace_tokens_with_depth(input, 0, &mut HashSet::new())
    }

    fn replace_tokens_with_depth(
        &self,
        input: &str,
        depth: usize,
        visited: &mut HashSet<String>,
    ) -> Result<String, crate::DomainError> {
        if depth > Self::MAX_DEPTH {
            return Err(crate::DomainError::ValidationError(
                "Maximum token replacement depth exceeded (possible circular reference)"
                    .to_string(),
            ));
        }

        let mut result = input.to_string();

        for (key, value) in &self.tokens {
            let token = format!("{{{{{}}}}}", key);

            // Only check for circular reference if this token appears in the current input
            if result.contains(&token) {
                if visited.contains(key) {
                    return Err(crate::DomainError::ValidationError(format!(
                        "Circular reference detected for token: {}",
                        key
                    )));
                }

                visited.insert(key.clone());
                let replaced_value = self.replace_tokens_with_depth(value, depth + 1, visited)?;
                result = result.replace(&token, &replaced_value);
                visited.remove(key);
            }
        }

        Ok(result)
    }
}

impl Default for TokenMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_name_valid() {
        let name = StyleName::new("default").unwrap();
        assert_eq!(name.as_str(), "default");
    }

    #[test]
    fn test_style_name_empty_rejected() {
        assert!(StyleName::new("").is_err());
        assert!(StyleName::new("   ").is_err());
    }

    #[test]
    fn test_style_name_too_long_rejected() {
        let long_name = "a".repeat(StyleName::MAX_LENGTH + 1);
        assert!(StyleName::new(&long_name).is_err());
    }

    #[test]
    fn test_style_name_display() {
        let name = StyleName::new("verbose").unwrap();
        assert_eq!(format!("{}", name), "verbose");
    }

    #[test]
    fn test_style_name_trimming() {
        let name = StyleName::new("  minimal  ").unwrap();
        assert_eq!(name.as_str(), "minimal");
    }

    #[test]
    fn test_token_map_replace() {
        let mut map = TokenMap::new();
        map.insert("feature", "authentication");
        map.insert("version", "1.0");

        let result = map
            .replace_tokens("Implementing {{feature}} v{{version}}")
            .unwrap();
        assert_eq!(result, "Implementing authentication v1.0");
    }

    #[test]
    fn test_token_map_nested() {
        let mut map = TokenMap::new();
        map.insert("inner", "world");
        map.insert("outer", "Hello {{inner}}");

        let result = map.replace_tokens("{{outer}}!").unwrap();
        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn test_token_map_circular_detection() {
        let mut map = TokenMap::new();
        map.insert("a", "{{b}}");
        map.insert("b", "{{a}}");

        let result = map.replace_tokens("{{a}}");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_map_max_depth() {
        let mut map = TokenMap::new();
        // Create a chain longer than MAX_DEPTH
        for i in 0..15 {
            map.insert(&format!("t{}", i), &format!("{{{{t{}}}}}", i + 1));
        }
        map.insert("t15", "value");

        let result = map.replace_tokens("{{t0}}");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_map_default() {
        let map = TokenMap::default_tokens();
        assert!(map.get("date").is_some());
        assert!(map.get("author").is_some());
    }

    #[test]
    fn test_token_map_no_tokens() {
        let map = TokenMap::new();
        let result = map.replace_tokens("No tokens here").unwrap();
        assert_eq!(result, "No tokens here");
    }

    #[test]
    fn test_token_map_unknown_token() {
        let map = TokenMap::new();
        let result = map.replace_tokens("{{unknown}}").unwrap();
        assert_eq!(result, "{{unknown}}");
    }
}
