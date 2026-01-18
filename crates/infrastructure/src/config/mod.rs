//! Configuration management module.

pub mod aad_config;
pub mod style_config;
pub mod validation;

// Re-export commonly used types
pub use aad_config::{AadConfig, WorkflowConfig};
pub use style_config::{StyleConfig, StyleDefinition};
pub use validation::{Validate, ValidationError};
