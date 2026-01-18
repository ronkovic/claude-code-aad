use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// 詳細画面
pub struct DetailView<'a> {
    detail_text: &'a str,
}

impl<'a> DetailView<'a> {
    pub fn new(detail_text: &'a str) -> Self {
        Self { detail_text }
    }
}

impl<'a> Widget for DetailView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.detail_text)
            .block(Block::default().title("Detail").borders(Borders::ALL));

        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detail_view_creation() {
        let view = DetailView::new("Detail information");
        assert_eq!(view.detail_text, "Detail information");
    }
}
