use crate::widgets::TaskProgress;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// 監視画面
pub struct MonitorView<'a> {
    spec_id: &'a str,
    tasks: Vec<(&'a str, f64)>, // (task_name, progress)
}

impl<'a> MonitorView<'a> {
    pub fn new(spec_id: &'a str, tasks: Vec<(&'a str, f64)>) -> Self {
        Self { spec_id, tasks }
    }
}

impl<'a> Widget for MonitorView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Paragraph::new(format!("Monitor: {}", self.spec_id))
            .block(Block::default().borders(Borders::ALL));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                std::iter::once(Constraint::Length(3))
                    .chain(self.tasks.iter().map(|_| Constraint::Length(3)))
                    .collect::<Vec<_>>(),
            )
            .split(area);

        title.render(chunks[0], buf);

        for (i, (task_name, progress)) in self.tasks.iter().enumerate() {
            TaskProgress::new(task_name, *progress).render(chunks[i + 1], buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_view_creation() {
        let tasks = vec![("Task 1", 0.5), ("Task 2", 0.75)];
        let view = MonitorView::new("SPEC-001", tasks);
        assert_eq!(view.spec_id, "SPEC-001");
        assert_eq!(view.tasks.len(), 2);
    }

    #[test]
    fn test_monitor_view_empty_tasks() {
        let view = MonitorView::new("SPEC-002", vec![]);
        assert_eq!(view.tasks.len(), 0);
    }
}
