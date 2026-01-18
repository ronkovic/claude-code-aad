//! File storage trait for persistence operations.

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

use crate::persistence::PersistenceError;

/// Trait for file-based storage operations.
///
/// This trait provides a common interface for reading and writing
/// data to files in JSON format.
#[async_trait]
pub trait FileStore: Send + Sync {
    /// Reads data from a file.
    ///
    /// # Errors
    ///
    /// Returns `PersistenceError::FileNotFound` if the file does not exist.
    /// Returns `PersistenceError::DeserializationError` if the file content cannot be parsed.
    async fn read<T: DeserializeOwned>(&self, path: &Path) -> Result<T, PersistenceError>;

    /// Writes data to a file.
    ///
    /// Creates parent directories if they don't exist.
    ///
    /// # Errors
    ///
    /// Returns `PersistenceError::IoError` if the file cannot be written.
    /// Returns `PersistenceError::SerializationError` if the data cannot be serialized.
    async fn write<T: Serialize>(&self, path: &Path, data: &T) -> Result<(), PersistenceError>;

    /// Deletes a file.
    ///
    /// Returns `Ok(())` even if the file does not exist.
    ///
    /// # Errors
    ///
    /// Returns `PersistenceError::IoError` if the file cannot be deleted.
    async fn delete(&self, path: &Path) -> Result<(), PersistenceError>;

    /// Checks if a file exists.
    async fn exists(&self, path: &Path) -> bool;

    /// Lists all files in a directory with a given extension.
    ///
    /// # Errors
    ///
    /// Returns `PersistenceError::IoError` if the directory cannot be read.
    async fn list_files(
        &self,
        dir: &Path,
        extension: &str,
    ) -> Result<Vec<std::path::PathBuf>, PersistenceError>;
}
