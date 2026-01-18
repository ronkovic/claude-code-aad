//! Orchestrate command implementation.
//!
//! Executes multiple specifications concurrently using the orchestration engine.

use application::orchestration::{Orchestrator, OrchestratorConfig, SessionStatus};
use domain::value_objects::{ids::SpecId, phase::Phase};
use std::str::FromStr;
use std::time::Duration;

/// Executes the orchestrate command to run multiple specs concurrently.
///
/// # Arguments
///
/// * `spec_ids` - List of specification IDs to execute (e.g., ["SPEC-001", "SPEC-002"])
///
/// # Examples
///
/// ```bash
/// aad orchestrate --specs SPEC-001 SPEC-002
/// ```
pub async fn execute(spec_ids: &[String]) -> anyhow::Result<()> {
    if spec_ids.is_empty() {
        anyhow::bail!("エラー: 少なくとも1つのSpec IDを指定してください");
    }

    println!("🚀 オーケストレーション開始\n");
    println!("📋 実行対象:");
    for spec_id in spec_ids {
        println!("  - {}", spec_id);
    }
    println!();

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

        for session in &all_sessions {
            if let Some(status) = orchestrator.get_session_status(&session.id).await {
                match status {
                    SessionStatus::Completed => completed += 1,
                    SessionStatus::Failed => failed += 1,
                    SessionStatus::TimedOut => timed_out += 1,
                    SessionStatus::Running => {
                        running += 1;
                        all_done = false;
                    }
                    SessionStatus::Pending => {
                        pending += 1;
                        all_done = false;
                    }
                }
            }
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

            println!(
                "│ {} {} - {:8} │",
                status_icon,
                &session.id,
                status_text
            );
        }
    }

    println!("└─────────────────────────────────────┘\n");

    // Check if any sessions failed
    let failed_count = all_sessions
        .iter()
        .filter(|s| {
            if let Some(status) = futures::executor::block_on(orchestrator.get_session_status(&s.id)) {
                matches!(status, SessionStatus::Failed | SessionStatus::TimedOut)
            } else {
                false
            }
        })
        .count();

    if failed_count > 0 {
        eprintln!("⚠️  警告: {} セッションが失敗しました", failed_count);
        eprintln!("詳細は .aad/sessions/ および .aad/escalations/ を確認してください");
        return Err(anyhow::anyhow!(
            "{} セッションが失敗しました",
            failed_count
        ));
    }

    println!("✅ すべてのセッションが正常に完了しました");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_empty_specs() {
        let result = execute(&[]).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("少なくとも1つのSpec ID"));
    }

    #[tokio::test]
    async fn test_execute_invalid_spec_id() {
        let result = execute(&["".to_string()]).await;
        assert!(result.is_err());
    }
}
