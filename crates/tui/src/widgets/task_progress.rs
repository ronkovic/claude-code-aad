use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Widget},
};

/// タスク進捗バーを表示するWidget
pub struct TaskProgress<'a> {
    label: &'a str,
    progress: f64, // 0.0 - 1.0
}

impl<'a> TaskProgress<'a> {
    pub fn new(label: &'a str, progress: f64) -> Self {
        Self { label, progress }
    }
}

impl<'a> Widget for TaskProgress<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let gauge = Gauge::default()
            .block(Block::default().title(self.label).borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .percent((self.progress * 100.0) as u16);

        gauge.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_progress_creation() {
        let widget = TaskProgress::new("Test Task", 0.5);
        assert_eq!(widget.label, "Test Task");
        assert_eq!(widget.progress, 0.5);
    }

    #[test]
    fn test_task_progress_full() {
        let widget = TaskProgress::new("Completed", 1.0);
        assert_eq!(widget.progress, 1.0);
    }

    #[test]
    fn test_task_progress_zero() {
        let widget = TaskProgress::new("Not Started", 0.0);
        assert_eq!(widget.progress, 0.0);
    }
}
