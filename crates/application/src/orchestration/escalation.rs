use super::orchestrator::SessionId;
use chrono::{DateTime, Utc};
use std::fs;
use std::io;
use std::path::PathBuf;

/// エスカレーションレベル
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscalationLevel {
    /// 警告 - 作業継続可能
    Warning,
    /// エラー - 対応が必要
    Error,
    /// クリティカル - 即時停止
    Critical,
}

impl EscalationLevel {
    /// レベルを文字列に変換
    pub fn as_str(&self) -> &str {
        match self {
            EscalationLevel::Warning => "Warning",
            EscalationLevel::Error => "Error",
            EscalationLevel::Critical => "Critical",
        }
    }

    /// ログプレフィックスを取得
    pub fn log_prefix(&self) -> &str {
        match self {
            EscalationLevel::Warning => "[ESCALATION:WARNING]",
            EscalationLevel::Error => "[ESCALATION:ERROR]",
            EscalationLevel::Critical => "[ESCALATION:CRITICAL]",
        }
    }

    /// 絵文字を取得
    pub fn emoji(&self) -> &str {
        match self {
            EscalationLevel::Warning => "🟡",
            EscalationLevel::Error => "🔴",
            EscalationLevel::Critical => "⛔",
        }
    }
}

/// エスカレーション情報
#[derive(Debug, Clone)]
pub struct Escalation {
    pub session_id: SessionId,
    pub level: EscalationLevel,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
    pub spec_id: Option<String>,
    pub phase: Option<String>,
}

impl Escalation {
    /// 新しいエスカレーションを作成
    pub fn new(session_id: SessionId, level: EscalationLevel, reason: String) -> Self {
        Self {
            session_id,
            level,
            reason,
            timestamp: Utc::now(),
            spec_id: None,
            phase: None,
        }
    }

    /// コンテキスト情報を追加
    pub fn with_context(mut self, spec_id: String, phase: String) -> Self {
        self.spec_id = Some(spec_id);
        self.phase = Some(phase);
        self
    }
}

/// エスカレーションハンドラー
pub struct EscalationHandler {
    escalations_dir: PathBuf,
}

impl EscalationHandler {
    /// 新しいエスカレーションハンドラーを作成
    pub fn new(escalations_dir: PathBuf) -> Self {
        Self { escalations_dir }
    }

    /// エスカレーションを処理
    pub fn handle(&self, escalation: &Escalation) -> io::Result<PathBuf> {
        // ディレクトリが存在しない場合は作成
        fs::create_dir_all(&self.escalations_dir)?;

        // コンソールログ出力
        log::escalation(escalation.level, &escalation.session_id, &escalation.reason);

        // ファイルログ出力
        self.write_log_file(escalation)
    }

    /// ログファイルに書き込み
    fn write_log_file(&self, escalation: &Escalation) -> io::Result<PathBuf> {
        // ファイル名: YYYY-MM-DD_HH-MM-SS_{session_id}.md
        let timestamp_str = escalation.timestamp.format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("{}_{}.md", timestamp_str, escalation.session_id);
        let file_path = self.escalations_dir.join(&filename);

        // Markdown形式でログを作成
        let mut content = String::new();
        content.push_str("# Escalation Log\n\n");
        content.push_str(&format!("- **Session ID**: {}\n", escalation.session_id));
        content.push_str(&format!(
            "- **Level**: {} {}\n",
            escalation.level.emoji(),
            escalation.level.as_str()
        ));
        content.push_str(&format!(
            "- **Timestamp**: {}\n",
            escalation.timestamp.to_rfc3339()
        ));
        content.push_str(&format!("- **Reason**: {}\n", escalation.reason));

        if escalation.spec_id.is_some() || escalation.phase.is_some() {
            content.push_str("\n## Context\n\n");
            if let Some(spec_id) = &escalation.spec_id {
                content.push_str(&format!("- **Spec**: {}\n", spec_id));
            }
            if let Some(phase) = &escalation.phase {
                content.push_str(&format!("- **Phase**: {}\n", phase));
            }
        }

        fs::write(&file_path, content)?;
        Ok(file_path)
    }
}

/// ログユーティリティ
pub mod log {
    use super::EscalationLevel;

    /// エスカレーションログを出力
    pub fn escalation(level: EscalationLevel, session_id: &str, reason: &str) {
        eprintln!(
            "{} {} Session: {} - {}",
            level.log_prefix(),
            level.emoji(),
            session_id,
            reason
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_escalation_level_as_str() {
        assert_eq!(EscalationLevel::Warning.as_str(), "Warning");
        assert_eq!(EscalationLevel::Error.as_str(), "Error");
        assert_eq!(EscalationLevel::Critical.as_str(), "Critical");
    }

    #[test]
    fn test_escalation_level_log_prefix() {
        assert_eq!(
            EscalationLevel::Warning.log_prefix(),
            "[ESCALATION:WARNING]"
        );
        assert_eq!(EscalationLevel::Error.log_prefix(), "[ESCALATION:ERROR]");
        assert_eq!(
            EscalationLevel::Critical.log_prefix(),
            "[ESCALATION:CRITICAL]"
        );
    }

    #[test]
    fn test_escalation_creation() {
        let escalation = Escalation::new(
            "session-001".to_string(),
            EscalationLevel::Error,
            "Test failed".to_string(),
        );

        assert_eq!(escalation.session_id, "session-001");
        assert_eq!(escalation.level, EscalationLevel::Error);
        assert_eq!(escalation.reason, "Test failed");
        assert!(escalation.spec_id.is_none());
        assert!(escalation.phase.is_none());
    }

    #[test]
    fn test_escalation_with_context() {
        let escalation = Escalation::new(
            "session-002".to_string(),
            EscalationLevel::Critical,
            "Security issue".to_string(),
        )
        .with_context("SPEC-004".to_string(), "TDD".to_string());

        assert_eq!(escalation.spec_id, Some("SPEC-004".to_string()));
        assert_eq!(escalation.phase, Some("TDD".to_string()));
    }

    #[test]
    fn test_escalation_handler_handle() {
        let temp_dir = TempDir::new().unwrap();
        let handler = EscalationHandler::new(temp_dir.path().to_path_buf());

        let escalation = Escalation::new(
            "session-test".to_string(),
            EscalationLevel::Warning,
            "Test escalation".to_string(),
        )
        .with_context("SPEC-999".to_string(), "TEST".to_string());

        let file_path = handler.handle(&escalation).unwrap();
        assert!(file_path.exists());

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("# Escalation Log"));
        assert!(content.contains("session-test"));
        assert!(content.contains("Warning"));
        assert!(content.contains("Test escalation"));
        assert!(content.contains("SPEC-999"));
        assert!(content.contains("TEST"));
    }
}
