//! File backup adapter for creating and managing backups.

use chrono::Utc;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::persistence::{PersistenceError, Result};

/// Adapter for creating and managing file backups.
///
/// Backups are stored in `.aad/backups/` with ISO 8601 timestamps.
pub struct BackupAdapter {
    backup_dir: PathBuf,
}

impl BackupAdapter {
    /// Default number of backup generations to keep.
    pub const DEFAULT_KEEP_COUNT: usize = 10;

    /// Creates a new BackupAdapter.
    ///
    /// # Arguments
    ///
    /// * `backup_dir` - Directory to store backup files (e.g., `.aad/backups`)
    pub fn new<P: AsRef<Path>>(backup_dir: P) -> Self {
        Self {
            backup_dir: backup_dir.as_ref().to_path_buf(),
        }
    }

    /// Creates a backup of the specified file.
    ///
    /// The backup file name format is: `<original-name>.<timestamp>.bak`
    /// where timestamp is in ISO 8601 format (with colons replaced by hyphens).
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file to backup
    ///
    /// # Returns
    ///
    /// The path to the created backup file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The source file does not exist
    /// - The backup directory cannot be created
    /// - The file cannot be copied
    pub async fn backup(&self, file_path: &Path) -> Result<PathBuf> {
        if !file_path.exists() {
            return Err(PersistenceError::BackupError(format!(
                "Source file does not exist: {}",
                file_path.display()
            )));
        }

        // Ensure backup directory exists
        if !self.backup_dir.exists() {
            fs::create_dir_all(&self.backup_dir).await?;
        }

        // Generate backup file name
        let file_name = file_path
            .file_name()
            .ok_or_else(|| {
                PersistenceError::InvalidFileName(format!(
                    "Invalid file path: {}",
                    file_path.display()
                ))
            })?
            .to_string_lossy();

        let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%S").to_string();
        let backup_name = format!("{}.{}.bak", file_name, timestamp);
        let backup_path = self.backup_dir.join(backup_name);

        // Copy file to backup location
        fs::copy(file_path, &backup_path).await?;

        Ok(backup_path)
    }

    /// Cleans up old backups, keeping only the most recent ones.
    ///
    /// # Arguments
    ///
    /// * `keep_count` - Number of recent backups to keep
    ///
    /// # Errors
    ///
    /// Returns an error if the backup directory cannot be read or files cannot be deleted.
    pub async fn cleanup_old_backups(&self, keep_count: usize) -> Result<()> {
        if !self.backup_dir.exists() {
            return Ok(());
        }

        // Get all backup files
        let mut backups = Vec::new();
        let mut entries = fs::read_dir(&self.backup_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("bak") {
                let metadata = fs::metadata(&path).await?;
                if let Ok(modified) = metadata.modified() {
                    backups.push((path, modified));
                }
            }
        }

        // Sort by modification time (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));

        // Delete old backups
        for (path, _) in backups.iter().skip(keep_count) {
            fs::remove_file(path).await?;
        }

        Ok(())
    }

    /// Lists all backup files for a given original file name.
    ///
    /// # Arguments
    ///
    /// * `original_name` - Name of the original file (e.g., "CLAUDE.md")
    ///
    /// # Returns
    ///
    /// A vector of backup file paths, sorted by modification time (newest first).
    pub async fn list_backups(&self, original_name: &str) -> Result<Vec<PathBuf>> {
        if !self.backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();
        let mut entries = fs::read_dir(&self.backup_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with(original_name) && file_name.ends_with(".bak") {
                    let metadata = fs::metadata(&path).await?;
                    if let Ok(modified) = metadata.modified() {
                        backups.push((path, modified));
                    }
                }
            }
        }

