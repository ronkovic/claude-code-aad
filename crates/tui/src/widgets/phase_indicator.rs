use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// フェーズインジケーターを表示するWidget
pub struct PhaseIndicator<'a> {
    current_phase: &'a str,
}

impl<'a> PhaseIndicator<'a> {
    pub fn new(current_phase: &'a str) -> Self {
        Self { current_phase }
    }

    fn create_phase_line(&self) -> Line<'a> {
        let phases = ["SPEC", "TASKS", "TDD", "REVIEW", "RETRO"];
        let spans: Vec<Span> = phases
            .iter()
            .map(|phase| {
                if *phase == self.current_phase {
                    Span::styled(
                        format!("[{}]", phase),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(format!(" {} ", phase))
                }
            })
            .collect();

        Line::from(spans)
    }
}

impl<'a> Widget for PhaseIndicator<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.create_phase_line();
        let paragraph =
            Paragraph::new(line).block(Block::default().title("Phase").borders(Borders::ALL));

        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_indicator_creation() {
        let widget = PhaseIndicator::new("SPEC");
        assert_eq!(widget.current_phase, "SPEC");
    }

    #[test]
    fn test_phase_indicator_tdd() {
        let widget = PhaseIndicator::new("TDD");
        assert_eq!(widget.current_phase, "TDD");
    }

    #[test]
    fn test_phase_indicator_line_creation() {
        let widget = PhaseIndicator::new("REVIEW");
        let line = widget.create_phase_line();
        assert!(line.spans.len() > 0);
    }
}
