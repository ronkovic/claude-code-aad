//! Persist command implementation for session state management.

use chrono::{DateTime, Utc};
use domain::entities::{Session, Spec};
use domain::repositories::{SessionRepository, SpecRepository};
use infrastructure::persistence::{
    BackupAdapter, SessionJsonRepo, SpecJsonRepo, TaskJsonRepo,
};
use std::io::{self, Write};
use std::path::Path;

/// Saves all session state to persistent storage.
pub async fn save() -> anyhow::Result<()> {
    let data_dir = Path::new(".aad/data");
    let backup_dir = Path::new(".aad/backups");

    // Initialize repositories
    let spec_repo = SpecJsonRepo::new(data_dir.join("specs"));
    let task_repo = TaskJsonRepo::new(data_dir.join("tasks"));
    let session_repo = SessionJsonRepo::new(data_dir.join("sessions"));
    let backup_adapter = BackupAdapter::new(backup_dir);

    // Create backup of existing data if it exists
    if data_dir.exists() {
        println!("📦 既存データのバックアップを作成中...");

        // Backup specs directory
        for entry in std::fs::read_dir(data_dir.join("specs"))
            .unwrap_or_else(|_| std::fs::read_dir(".").unwrap())
        {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    backup_adapter.backup(&path).await?;
                }
            }
        }

        // Cleanup old backups (keep last 10)
        backup_adapter
            .cleanup_old_backups(BackupAdapter::DEFAULT_KEEP_COUNT)
            .await?;
    }

    println!("💾 セッション状態を保存中...");

    // Load existing data
    let specs: Vec<Spec> = spec_repo.find_all().await?;
    let active_sessions: Vec<Session> = session_repo.find_active().await?;

    // Save all specs to ensure persistence
    for spec in &specs {
        spec_repo.save(spec).await?;
    }

    // Save all active sessions
    for session in &active_sessions {
        session_repo.save(session).await?;
    }

    // TODO: Load and save tasks from .aad/tasks/ directory
    // Currently, tasks are not being persisted as there's no source to load from

    println!("✓ セッション状態を保存しました (.aad/data/)");
    println!("  • 仕様: {} 件", specs.len());
    println!("  • アクティブセッション: {} 件", active_sessions.len());

    Ok(())
}

/// Restores session state from a backup timestamp.
pub async fn restore(timestamp: &str) -> anyhow::Result<()> {
    let backup_dir = Path::new(".aad/backups");
    let data_dir = Path::new(".aad/data");

    if !backup_dir.exists() {
        anyhow::bail!("エラー: バックアップディレクトリが見つかりません");
    }

    // Parse timestamp
    let _parsed_timestamp: DateTime<Utc> = timestamp.parse().map_err(|_| {
        anyhow::anyhow!("エラー: 無効なタイムスタンプ形式です。ISO 8601形式を使用してください（例: 2026-01-18T10:30:00Z）")
    })?;

    // Find backup files matching the timestamp
    let backup_adapter = BackupAdapter::new(backup_dir);
    let mut backup_files = Vec::new();

    let mut entries = tokio::fs::read_dir(backup_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.contains(timestamp) && file_name.ends_with(".bak") {
                backup_files.push(path);
            }
        }
    }

    if backup_files.is_empty() {
        anyhow::bail!(
            "エラー: タイムスタンプ '{}' に対応するバックアップが見つかりません",
            timestamp
        );
    }

    // Confirm with user
    print!(
        "⚠  現在の状態は上書きされます。続行しますか? (y/N): "
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("キャンセルしました");
        return Ok(());
    }

    // Restore backups
    println!("📂 セッション状態を復元中...");

    for backup_path in &backup_files {
        let file_name = backup_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("無効なファイル名"))?;

        // Extract original file path from backup name
        // Format: <original-name>.<timestamp>.bak
        let parts: Vec<&str> = file_name.rsplitn(3, '.').collect();
        if parts.len() >= 3 {
            let original_name = parts[2];

            // Determine target path based on file name
            let target_path = if original_name.starts_with("SPEC-") {
                data_dir.join("specs").join(format!("{}.json", original_name))
            } else if original_name.starts_with("TASK-") {
                data_dir.join("tasks").join(format!("{}.json", original_name))
            } else {
                data_dir.join("sessions").join(format!("{}.json", original_name))
            };

            backup_adapter.restore(backup_path, &target_path).await?;
        }
    }

    println!("✓ セッション状態を復元しました");
    println!("  • 復元したファイル: {} 件", backup_files.len());

    Ok(())
}

/// Lists all available backups.
pub async fn list() -> anyhow::Result<()> {
    let backup_dir = Path::new(".aad/backups");

    if !backup_dir.exists() {
        println!("バックアップが見つかりません");
        return Ok(());
    }

    // Collect all backup files with their timestamps
    let mut backups: Vec<(String, std::time::SystemTime)> = Vec::new();
    let mut entries = tokio::fs::read_dir(backup_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("bak") {
            let metadata = tokio::fs::metadata(&path).await?;
            if let Ok(modified) = metadata.modified() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    backups.push((file_name.to_string(), modified));
                }
            }
        }
    }

    if backups.is_empty() {
        println!("バックアップが見つかりません");
        return Ok(());
    }

    // Group backups by timestamp
    use std::collections::HashMap;
    let mut grouped: HashMap<String, Vec<String>> = HashMap::new();

    for (file_name, _) in &backups {
        // Extract timestamp from filename
        // Format: <original-name>.<timestamp>.bak
        let parts: Vec<&str> = file_name.rsplitn(3, '.').collect();
        if parts.len() >= 2 {
            let timestamp = parts[1];
            grouped
                .entry(timestamp.to_string())
                .or_insert_with(Vec::new)
                .push(file_name.clone());
        }
    }

    // Sort timestamps
    let mut timestamps: Vec<_> = grouped.keys().collect();
    timestamps.sort_by(|a, b| b.cmp(a)); // Newest first

    println!("📋 バックアップ一覧:\n");

    for (idx, timestamp) in timestamps.iter().enumerate() {
        let files = &grouped[*timestamp];
        println!(
            "  {}. {} ({} ファイル)",
            idx + 1,
            timestamp,
            files.len()
        );

        // Extract spec IDs if available
        let mut spec_ids = Vec::new();
        for file in files {
            if file.starts_with("SPEC-") {
                let parts: Vec<&str> = file.split('.').collect();
                if !parts.is_empty() {
                    spec_ids.push(parts[0]);
                }
            }
        }

        if !spec_ids.is_empty() {
            println!("     仕様: {}", spec_ids.join(", "));
        }
    }

    println!();
    Ok(())
}
