//! Orchestrate command implementation.
//!
//! Executes multiple specifications concurrently using the orchestration engine.

use application::orchestration::{
    print_execution_plan, restore_state, save_state, Orchestrator, OrchestratorConfig,
    OrchestratorState, SessionStatus,
};
use domain::value_objects::{ids::SpecId, phase::Phase};
use std::str::FromStr;
use std::time::Duration;

/// Executes the orchestrate command to run multiple specs concurrently.
///
/// # Arguments
///
/// * `spec_ids` - List of specification IDs to execute (e.g., ["SPEC-001", "SPEC-002"])
/// * `resume` - Whether to resume a previous orchestration session
/// * `dry_run` - Whether to only show the execution plan without running
///
/// # Examples
///
/// ```bash
/// aad orchestrate --specs SPEC-001 SPEC-002
/// aad orchestrate --resume
/// aad orchestrate --specs SPEC-001 SPEC-002 --dry-run
/// ```
pub async fn execute(spec_ids: &[String], resume: bool, dry_run: bool) -> anyhow::Result<()> {
    // Handle resume mode
    if resume {
        return execute_resume(dry_run).await;
    }

    // Validate spec_ids
    if spec_ids.is_empty() {
        anyhow::bail!("エラー: 少なくとも1つのSpec IDを指定してください");
    }

    // Handle dry-run mode
    if dry_run {
        return execute_dry_run(spec_ids).await;
    }

    // Normal execution mode
    execute_normal(spec_ids).await
}

/// Executes orchestration in resume mode.
async fn execute_resume(dry_run: bool) -> anyhow::Result<()> {
    println!("🔄 オーケストレーション再開モード\n");

    // Restore state
    let state = match restore_state(None) {
        Ok(state) => state,
        Err(e) => {
            anyhow::bail!("エラー: 状態ファイルが見つかりません: {}\n💡 Tip: 最初に --specs オプションで実行してください", e);
        }
    };

    println!("📋 復元した状態:");
    println!("  - 保存日時: {}", state.saved_at);
    println!("  - 全体: {} specs", state.spec_ids.len());
    println!("  - 完了: {} specs", state.completed.len());
    println!("  - 失敗: {} specs", state.failed.len());
    println!("  - 実行中: {} specs", state.running.len());
    println!("  - 待機中: {} specs", state.pending.len());
    println!();

    if state.is_complete() {
        println!("✅ すべてのセッションは既に完了しています");
        return Ok(());
    }

    if dry_run {
        // Show remaining execution plan
        let mut remaining_state = state.clone();
        remaining_state.spec_ids = remaining_state.remaining_specs();
        print_execution_plan(&remaining_state);
        return Ok(());
    }

    // Resume remaining specs
    let remaining = state.remaining_specs();
    println!("▶️  残りの {} specs を実行します\n", remaining.len());

    execute_normal(&remaining).await
}

/// Executes orchestration in dry-run mode.
async fn execute_dry_run(spec_ids: &[String]) -> anyhow::Result<()> {
    let state = OrchestratorState::new(spec_ids.iter().cloned().collect());

    // TODO: Load dependencies from .aad/specs/SPEC-XXX/dependencies.json if exists
    // For now, assume no dependencies

    print_execution_plan(&state);
    Ok(())
}

