//! Orchestration module for managing multiple sessions and specs.
//!
//! This module provides the core orchestration functionality including:
//! - Session management
//! - Parallel spec execution
//! - Dependency resolution
//! - Escalation handling

pub mod config;
pub mod dependency_graph;
pub mod monitor;
pub mod orchestrator;

pub use config::OrchestratorConfig;
pub use dependency_graph::DependencyGraph;
pub use monitor::{MonitorEvent, MonitorProgress, SessionStatus};
pub use orchestrator::Orchestrator;
