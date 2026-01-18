//! Domain value objects.
//!
//! Value objects are immutable objects that are defined by their attributes
//! rather than a unique identity.

pub mod ids;
pub mod phase;
pub mod priority;
pub mod status;
pub mod style;

// Re-export commonly used types
pub use ids::{SpecId, TaskId};
pub use phase::Phase;
pub use priority::Priority;
pub use status::Status;
pub use style::{StyleName, TokenMap};
