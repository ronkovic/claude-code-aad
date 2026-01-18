use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Widget},
};

/// コンテキスト使用率を表示するWidget（70%ルール対応）
pub struct ContextBar {
    usage: f64, // 0.0 - 1.0
}

impl ContextBar {
    pub fn new(usage: f64) -> Self {
        Self { usage }
    }

    fn get_color(&self) -> Color {
        if self.usage < 0.5 {
            Color::Green // 快適
        } else if self.usage < 0.7 {
            Color::Yellow // 注意
        } else if self.usage < 0.85 {
            Color::LightRed // 警告
        } else if self.usage < 0.95 {
            Color::Red // 危機的
        } else {
            Color::Magenta // 限界
        }
    }
}

impl Widget for ContextBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color = self.get_color();
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title("Context Usage")
                    .borders(Borders::ALL),
            )
            .gauge_style(Style::default().fg(color))
            .percent((self.usage * 100.0) as u16);

        gauge.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_bar_creation() {
        let widget = ContextBar::new(0.5);
        assert_eq!(widget.usage, 0.5);
    }

    #[test]
    fn test_context_bar_color_green() {
        let widget = ContextBar::new(0.3);
        assert_eq!(widget.get_color(), Color::Green);
    }

    #[test]
    fn test_context_bar_color_yellow() {
        let widget = ContextBar::new(0.6);
        assert_eq!(widget.get_color(), Color::Yellow);
    }

    #[test]
    fn test_context_bar_color_light_red() {
        let widget = ContextBar::new(0.75);
        assert_eq!(widget.get_color(), Color::LightRed);
    }

    #[test]
    fn test_context_bar_color_red() {
        let widget = ContextBar::new(0.9);
        assert_eq!(widget.get_color(), Color::Red);
    }

    #[test]
    fn test_context_bar_color_magenta() {
        let widget = ContextBar::new(0.97);
        assert_eq!(widget.get_color(), Color::Magenta);
    }

    #[test]
    fn test_context_bar_70_percent_rule() {
        // 70%ルールの境界確認
        let widget_below = ContextBar::new(0.69);
        assert_eq!(widget_below.get_color(), Color::Yellow);

        let widget_at = ContextBar::new(0.7);
        assert_eq!(widget_at.get_color(), Color::LightRed);
    }
}
