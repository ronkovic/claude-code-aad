//! Repository trait definitions.
//!
//! Repositories provide an abstraction for data persistence operations.
//! These are trait definitions only; concrete implementations belong in
//! the infrastructure layer.

pub mod session_repository;
pub mod spec_repository;
pub mod task_repository;

// Re-export repository traits
pub use session_repository::SessionRepository;
pub use spec_repository::SpecRepository;
pub use task_repository::TaskRepository;