        // Sort by modification time (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(backups.into_iter().map(|(path, _)| path).collect())
    }

    /// Restores a file from a backup.
    ///
    /// # Arguments
    ///
    /// * `backup_path` - Path to the backup file
    /// * `target_path` - Path where the file should be restored
    ///
    /// # Errors
    ///
    /// Returns an error if the backup file cannot be copied.
    pub async fn restore(&self, backup_path: &Path, target_path: &Path) -> Result<()> {
        if !backup_path.exists() {
            return Err(PersistenceError::BackupError(format!(
                "Backup file does not exist: {}",
                backup_path.display()
            )));
        }

        // Ensure target directory exists
        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await?;
            }
        }

        fs::copy(backup_path, target_path).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_backup_creates_backup_file() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("test.txt");
        fs::write(&source_file, "test content").await.unwrap();

        let backup_dir = temp_dir.path().join("backups");
        let adapter = BackupAdapter::new(&backup_dir);

        let backup_path = adapter.backup(&source_file).await.unwrap();

        assert!(backup_path.exists());
        assert!(backup_path.to_string_lossy().contains("test.txt"));
        assert!(backup_path.to_string_lossy().ends_with(".bak"));

        let backup_content = fs::read_to_string(&backup_path).await.unwrap();
        assert_eq!(backup_content, "test content");
    }

    #[tokio::test]
    async fn test_backup_nonexistent_file_fails() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent = temp_dir.path().join("nonexistent.txt");
        let backup_dir = temp_dir.path().join("backups");
        let adapter = BackupAdapter::new(&backup_dir);

        let result = adapter.backup(&nonexistent).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cleanup_old_backups() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("test.txt");
        fs::write(&source_file, "content").await.unwrap();

        let backup_dir = temp_dir.path().join("backups");
        let adapter = BackupAdapter::new(&backup_dir);

        // Create 5 backups with slight delays
        for _ in 0..5 {
            adapter.backup(&source_file).await.unwrap();
            sleep(tokio::time::Duration::from_millis(1100)).await;
        }

        // Keep only 3
        adapter.cleanup_old_backups(3).await.unwrap();

        // Verify only 3 remain
        let mut entries = fs::read_dir(&backup_dir).await.unwrap();
        let mut count = 0;
        while entries.next_entry().await.unwrap().is_some() {
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_list_backups() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("CLAUDE.md");
        fs::write(&source_file, "content").await.unwrap();

        let backup_dir = temp_dir.path().join("backups");
        let adapter = BackupAdapter::new(&backup_dir);

        // Create 3 backups
        for _ in 0..3 {
            adapter.backup(&source_file).await.unwrap();
            sleep(tokio::time::Duration::from_millis(1100)).await;
        }

        let backups = adapter.list_backups("CLAUDE.md").await.unwrap();
        assert_eq!(backups.len(), 3);

        // Verify they are sorted newest first
        for backup in &backups {
            assert!(backup.to_string_lossy().contains("CLAUDE.md"));
            assert!(backup.to_string_lossy().ends_with(".bak"));
        }
    }

    #[tokio::test]
    async fn test_list_backups_filters_by_name() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "content1").await.unwrap();
        fs::write(&file2, "content2").await.unwrap();

        let backup_dir = temp_dir.path().join("backups");
        let adapter = BackupAdapter::new(&backup_dir);

        adapter.backup(&file1).await.unwrap();
        adapter.backup(&file2).await.unwrap();

        let backups1 = adapter.list_backups("file1.txt").await.unwrap();
        assert_eq!(backups1.len(), 1);

        let backups2 = adapter.list_backups("file2.txt").await.unwrap();
        assert_eq!(backups2.len(), 1);
    }

    #[tokio::test]
    async fn test_restore_backup() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("original.txt");
        fs::write(&source_file, "original content").await.unwrap();

        let backup_dir = temp_dir.path().join("backups");
        let adapter = BackupAdapter::new(&backup_dir);

        // Create backup
        let backup_path = adapter.backup(&source_file).await.unwrap();

        // Modify original
        fs::write(&source_file, "modified content").await.unwrap();

        // Restore from backup
        adapter.restore(&backup_path, &source_file).await.unwrap();

        let restored_content = fs::read_to_string(&source_file).await.unwrap();
        assert_eq!(restored_content, "original content");
    }

    #[tokio::test]
    async fn test_restore_nonexistent_backup_fails() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let adapter = BackupAdapter::new(&backup_dir);

        let nonexistent = temp_dir.path().join("backups/nonexistent.bak");
        let target = temp_dir.path().join("target.txt");

        let result = adapter.restore(&nonexistent, &target).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_backup_timestamp_format() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("test.txt");
        fs::write(&source_file, "content").await.unwrap();

        let backup_dir = temp_dir.path().join("backups");
        let adapter = BackupAdapter::new(&backup_dir);

        let backup_path = adapter.backup(&source_file).await.unwrap();
        let backup_name = backup_path.file_name().unwrap().to_string_lossy();

        // Should match pattern: test.txt.YYYY-MM-DDTHH-MM-SS.bak
        assert!(backup_name.starts_with("test.txt."));
        assert!(backup_name.contains("T"));
        assert!(backup_name.ends_with(".bak"));
    }
}