/// Executes orchestration in normal mode.
async fn execute_normal(spec_ids: &[String]) -> anyhow::Result<()> {
    println!("🚀 オーケストレーション開始\n");
    println!("📋 実行対象:");
    for spec_id in spec_ids {
        println!("  - {}", spec_id);
    }
    println!();

    // Create initial state
    let mut state = OrchestratorState::new(spec_ids.iter().cloned().collect());

    // 1. Create orchestrator with default config
    let config = OrchestratorConfig::default();
    let orchestrator = std::sync::Arc::new(Orchestrator::new(config.clone()));

    println!(
        "⚙️  設定: 最大並列数 = {}, タイムアウト = {}秒\n",
        config.max_parallel_sessions, config.session_timeout_secs
    );

    // 2. Register all specs
    println!("📝 セッション登録中...");
    let mut session_ids = Vec::new();
    for spec_id_str in spec_ids {
        let spec_id = SpecId::from_str(spec_id_str)
            .map_err(|e| anyhow::anyhow!("無効なSpec ID '{}': {}", spec_id_str, e))?;

        match orchestrator.register_spec(&spec_id, Phase::Tdd).await {
            Ok(session_id) => {
                println!("  ✓ {} -> {}", spec_id_str, session_id);
                session_ids.push(session_id);
                state
                    .spec_phases
                    .insert(spec_id_str.clone(), "TDD".to_string());
            }
            Err(e) => {
                eprintln!("  ✗ {} の登録に失敗: {}", spec_id_str, e);
                return Err(anyhow::anyhow!(
                    "セッション登録エラー: {} - {}",
                    spec_id_str,
                    e
                ));
            }
        }
    }
    println!();

    // Save initial state
    if let Err(e) = save_state(&state, None) {
        eprintln!("⚠️  警告: 状態の保存に失敗しました: {}", e);
    }

    // 3. Start all sessions
    println!("▶️  セッション開始中...");
    match orchestrator.start_all_sessions().await {
        Ok(started_ids) => {
            println!("  ✓ {} セッションを開始しました\n", started_ids.len());
        }
        Err(e) => {
            eprintln!("  ✗ セッション開始エラー: {}", e);
            return Err(anyhow::anyhow!("セッション開始失敗: {}", e));
        }
    }

    // 4. Monitor sessions until all complete
    println!("🔍 セッション監視中...\n");

    // Start monitor loop in background
    let monitor_orchestrator = orchestrator.clone();
    tokio::spawn(async move {
        monitor_orchestrator.monitor_loop().await;
    });

    // Wait for all sessions to complete
    loop {
        tokio::time::sleep(Duration::from_secs(2)).await;

        let all_sessions = orchestrator.get_all_sessions().await;
        let mut all_done = true;
        let mut completed = 0;
        let mut failed = 0;
        let mut timed_out = 0;
        let mut running = 0;
        let mut pending = 0;

        // Update state based on session statuses
        state.completed.clear();
        state.failed.clear();
        state.running.clear();
        state.pending.clear();

        for session in &all_sessions {
            let spec_id = session.spec_id.to_string();
            if let Some(status) = orchestrator.get_session_status(&session.id).await {
                match status {
                    SessionStatus::Completed => {
                        completed += 1;
                        state.mark_completed(&spec_id);
                    }
                    SessionStatus::Failed | SessionStatus::TimedOut => {
                        if matches!(status, SessionStatus::Failed) {
                            failed += 1;
                        } else {
                            timed_out += 1;
                        }
                        state.mark_failed(&spec_id);
                    }
                    SessionStatus::Running => {
                        running += 1;
                        state.mark_running(&spec_id);
                        all_done = false;
                    }
                    SessionStatus::Pending => {
                        pending += 1;
                        all_done = false;
                    }
                }
            }
        }

        // Save updated state
        if let Err(e) = save_state(&state, None) {
            eprintln!("⚠️  警告: 状態の保存に失敗しました: {}", e);
        }

        // Print progress
        print!("\r進捗: ");
        if completed > 0 {
            print!("✅ {}", completed);
        }
        if running > 0 {
            print!(" 🔄 {}", running);
        }
        if pending > 0 {
            print!(" ⏳ {}", pending);
        }
        if failed > 0 {
            print!(" ❌ {}", failed);
        }
        if timed_out > 0 {
            print!(" ⏰ {}", timed_out);
        }
        print!("   ");
        std::io::Write::flush(&mut std::io::stdout())?;

        if all_done {
            println!("\n");
            break;
        }
    }

    // 5. Display final summary
    println!("📊 実行結果サマリー\n");
    println!("┌─────────────────────────────────────┐");

    let all_sessions = orchestrator.get_all_sessions().await;
    for session in &all_sessions {
        if let Some(status) = orchestrator.get_session_status(&session.id).await {
            let status_icon = match status {
                SessionStatus::Completed => "✅",
                SessionStatus::Failed => "❌",
                SessionStatus::TimedOut => "⏰",
                SessionStatus::Running => "🔄",
                SessionStatus::Pending => "⏳",
            };

            let status_text = match status {
                SessionStatus::Completed => "完了",
                SessionStatus::Failed => "失敗",
                SessionStatus::TimedOut => "タイムアウト",
                SessionStatus::Running => "実行中",
                SessionStatus::Pending => "待機中",
            };

            println!("│ {} {} - {:8} │", status_icon, &session.id, status_text);
        }
    }

    println!("└─────────────────────────────────────┘\n");

    // Check if any sessions failed
    let failed_count = all_sessions
        .iter()
        .filter(|s| {
            if let Some(status) =
                futures::executor::block_on(orchestrator.get_session_status(&s.id))
            {
                matches!(status, SessionStatus::Failed | SessionStatus::TimedOut)
            } else {
                false
            }
        })
        .count();

    if failed_count > 0 {
        eprintln!("⚠️  警告: {} セッションが失敗しました", failed_count);
        eprintln!("詳細は .aad/sessions/ および .aad/escalations/ を確認してください");
        return Err(anyhow::anyhow!("{} セッションが失敗しました", failed_count));
    }

    println!("✅ すべてのセッションが正常に完了しました");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_empty_specs() {
        let result = execute(&[], false, false).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("少なくとも1つのSpec ID"));
    }

    #[tokio::test]
    async fn test_execute_invalid_spec_id() {
        let result = execute(&["".to_string()], false, false).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_dry_run() {
        let result = execute(&["SPEC-001".to_string()], false, true).await;
        // Dry run should succeed without actually running anything
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_resume_without_state() {
        // Clean up state file if exists
        let state_path = std::path::PathBuf::from(".aad/orchestration/state.json");
        let _ = std::fs::remove_file(&state_path);

        let result = execute(&[], true, false).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("状態ファイルが見つかりません"));
    }
}
