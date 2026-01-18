//! Specification repository trait.

use crate::entities::Spec;
use crate::value_objects::SpecId;
use crate::Result;

/// Repository interface for Spec entities.
///
/// This trait defines the contract for persisting and retrieving specifications.
/// Concrete implementations belong in the infrastructure layer.
#[async_trait::async_trait]
pub trait SpecRepository: Send + Sync {
    /// Finds a specification by its ID.
    ///
    /// Returns `None` if the specification does not exist.
    async fn find_by_id(&self, id: &SpecId) -> Result<Option<Spec>>;

    /// Finds all specifications.
    async fn find_all(&self) -> Result<Vec<Spec>>;

    /// Saves a specification.
    ///
    /// If the specification already exists, it will be updated.
    async fn save(&self, spec: &Spec) -> Result<()>;

    /// Deletes a specification by its ID.
    ///
    /// Returns `Ok(())` even if the specification does not exist.
    async fn delete(&self, id: &SpecId) -> Result<()>;
}
