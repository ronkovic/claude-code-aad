//! TUIアプリケーション状態管理

use anyhow::Result;
use crossterm::event::Event;
use ratatui::Frame;

use crate::views::View;

/// TUIアプリケーション状態
pub struct App {
    /// 現在の画面
    current_view: View,
    /// 選択中のインデックス
    selected_index: usize,
    /// 終了フラグ
    should_quit: bool,
}

impl App {
    /// 新しいAppインスタンスを作成
    pub fn new() -> Self {
        Self {
            current_view: View::Dashboard,
            selected_index: 0,
            should_quit: false,
        }
    }

    /// 状態を更新
    pub fn update(&mut self) {
        // 後のタスクで実装
    }

    /// 画面を描画
    pub fn render(&mut self, _frame: &mut Frame) {
        // 後のタスクで実装
    }

    /// イベントを処理
    pub fn handle_event(&mut self, _event: Event) -> Result<()> {
        // 後のタスクで実装
        Ok(())
    }

    /// 終了すべきかどうかを返す
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// 現在のビューを返す
    pub fn current_view(&self) -> View {
        self.current_view
    }

    /// 選択中のインデックスを返す
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.current_view(), View::Dashboard);
        assert_eq!(app.selected_index(), 0);
        assert!(!app.should_quit());
    }

    #[test]
    fn test_default_app() {
        let app = App::default();
        assert_eq!(app.current_view(), View::Dashboard);
        assert_eq!(app.selected_index(), 0);
        assert!(!app.should_quit());
    }

    #[test]
    fn test_handle_event() {
        let mut app = App::new();
        let event = Event::Resize(80, 24);
        assert!(app.handle_event(event).is_ok());
    }
}
