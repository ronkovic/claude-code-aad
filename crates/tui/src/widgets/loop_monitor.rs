//! Loop monitor widget.
//!
//! Displays the current state of the task execution loop.

use domain::entities::LoopState;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Widget},
};

/// Loop monitor widget.
///
/// Displays:
/// - Current task (highlighted)
/// - Task queue with progress
/// - Color-coded status (green=completed, red=failed, yellow=skipped)
pub struct LoopMonitor<'a> {
    /// Loop state to display
    state: &'a LoopState,
    /// Completed task count
    completed_count: usize,
    /// Failed task count
    failed_count: usize,
    /// Skipped task count
    skipped_count: usize,
    /// Total task count
    total_count: usize,
}

impl<'a> LoopMonitor<'a> {
    /// Creates a new loop monitor widget.
    ///
    /// # Arguments
    ///
    /// * `state` - The loop state to display
    /// * `completed_count` - Number of completed tasks
    /// * `failed_count` - Number of failed tasks
    /// * `skipped_count` - Number of skipped tasks
    /// * `total_count` - Total number of tasks
    pub fn new(
        state: &'a LoopState,
        completed_count: usize,
        failed_count: usize,
        skipped_count: usize,
        total_count: usize,
    ) -> Self {
        Self {
            state,
            completed_count,
            failed_count,
            skipped_count,
            total_count,
        }
    }

    /// Calculates the overall progress (0.0 - 1.0).
    pub fn progress(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }
        (self.completed_count + self.failed_count + self.skipped_count) as f64
            / self.total_count as f64
    }
}

impl<'a> Widget for LoopMonitor<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split into sections:
        // 1. Spec ID and status
        // 2. Progress bar
        // 3. Current task (highlighted)
        // 4. Task queue
        // 5. Statistics
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Progress bar
                Constraint::Length(3), // Current task
                Constraint::Min(5),    // Task queue
                Constraint::Length(3), // Statistics
            ])
            .split(area);

        // Header: Spec ID and status
        let status_text = if self.state.is_active {
            "🔄 Running"
        } else {
            "⏸️  Paused"
        };
        let header = Paragraph::new(format!(
            "Loop Monitor: {} ({})",
            self.state.spec_id, status_text
        ))
        .block(Block::default().borders(Borders::ALL));
        header.render(chunks[0], buf);

        // Progress bar
        let progress_percent = (self.progress() * 100.0) as u16;
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title("Overall Progress")
                    .borders(Borders::ALL),
            )
            .gauge_style(Style::default().fg(Color::Green))
            .percent(progress_percent);
        gauge.render(chunks[1], buf);

        // Current task
        let current_task_text = if let Some(ref task_id) = self.state.current_task {
            format!("⏩ Current: {}", task_id)
        } else {
            "No current task".to_string()
        };
        let current_task_style = if self.state.current_task.is_some() {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        let current_task = Paragraph::new(current_task_text)
            .block(Block::default().title("Current Task").borders(Borders::ALL))
            .style(current_task_style);
        current_task.render(chunks[2], buf);

        // Task queue
        let queue_items: Vec<ListItem> = self
            .state
            .task_queue
            .iter()
            .take(10) // Show only first 10 tasks
            .map(|task_id| ListItem::new(format!("  • {}", task_id)))
            .collect();

        let queue_title = format!("Task Queue ({} pending)", self.state.task_queue.len());
        let task_queue =
            List::new(queue_items).block(Block::default().title(queue_title).borders(Borders::ALL));
        task_queue.render(chunks[3], buf);

        // Statistics
        let stats_text = format!(
            "✅ Completed: {}  ❌ Failed: {}  ⏭️  Skipped: {}  📋 Total: {}",
            self.completed_count, self.failed_count, self.skipped_count, self.total_count
        );
        let stats = Paragraph::new(stats_text)
            .block(Block::default().title("Statistics").borders(Borders::ALL))
            .style(Style::default());
        stats.render(chunks[4], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_objects::{SpecId, TaskId};

    #[test]
    fn test_loop_monitor_creation() {
        let spec_id = SpecId::new();
        let state = LoopState::new(spec_id);

        let widget = LoopMonitor::new(&state, 0, 0, 0, 0);
        assert_eq!(widget.completed_count, 0);
        assert_eq!(widget.failed_count, 0);
        assert_eq!(widget.skipped_count, 0);
        assert_eq!(widget.total_count, 0);
    }

    #[test]
    fn test_loop_monitor_progress_zero_total() {
        let spec_id = SpecId::new();
        let state = LoopState::new(spec_id);

        let widget = LoopMonitor::new(&state, 0, 0, 0, 0);
        assert_eq!(widget.progress(), 0.0);
    }

    #[test]
    fn test_loop_monitor_progress_calculation() {
        let spec_id = SpecId::new();
        let state = LoopState::new(spec_id);

        // 50% complete
        let widget = LoopMonitor::new(&state, 5, 0, 0, 10);
        assert_eq!(widget.progress(), 0.5);

        // 75% complete (5 completed + 2 failed + 1 skipped out of 10)
        let widget = LoopMonitor::new(&state, 5, 2, 1, 10);
        assert_eq!(widget.progress(), 0.8);

        // 100% complete
        let widget = LoopMonitor::new(&state, 10, 0, 0, 10);
        assert_eq!(widget.progress(), 1.0);
    }

    #[test]
    fn test_loop_monitor_with_current_task() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);
        let task_id = TaskId::new();
        state.set_current_task(Some(task_id));

        let widget = LoopMonitor::new(&state, 0, 0, 0, 1);
        assert!(widget.state.current_task.is_some());
    }

    #[test]
    fn test_loop_monitor_with_task_queue() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);

        let task_id1 = TaskId::new();
        let task_id2 = TaskId::new();
        state.enqueue_task(task_id1);
        state.enqueue_task(task_id2);

        let widget = LoopMonitor::new(&state, 0, 0, 0, 2);
        assert_eq!(widget.state.task_queue.len(), 2);
    }

    #[test]
    fn test_loop_monitor_active_state() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);
        state.start();

        let widget = LoopMonitor::new(&state, 0, 0, 0, 0);
        assert!(widget.state.is_active);
    }

    #[test]
    fn test_loop_monitor_paused_state() {
        let spec_id = SpecId::new();
        let mut state = LoopState::new(spec_id);
        state.start();
        state.pause();

        let widget = LoopMonitor::new(&state, 0, 0, 0, 0);
        assert!(!widget.state.is_active);
    }
}
