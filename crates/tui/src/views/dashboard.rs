use crate::widgets::{ContextBar, PhaseIndicator, SessionList};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

/// メインダッシュボード画面
pub struct DashboardView<'a> {
    context_usage: f64,
    current_phase: &'a str,
    sessions: Vec<&'a str>,
    selected: usize,
}

impl<'a> DashboardView<'a> {
    pub fn new(
        context_usage: f64,
        current_phase: &'a str,
        sessions: Vec<&'a str>,
        selected: usize,
    ) -> Self {
        Self {
            context_usage,
            current_phase,
            sessions,
            selected,
        }
    }
}

impl<'a> Widget for DashboardView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // ContextBar
                Constraint::Length(3), // PhaseIndicator
                Constraint::Min(0),    // SessionList
            ])
            .split(area);

        ContextBar::new(self.context_usage).render(chunks[0], buf);
        PhaseIndicator::new(self.current_phase).render(chunks[1], buf);
        SessionList::new(self.sessions, self.selected).render(chunks[2], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_view_creation() {
        let view = DashboardView::new(0.5, "TDD", vec!["Session 1", "Session 2"], 0);
        assert_eq!(view.context_usage, 0.5);
        assert_eq!(view.current_phase, "TDD");
        assert_eq!(view.sessions.len(), 2);
        assert_eq!(view.selected, 0);
    }
}
