//! Configuration for the orchestrator.

use serde::{Deserialize, Serialize};

/// Configuration for the orchestrator.
///
/// Controls various aspects of the orchestration process including
/// parallelism, timeouts, and monitoring intervals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    /// Maximum number of sessions that can run in parallel.
    ///
    /// Defaults to the number of CPU cores.
    pub max_parallel_sessions: usize,

    /// Session timeout in seconds.
    ///
    /// Sessions exceeding this duration will be terminated.
    /// Default: 3600 seconds (1 hour).
    pub session_timeout_secs: u64,

    /// Monitoring loop interval in seconds.
    ///
    /// How often the orchestrator checks session status.
    /// Default: 1 second.
    pub monitor_interval_secs: u64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            max_parallel_sessions: num_cpus(),
            session_timeout_secs: 3600,
            monitor_interval_secs: 1,
        }
    }
}

/// Returns the number of logical CPU cores.
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = OrchestratorConfig::default();

        assert!(config.max_parallel_sessions > 0);
        assert_eq!(config.session_timeout_secs, 3600);
        assert_eq!(config.monitor_interval_secs, 1);
    }

    #[test]
    fn test_num_cpus() {
        let cpus = num_cpus();
        assert!(cpus > 0);
        assert!(cpus <= 256); // Reasonable upper bound
    }

    #[test]
    fn test_config_clone() {
        let config = OrchestratorConfig::default();
        let cloned = config.clone();

        assert_eq!(config.max_parallel_sessions, cloned.max_parallel_sessions);
        assert_eq!(config.session_timeout_secs, cloned.session_timeout_secs);
        assert_eq!(config.monitor_interval_secs, cloned.monitor_interval_secs);
    }

    #[test]
    fn test_config_serialization() {
        let config = OrchestratorConfig {
            max_parallel_sessions: 8,
            session_timeout_secs: 7200,
            monitor_interval_secs: 2,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: OrchestratorConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(
            config.max_parallel_sessions,
            deserialized.max_parallel_sessions
        );
        assert_eq!(
            config.session_timeout_secs,
            deserialized.session_timeout_secs
        );
        assert_eq!(
            config.monitor_interval_secs,
            deserialized.monitor_interval_secs
        );
    }
}
