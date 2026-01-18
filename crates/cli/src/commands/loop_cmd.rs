//! Loop command implementation.
//!
//! Executes tasks in a loop with support for pause/resume functionality.

use anyhow::Result;
use application::loop_engine::LoopEngine;
use domain::entities::Task;
use domain::value_objects::SpecId;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;

/// Executes the loop command to run tasks in a loop.
///
/// # Arguments
///
/// * `spec_id` - The specification ID to run the loop for (e.g., "SPEC-001")
/// * `resume` - Whether to resume a previous loop session
///
/// # Examples
///
/// ```bash
/// aad loop SPEC-001
/// aad loop SPEC-001 --resume
/// ```
pub async fn execute(spec_id: &str, resume: bool) -> Result<()> {
    // Parse spec ID
    let spec_id_parsed = SpecId::from_str(spec_id)
        .map_err(|e| anyhow::anyhow!("無効なSpec ID '{}': {}", spec_id, e))?;

    if resume {
        execute_resume().await
    } else {
        execute_normal(spec_id_parsed).await
    }
}

/// Executes the loop in resume mode.
async fn execute_resume() -> Result<()> {
    println!("🔄 ループ再開モード\n");

    let state_file = PathBuf::from(".aad/loop-state.json");
    if !state_file.exists() {
        anyhow::bail!(
            "エラー: 状態ファイルが見つかりません: {}\n💡 Tip: 最初に --resume なしで実行してください",
            state_file.display()
        );
    }

    // Load state
    let mut engine = LoopEngine::load(state_file)?;

    println!("📋 復元した状態:");
    println!("  - Spec ID: {}", engine.state().spec_id);
    println!("  - 待機中タスク: {}", engine.state().pending_count());
    if let Some(current) = &engine.state().current_task {
        println!("  - 現在実行中: {}", current);
    }
    println!();

    if engine.state().is_queue_empty() && engine.state().current_task.is_none() {
        println!("✅ すべてのタスクが完了しています");
        return Ok(());
    }

    // Resume the loop
    println!("▶️  ループを再開します\n");

    // TODO: Load tasks from .aad/tasks/SPEC-XXX/
    // For now, we just resume with an empty task list
    let tasks: Vec<Task> = vec![];

    run_with_signal_handling(&mut engine, tasks).await
}

/// Executes the loop in normal mode.
async fn execute_normal(spec_id: SpecId) -> Result<()> {
    println!("🚀 ループ開始\n");
    println!("📋 Spec ID: {}\n", spec_id);

    // Create engine
    let mut engine = LoopEngine::new(spec_id);

    // TODO: Load tasks from .aad/tasks/SPEC-XXX/
    // For now, we use an empty task list
    let tasks: Vec<Task> = vec![];

    println!("📝 タスク数: {}\n", tasks.len());

    if tasks.is_empty() {
        println!("⚠️  タスクが見つかりません");
        return Ok(());
    }

    run_with_signal_handling(&mut engine, tasks).await
}

/// Runs the loop with Ctrl+C signal handling.
async fn run_with_signal_handling(engine: &mut LoopEngine, tasks: Vec<Task>) -> Result<()> {
    // Set up Ctrl+C handler
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    tokio::spawn(async move {
        if signal::ctrl_c().await.is_ok() {
            println!("\n\n⚠️  Ctrl+C を検出しました。ループを中断します...");
            shutdown_clone.store(true, Ordering::SeqCst);
        }
    });

    // Run the loop
    // TODO: Implement actual task execution loop with progress display
    engine.run_loop(tasks).await?;

    if shutdown.load(Ordering::SeqCst) {
        println!("\n💾 状態を保存しました");
        println!(
            "💡 再開するには: aad loop {} --resume",
            engine.state().spec_id
        );
    } else {
        println!("\n✅ すべてのタスクが完了しました");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_invalid_spec_id() {
        let result = execute("", false).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("無効なSpec ID"));
    }

    #[tokio::test]
    async fn test_execute_resume_without_state() {
        // Clean up state file if exists
        let state_file = PathBuf::from(".aad/loop-state.json");
        let _ = std::fs::remove_file(&state_file);

        let result = execute("SPEC-001", true).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("状態ファイルが見つかりません"));
    }

    #[tokio::test]
    async fn test_execute_normal_with_empty_tasks() {
        // This test just ensures the command doesn't crash with empty tasks
        let result = execute("SPEC-001", false).await;
        // Should succeed even with empty tasks
        assert!(result.is_ok());
    }
}
