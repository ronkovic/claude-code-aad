//! Infrastructure layer for the AI-driven development tool.
//!
//! This crate contains infrastructure concerns such as configuration management,
//! file I/O, and external service integrations following Clean Architecture principles.

pub mod adapters;
pub mod config;
pub mod error;
pub mod persistence;

// Re-export commonly used types
pub use error::{InfrastructureError, Result};
