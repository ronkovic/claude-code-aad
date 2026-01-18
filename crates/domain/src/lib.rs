//! Domain layer for the AI-driven development tool.
//!
//! This crate contains the core domain models and business logic following
//! Clean Architecture principles.

pub mod entities;
pub mod error;
pub mod repositories;
pub mod value_objects;

// Re-export commonly used types
pub use error::{DomainError, Result};
