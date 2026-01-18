//! Style entity for output formatting.

use crate::value_objects::{StyleName, TokenMap};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Style entity.
///
/// Represents a style configuration for output formatting with templates and tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    /// Name of this style.
    pub name: StyleName,
    /// Description of what this style does.
    pub description: String,
    /// Token mappings for template replacement.
    pub tokens: TokenMap,
    /// Path to the template file.
    pub template_path: PathBuf,
    /// When this style was created.
    pub created_at: DateTime<Utc>,
    /// When this style was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Style {
    /// Creates a new style.
    pub fn new(name: StyleName, description: String, template_path: PathBuf) -> Self {
        let now = Utc::now();
        Self {
            name,
            description,
            tokens: TokenMap::default_tokens(),
            template_path,
            created_at: now,
            updated_at: now,
        }
    }

    /// Applies this style to a template string.
    ///
    /// # Errors
    ///
    /// Returns an error if token replacement fails (e.g., circular references).
    pub fn apply(&self, template: &str) -> Result<String, crate::DomainError> {
        self.tokens.replace_tokens(template)
    }

    /// Updates the token map.
    pub fn update_tokens(&mut self, tokens: TokenMap) {
        self.tokens = tokens;
        self.updated_at = Utc::now();
    }

    /// Adds or updates a single token.
    pub fn set_token(&mut self, key: &str, value: &str) {
        self.tokens.insert(key, value);
        self.updated_at = Utc::now();
    }

    /// Gets a token value.
    pub fn get_token(&self, key: &str) -> Option<&str> {
        self.tokens.get(key)
    }

    /// Validates that the template path exists.
    ///
    /// Note: This is a simple check - in production, you might want to
    /// check file permissions, readability, etc.
    pub fn validate_template_path(&self) -> Result<(), crate::DomainError> {
        if !self.template_path.exists() {
            return Err(crate::DomainError::ValidationError(format!(
                "Template path does not exist: {}",
                self.template_path.display()
            )));
        }

        if !self.template_path.is_file() {
            return Err(crate::DomainError::ValidationError(format!(
                "Template path is not a file: {}",
                self.template_path.display()
            )));
        }

        Ok(())
    }

    /// Reads and applies the template from the file system.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or token replacement fails.
    pub fn apply_from_file(&self) -> Result<String, crate::DomainError> {
        self.validate_template_path()?;

        let template_content = std::fs::read_to_string(&self.template_path).map_err(|e| {
            crate::DomainError::Other(format!("Failed to read template file: {}", e))
        })?;

        self.apply(&template_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_creation() {
        let name = StyleName::new("default").unwrap();
        let path = PathBuf::from("/tmp/template.txt");
        let style = Style::new(name.clone(), "Default style".to_string(), path.clone());

        assert_eq!(style.name, name);
        assert_eq!(style.description, "Default style");
        assert_eq!(style.template_path, path);
    }

    #[test]
    fn test_style_apply_template() {
        let name = StyleName::new("test").unwrap();
        let mut style = Style::new(name, "Test".to_string(), PathBuf::from("/tmp/test"));

        style.set_token("name", "Alice");
        style.set_token("feature", "authentication");

        let result = style
            .apply("Hello {{name}}, working on {{feature}}")
            .unwrap();
        assert_eq!(result, "Hello Alice, working on authentication");
    }

    #[test]
    fn test_style_update_tokens() {
        let name = StyleName::new("test").unwrap();
        let mut style = Style::new(name, "Test".to_string(), PathBuf::from("/tmp/test"));

        let initial_updated = style.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));

        let mut new_tokens = TokenMap::new();
        new_tokens.insert("key", "value");
        style.update_tokens(new_tokens);

        assert!(style.updated_at > initial_updated);
        assert_eq!(style.get_token("key"), Some("value"));
    }

    #[test]
    fn test_style_set_token() {
        let name = StyleName::new("test").unwrap();
        let mut style = Style::new(name, "Test".to_string(), PathBuf::from("/tmp/test"));

        assert!(style.get_token("custom").is_none());

        style.set_token("custom", "value");
        assert_eq!(style.get_token("custom"), Some("value"));

        style.set_token("custom", "updated");
        assert_eq!(style.get_token("custom"), Some("updated"));
    }

    #[test]
    fn test_style_token_replacement() {
        let name = StyleName::new("test").unwrap();
        let mut style = Style::new(name, "Test".to_string(), PathBuf::from("/tmp/test"));

        style.set_token("x", "10");
        style.set_token("y", "20");

        let result = style.apply("Values: x={{x}}, y={{y}}").unwrap();
        assert_eq!(result, "Values: x=10, y=20");
    }

    #[test]
    fn test_style_template_path_validation() {
        let name = StyleName::new("test").unwrap();
        let style = Style::new(name, "Test".to_string(), PathBuf::from("/nonexistent/path"));

        assert!(style.validate_template_path().is_err());
    }

    #[test]
    fn test_style_apply_from_file() {
        let name = StyleName::new("test").unwrap();
        let mut style = Style::new(name, "Test".to_string(), PathBuf::from("/tmp/test"));

        // Create a temporary file
        let temp_path = std::env::temp_dir().join("test_template.txt");
        {
            use std::io::Write;
            let mut file = std::fs::File::create(&temp_path).unwrap();
            file.write_all(b"Hello {{name}}!").unwrap();
        }

        style.template_path = temp_path.clone();
        style.set_token("name", "World");

        let result = style.apply_from_file().unwrap();
        assert_eq!(result.trim(), "Hello World!");

        // Cleanup
        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_style_clone() {
        let name = StyleName::new("test").unwrap();
        let style = Style::new(name, "Test".to_string(), PathBuf::from("/tmp/test"));

        let cloned = style.clone();
        assert_eq!(style.name, cloned.name);
        assert_eq!(style.description, cloned.description);
        assert_eq!(style.template_path, cloned.template_path);
    }

    #[test]
    fn test_style_default_tokens() {
        let name = StyleName::new("test").unwrap();
        let style = Style::new(name, "Test".to_string(), PathBuf::from("/tmp/test"));

        // Default tokens should include date and author
        assert!(style.get_token("date").is_some());
        assert!(style.get_token("author").is_some());
    }
}
