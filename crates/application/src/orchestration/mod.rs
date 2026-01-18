//! Orchestration module for managing multiple sessions and specs.
//!
//! This module provides the core orchestration functionality including:
//! - Session management
//! - Parallel spec execution
//! - Dependency resolution
//! - Escalation handling

pub mod config;
pub mod dependency_graph;
pub mod escalation;
pub mod monitor;
pub mod orchestrator;
pub mod state;

pub use config::OrchestratorConfig;
pub use dependency_graph::DependencyGraph;
pub use escalation::{Escalation, EscalationHandler, EscalationLevel};
pub use monitor::{MonitorEvent, MonitorProgress, SessionStatus};
pub use orchestrator::Orchestrator;
pub use state::{print_execution_plan, restore_state, save_state, OrchestratorState};
