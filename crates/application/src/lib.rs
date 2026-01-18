//! Application layer for the AI-driven development tool.
//!
//! This crate contains use case implementations and application logic following
//! Clean Architecture principles.

pub mod error;
pub mod integration;
pub mod loop_engine;
pub mod orchestration;
pub mod quality;
pub mod workflow;

// Re-export commonly used types
pub use error::{ApplicationError, Result};
