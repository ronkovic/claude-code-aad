//! AadConfig structure for managing AAD tool configuration.

use crate::config::validation::{validate_not_empty, validate_range, Validate};
use crate::error::{InfrastructureError, Result};
use domain::value_objects::Phase;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// AAD configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AadConfig {
    /// Configuration version.
    pub version: String,
    /// Context usage threshold (0-100).
    #[serde(default = "default_context_threshold")]
    pub context_threshold: u8,
    /// Default Git branch.
    pub default_branch: Option<String>,
    /// Workflow configuration.
    #[serde(default)]
    pub workflow: WorkflowConfig,
}

/// Workflow configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    /// Workflow phases in order.
    #[serde(default = "default_phases")]
    pub phases: Vec<Phase>,
    /// Enable automatic phase transitions.
    #[serde(default)]
    pub auto_transition: bool,
}

impl Default for AadConfig {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            context_threshold: default_context_threshold(),
            default_branch: Some("main".to_string()),
            workflow: WorkflowConfig::default(),
        }
    }
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            phases: default_phases(),
            auto_transition: false,
        }
    }
}

impl AadConfig {
    /// Loads configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File does not exist
    /// - File cannot be read
    /// - TOML parsing fails
    /// - Required fields are missing
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(InfrastructureError::Config(format!(
                "設定ファイル '{}' が見つかりません",
                path.display()
            )));
        }

        let content = fs::read_to_string(path).map_err(|e| {
            InfrastructureError::Config(format!(
                "設定ファイル '{}' の読み込みに失敗しました: {}",
                path.display(),
                e
            ))
        })?;

        let config: AadConfig = toml::from_str(&content)?;

        // Validate context_threshold range
        if config.context_threshold > 100 {
            return Err(InfrastructureError::Validation(format!(
                "'context_threshold' の値 {} は範囲外です（0〜100）",
                config.context_threshold
            )));
        }

        Ok(config)
    }

    /// Saves configuration to a TOML file.
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
                "設定ファイル '{}' の書き込みに失敗しました: {}",
                path.display(),
                e
            ))
        })?;

        Ok(())
    }
}

fn default_context_threshold() -> u8 {
    70
}

fn default_phases() -> Vec<Phase> {
    Phase::all()
}

impl Validate for AadConfig {
    fn validate(&self) -> Result<()> {
        // Validate version is not empty
        validate_not_empty("version", &self.version)?;

        // Validate context_threshold is in range 0-100
        validate_range("context_threshold", self.context_threshold as i64, 0, 100)?;

        // Validate workflow configuration
        self.workflow.validate()?;

        Ok(())
    }
}

impl Validate for WorkflowConfig {
    fn validate(&self) -> Result<()> {
        // Validate phases is not empty
        if self.phases.is_empty() {
            return Err(InfrastructureError::Validation(
                "ワークフローには少なくとも1つのフェーズが必要です".to_string(),
            ));
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
    fn test_aad_config_default() {
        let config = AadConfig::default();

        assert_eq!(config.version, "0.1.0");
        assert_eq!(config.context_threshold, 70);
        assert_eq!(config.default_branch, Some("main".to_string()));
        assert_eq!(config.workflow.phases.len(), 6);
        assert!(!config.workflow.auto_transition);
    }

    #[test]
    fn test_aad_config_load_valid() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
version = "0.2.0"
context_threshold = 80
default_branch = "develop"

[workflow]
auto_transition = true
"#
        )
        .unwrap();

        let config = AadConfig::load(temp_file.path()).unwrap();

        assert_eq!(config.version, "0.2.0");
        assert_eq!(config.context_threshold, 80);
        assert_eq!(config.default_branch, Some("develop".to_string()));
        assert!(config.workflow.auto_transition);
    }

    #[test]
    fn test_aad_config_load_file_not_found() {
        let result = AadConfig::load(Path::new("/nonexistent/path/config.toml"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, InfrastructureError::Config(_)));
    }

    #[test]
    fn test_aad_config_load_invalid_toml() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "invalid toml [[[").unwrap();

        let result = AadConfig::load(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_aad_config_context_threshold_out_of_range() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
version = "0.1.0"
context_threshold = 150
"#
        )
        .unwrap();

        let result = AadConfig::load(temp_file.path());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, InfrastructureError::Validation(_)));
    }

    #[test]
    fn test_aad_config_save() {
        let config = AadConfig::default();
        let temp_file = NamedTempFile::new().unwrap();

        config.save(temp_file.path()).unwrap();

        let loaded = AadConfig::load(temp_file.path()).unwrap();
        assert_eq!(loaded.version, config.version);
        assert_eq!(loaded.context_threshold, config.context_threshold);
    }

    #[test]
    fn test_workflow_config_default() {
        let config = WorkflowConfig::default();

        assert_eq!(config.phases, Phase::all());
        assert!(!config.auto_transition);
    }

    #[test]
    fn test_aad_config_validate_success() {
        let config = AadConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_aad_config_validate_empty_version() {
        let mut config = AadConfig::default();
        config.version = "".to_string();

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_aad_config_validate_threshold_negative() {
        // Note: u8 can't be negative, but we test the validation logic
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
version = "0.1.0"
context_threshold = -10
"#
        )
        .unwrap();

        let result = AadConfig::load(temp_file.path());
        // TOML parsing will fail for negative u8
        assert!(result.is_err());
    }

    #[test]
    fn test_workflow_config_validate_empty_phases() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
version = "0.1.0"

[workflow]
phases = []
"#
        )
        .unwrap();

        let result = AadConfig::load(temp_file.path());
        assert!(result.is_ok());

        let config = result.unwrap();
        let validation_result = config.validate();
        assert!(validation_result.is_err());
    }
}
