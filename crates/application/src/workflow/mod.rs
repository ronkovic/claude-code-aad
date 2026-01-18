//! Workflow use cases and transition logic.

pub mod transition;

// Re-export commonly used functions
pub use transition::{auto_transition, can_transition, next_phase, transition};
