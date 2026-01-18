//! Domain entities.
//!
//! Entities are objects that have a distinct identity that runs through time
//! and different states.

pub mod session;
pub mod spec;
pub mod style;
pub mod task;
pub mod workflow;

// Re-export commonly used types
pub use session::Session;
pub use spec::Spec;
pub use style::Style;
pub use task::Task;
pub use workflow::Workflow;
