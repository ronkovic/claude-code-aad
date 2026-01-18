use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// ワークフロー画面
pub struct WorkflowView<'a> {
    workflow_text: &'a str,
}

impl<'a> WorkflowView<'a> {
    pub fn new(workflow_text: &'a str) -> Self {
        Self { workflow_text }
    }
}

impl<'a> Widget for WorkflowView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.workflow_text)
            .block(Block::default().title("Workflow").borders(Borders::ALL));

        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_view_creation() {
        let view = WorkflowView::new("Test workflow");
        assert_eq!(view.workflow_text, "Test workflow");
    }
}
