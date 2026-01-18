//! StyleConfig structure for managing output styles.

use crate::config::validation::Validate;
use crate::error::{InfrastructureError, Result};
use domain::value_objects::{StyleName, TokenMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Style configuration structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StyleConfig {
    /// Map of style names to their definitions.
    #[serde(default)]
    pub styles: HashMap<String, StyleDefinition>,
}

/// Style definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleDefinition {
    /// Optional description of the style.
    pub description: Option<String>,
    /// Token replacements for this style.
    #[serde(default)]
    pub tokens: HashMap<String, String>,
}

impl StyleConfig {
    /// Loads style configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File does not exist
    /// - File cannot be read
    /// - TOML parsing fails
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(InfrastructureError::Config(format!(
                "スタイル設定ファイル '{}' が見つかりません",
                path.display()
            )));
        }

        let content = fs::read_to_string(path).map_err(|e| {
            InfrastructureError::Config(format!(
                "スタイル設定ファイル '{}' の読み込みに失敗しました: {}",
                path.display(),
                e
            ))
        })?;

        let config: StyleConfig = toml::from_str(&content)?;

        Ok(config)
    }

    /// Gets a TokenMap for the specified style name.
    ///
    /// Returns None if the style is not defined.
    pub fn get_token_map(&self, name: &StyleName) -> Option<TokenMap> {
        let style_def = self.styles.get(name.as_str())?;

        let mut token_map = TokenMap::new();
        for (key, value) in &style_def.tokens {
            token_map.insert(key, value);
        }

        Some(token_map)
    }

    /// Checks if a style is defined.
    pub fn has_style(&self, name: &StyleName) -> bool {
        self.styles.contains_key(name.as_str())
    }

    /// Returns all defined style names.
    pub fn style_names(&self) -> Vec<StyleName> {
        self.styles
            .keys()
            .filter_map(|k| StyleName::new(k).ok())
            .collect()
    }

    /// Checks the configuration and returns warnings.
    ///
    /// Checks for:
    /// - Token name duplicates across styles (warning)
    /// - Invalid style names
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn check_warnings(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate style names
        for style_name in self.styles.keys() {
            if StyleName::new(style_name).is_err() {
                return Err(InfrastructureError::Validation(format!(
                    "無効なスタイル名: '{}'",
                    style_name
                )));
            }
        }

        // Check for token duplicates (informational)
        let mut all_tokens = HashMap::new();
        for (style_name, style_def) in &self.styles {
            for token_name in style_def.tokens.keys() {
                if let Some(existing_style) = all_tokens.get(token_name) {
                    if existing_style != style_name {
                        warnings.push(format!(
                            "トークン '{}' はスタイル '{}' と '{}' の両方で定義されています",
                            token_name, existing_style, style_name
                        ));
                    }
                }
                all_tokens.insert(token_name.clone(), style_name.clone());
            }
        }

        Ok(warnings)
    }

    /// Saves style configuration to a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or file writing fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).map_err(|e| {
            InfrastructureError::Config(format!("TOML シリアライズに失敗しました: {}", e))
        })?;

        fs::write(path, content).map_err(|e| {
            InfrastructureError::Config(format!(
                "スタイル設定ファイル '{}' の書き込みに失敗しました: {}",
                path.display(),
                e
            ))
        })?;

        Ok(())
    }
}

