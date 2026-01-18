//! Persistence layer for JSON file-based storage.
//!
//! This module provides concrete implementations of repository traits
//! defined in the domain layer, using JSON files for storage.

mod backup_adapter;
mod error;
mod file_store;
mod session_json_repo;
mod spec_json_repo;
mod style_file_adapter;
mod task_json_repo;
mod token_replacer;

pub use backup_adapter::BackupAdapter;
pub use error::{PersistenceError, Result};
pub use file_store::FileStore;
pub use session_json_repo::SessionJsonRepo;
pub use spec_json_repo::SpecJsonRepo;
pub use style_file_adapter::StyleFileAdapter;
pub use task_json_repo::TaskJsonRepo;
pub use token_replacer::TokenReplacer;
