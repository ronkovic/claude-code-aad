//! TUI Views

pub mod dashboard;
pub mod detail;
pub mod monitor;
pub mod workflow;

pub use dashboard::DashboardView;
pub use detail::DetailView;
pub use monitor::MonitorView;
pub use workflow::WorkflowView;

/// TUI画面種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    Monitor,
    Workflow,
    Detail,
}

impl View {
    pub fn next(&self) -> Self {
        match self {
            View::Dashboard => View::Monitor,
            View::Monitor => View::Workflow,
            View::Workflow => View::Detail,
            View::Detail => View::Dashboard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_creation() {
        let _view = View::Dashboard;
    }

    #[test]
    fn test_view_navigation() {
        assert_eq!(View::Dashboard.next(), View::Monitor);
        assert_eq!(View::Monitor.next(), View::Workflow);
        assert_eq!(View::Workflow.next(), View::Detail);
        assert_eq!(View::Detail.next(), View::Dashboard);
    }
}