impl Validate for StyleConfig {
    fn validate(&self) -> Result<()> {
        // Validate style names
        for style_name in self.styles.keys() {
            if StyleName::new(style_name).is_err() {
                return Err(InfrastructureError::Validation(format!(
                    "無効なスタイル名: '{}'",
                    style_name
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_style_config_default() {
        let config = StyleConfig::default();
        assert!(config.styles.is_empty());
    }

    #[test]
    fn test_style_config_load_valid() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[styles.standard]
description = "Standard output style"

[styles.standard.tokens]
feature = "認証機能"
version = "1.0"

[styles.sage]
description = "Sage style output"

[styles.sage.tokens]
emoji_success = "✅"
emoji_warning = "⚠️"
"#
        )
        .unwrap();

        let config = StyleConfig::load(temp_file.path()).unwrap();

        assert_eq!(config.styles.len(), 2);
        assert!(config.styles.contains_key("standard"));
        assert!(config.styles.contains_key("sage"));

        let standard = &config.styles["standard"];
        assert_eq!(
            standard.description,
            Some("Standard output style".to_string())
        );
        assert_eq!(standard.tokens.len(), 2);
    }

    #[test]
    fn test_style_config_load_file_not_found() {
        let result = StyleConfig::load(Path::new("/nonexistent/styles.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_style_config_get_token_map() {
        let mut config = StyleConfig::default();

        let mut tokens = HashMap::new();
        tokens.insert("feature".to_string(), "認証".to_string());
        tokens.insert("version".to_string(), "2.0".to_string());

        config.styles.insert(
            "test".to_string(),
            StyleDefinition {
                description: None,
                tokens,
            },
        );

        let style_name = StyleName::new("test").unwrap();
        let token_map = config.get_token_map(&style_name).unwrap();

        assert_eq!(token_map.get("feature"), Some("認証"));
        assert_eq!(token_map.get("version"), Some("2.0"));
    }

    #[test]
    fn test_style_config_get_token_map_undefined_style() {
        let config = StyleConfig::default();
        let style_name = StyleName::new("undefined").unwrap();

        assert!(config.get_token_map(&style_name).is_none());
    }

    #[test]
    fn test_style_config_has_style() {
        let mut config = StyleConfig::default();
        config.styles.insert(
            "test".to_string(),
            StyleDefinition {
                description: None,
                tokens: HashMap::new(),
            },
        );

        let test_name = StyleName::new("test").unwrap();
        let undefined_name = StyleName::new("undefined").unwrap();

        assert!(config.has_style(&test_name));
        assert!(!config.has_style(&undefined_name));
    }

    #[test]
    fn test_style_config_style_names() {
        let mut config = StyleConfig::default();
        config.styles.insert(
            "standard".to_string(),
            StyleDefinition {
                description: None,
                tokens: HashMap::new(),
            },
        );
        config.styles.insert(
            "sage".to_string(),
            StyleDefinition {
                description: None,
                tokens: HashMap::new(),
            },
        );

        let names = config.style_names();
        assert_eq!(names.len(), 2);
        assert!(names.iter().any(|n| n.as_str() == "standard"));
        assert!(names.iter().any(|n| n.as_str() == "sage"));
    }

    #[test]
    fn test_style_config_validate_duplicate_tokens() {
        let mut config = StyleConfig::default();

        let mut tokens1 = HashMap::new();
        tokens1.insert("shared".to_string(), "value1".to_string());

        let mut tokens2 = HashMap::new();
        tokens2.insert("shared".to_string(), "value2".to_string());

        config.styles.insert(
            "style1".to_string(),
            StyleDefinition {
                description: None,
                tokens: tokens1,
            },
        );
        config.styles.insert(
            "style2".to_string(),
            StyleDefinition {
                description: None,
                tokens: tokens2,
            },
        );

        let warnings = config.check_warnings().unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("shared"));
    }

    #[test]
    fn test_style_config_validate_invalid_style_name() {
        let mut config = StyleConfig::default();

        // Style name that's too long
        let long_name = "a".repeat(StyleName::MAX_LENGTH + 1);
        config.styles.insert(
            long_name,
            StyleDefinition {
                description: None,
                tokens: HashMap::new(),
            },
        );

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_style_config_save() {
        let mut config = StyleConfig::default();
        let mut tokens = HashMap::new();
        tokens.insert("key".to_string(), "value".to_string());

        config.styles.insert(
            "test".to_string(),
            StyleDefinition {
                description: Some("Test style".to_string()),
                tokens,
            },
        );

        let temp_file = NamedTempFile::new().unwrap();
        config.save(temp_file.path()).unwrap();

        let loaded = StyleConfig::load(temp_file.path()).unwrap();
        assert_eq!(loaded.styles.len(), 1);
        assert!(loaded.styles.contains_key("test"));
    }

    #[test]
    fn test_style_config_empty_styles() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[styles]").unwrap();

        let config = StyleConfig::load(temp_file.path()).unwrap();
        assert!(config.styles.is_empty());
    }
}
