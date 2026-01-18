use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// エスカレーションログエンティティ（永続化用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationLog {
    pub session_id: String,
    pub level: String,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
    pub spec_id: Option<String>,
    pub phase: Option<String>,
}

impl EscalationLog {
    /// 新しいエスカレーションログを作成
    pub fn new(
        session_id: String,
        level: String,
        reason: String,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            session_id,
            level,
            reason,
            timestamp,
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_escalation_log_creation() {
        let now = Utc::now();
        let log = EscalationLog::new(
            "session-001".to_string(),
            "Error".to_string(),
            "Test failed".to_string(),
            now,
        );

        assert_eq!(log.session_id, "session-001");
        assert_eq!(log.level, "Error");
        assert_eq!(log.reason, "Test failed");
        assert_eq!(log.timestamp, now);
        assert!(log.spec_id.is_none());
        assert!(log.phase.is_none());
    }

    #[test]
    fn test_escalation_log_with_context() {
        let now = Utc::now();
        let log = EscalationLog::new(
            "session-002".to_string(),
            "Critical".to_string(),
            "Security issue".to_string(),
            now,
        )
        .with_context("SPEC-004".to_string(), "TDD".to_string());

        assert_eq!(log.spec_id, Some("SPEC-004".to_string()));
        assert_eq!(log.phase, Some("TDD".to_string()));
    }
}
