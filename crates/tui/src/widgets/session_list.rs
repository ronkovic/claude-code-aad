use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Widget},
};

/// セッション一覧を表示するWidget
pub struct SessionList<'a> {
    sessions: Vec<&'a str>,
    selected: usize,
}

impl<'a> SessionList<'a> {
    pub fn new(sessions: Vec<&'a str>, selected: usize) -> Self {
        Self { sessions, selected }
    }
}

impl<'a> Widget for SessionList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .sessions
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let style = if i == self.selected {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(*s).style(style)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .title("Active Sessions")
                .borders(Borders::ALL),
        );

        list.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_list_creation() {
        let sessions = vec!["Session 1", "Session 2"];
        let widget = SessionList::new(sessions, 0);
        assert_eq!(widget.sessions.len(), 2);
        assert_eq!(widget.selected, 0);
    }

    #[test]
    fn test_session_list_with_selection() {
        let sessions = vec!["Session 1", "Session 2", "Session 3"];
        let widget = SessionList::new(sessions, 1);
        assert_eq!(widget.selected, 1);
    }
}
