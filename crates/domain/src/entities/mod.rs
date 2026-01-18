//! Domain entities.
//!
//! Entities are objects that have a distinct identity that runs through time
//! and different states.

pub mod escalation;
pub mod loop_state;
pub mod quality_gate;
pub mod session;
pub mod spec;
pub mod style;
pub mod task;
pub mod workflow;

// Re-export commonly used types
pub use escalation::EscalationLog;
pub use loop_state::LoopState;
pub use quality_gate::{CheckStatus, QualityCheck, QualityGate};
pub use session::Session;
pub use spec::Spec;
pub use style::Style;
pub use task::Task;
pub use workflow::Workflow;
