//! TUIアプリケーション状態管理

use anyhow::Result;
use application::loop_engine::LoopEngine;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use domain::entities::LoopState;
use ratatui::Frame;
use std::path::PathBuf;

use crate::views::View;

/// TUIアプリケーション状態
pub struct App {
    /// 現在の画面
    current_view: View,
    /// 前の画面（Escで戻る用）
    previous_view: Option<View>,
    /// 選択中のインデックス
    selected_index: usize,
    /// 終了フラグ
    should_quit: bool,
    /// ループ状態（オプション）
    loop_state: Option<LoopState>,
}

impl App {
    /// 新しいAppインスタンスを作成
    pub fn new() -> Self {
        Self {
            current_view: View::Dashboard,
            previous_view: None,
            selected_index: 0,
            should_quit: false,
            loop_state: None,
        }
    }

    /// 状態を更新
    pub fn update(&mut self) {
        // ループ状態をリロード
        self.reload_loop_state();
    }

    /// ループ状態をファイルからリロード
    fn reload_loop_state(&mut self) {
        let state_file = PathBuf::from(".aad/loop-state.json");
        if state_file.exists() {
            if let Ok(engine) = LoopEngine::load(state_file) {
                self.loop_state = Some(engine.state().clone());
            }
        }
    }

    /// ループ状態を取得
    pub fn loop_state(&self) -> Option<&LoopState> {
        self.loop_state.as_ref()
    }

    /// 画面を描画
    pub fn render(&mut self, frame: &mut Frame) {
        // ダミーデータで画面を描画
        use crate::views::{DashboardView, DetailView, MonitorView, WorkflowView};
        use ratatui::widgets::Widget;

        let area = frame.area();

        match self.current_view {
            View::Dashboard => {
                let view = DashboardView::new(
                    0.45,  // context_usage
                    "TDD", // current_phase
                    vec!["Session 1", "Session 2", "Session 3"],
                    self.selected_index,
                );
                view.render(area, frame.buffer_mut());
            }
            View::Monitor => {
                // ループ状態があればLoopMonitorを使用、なければ従来のMonitorViewを使用
                if let Some(loop_state) = &self.loop_state {
                    use crate::widgets::LoopMonitor;
                    // TODO: 実際のタスクステータスから統計を取得
                    // 今はダミーデータを使用
                    let monitor = LoopMonitor::new(loop_state, 5, 1, 0, 10);
                    monitor.render(area, frame.buffer_mut());
                } else {
                    let view = MonitorView::new(
                        "SPEC-006",
                        vec![
                            ("T01: TUIクレート作成", 1.0),
                            ("T02: App構造体実装", 1.0),
                            ("T03: Widgets実装", 1.0),
                            ("T04: Views実装", 1.0),
                            ("T05: イベント処理", 1.0),
                            ("T06: monitor連携", 0.8),
                            ("T07: 品質チェック", 0.0),
                        ],
                    );
                    view.render(area, frame.buffer_mut());
                }
            }
            View::Workflow => {
                let view = WorkflowView::new(
                    "ワークフロー:\n1. SPEC\n2. TASKS\n3. TDD\n4. REVIEW\n5. RETRO",
                );
                view.render(area, frame.buffer_mut());
            }
            View::Detail => {
                let view = DetailView::new("詳細情報:\n- セッション詳細\n- タスク詳細\n- 進捗詳細");
                view.render(area, frame.buffer_mut());
            }
        }
    }

    /// イベントを処理
    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            self.handle_key_event(key)?;
        }
        Ok(())
    }

    /// キーイベントを処理
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            // 終了
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            // Ctrl+C でも終了
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            // Tab: View切り替え
            KeyCode::Tab => {
                self.current_view = self.current_view.next();
            }
            // ↑: リスト選択（上）
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            // ↓: リスト選択（下）
            KeyCode::Down => {
                self.selected_index += 1;
                // 上限チェックは実際のデータ量に応じて調整
            }
            // Enter: 詳細画面遷移
            KeyCode::Enter => {
                self.on_enter_pressed()?;
            }
            // Esc: 戻る
            KeyCode::Esc => {
                self.on_esc_pressed()?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Enterキー押下時の処理
    fn on_enter_pressed(&mut self) -> Result<()> {
        match self.current_view {
            View::Dashboard | View::Monitor => {
                // 詳細画面に遷移
                self.previous_view = Some(self.current_view);
                self.current_view = View::Detail;
            }
            _ => {}
        }
        Ok(())
    }

    /// Escキー押下時の処理
    fn on_esc_pressed(&mut self) -> Result<()> {
        if let Some(prev) = self.previous_view {
            self.current_view = prev;
            self.previous_view = None;
        }
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
mod loop_state_tests {
    use super::*;
    

    #[test]
    fn test_loop_state_getter() {
        let app = App::new();
        assert!(app.loop_state().is_none());
    }

    #[test]
    fn test_update_reloads_loop_state() {
        let mut app = App::new();
        app.update();
        // ファイルが存在しない場合はNoneのまま
        assert!(app.loop_state().is_none());
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

    #[test]
    fn test_quit_on_q_key() {
        let mut app = App::new();
        let event = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        app.handle_event(event).unwrap();
        assert!(app.should_quit);
    }

    #[test]
    fn test_quit_on_capital_q_key() {
        let mut app = App::new();
        let event = Event::Key(KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE));
        app.handle_event(event).unwrap();
        assert!(app.should_quit);
    }

    #[test]
    fn test_quit_on_ctrl_c() {
        let mut app = App::new();
        let event = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        app.handle_event(event).unwrap();
        assert!(app.should_quit);
    }

    #[test]
    fn test_view_switch_on_tab() {
        let mut app = App::new();
        assert_eq!(app.current_view, View::Dashboard);

        let event = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        app.handle_event(event).unwrap();
        assert_eq!(app.current_view, View::Monitor);
    }

    #[test]
    fn test_list_navigation() {
        let mut app = App::new();
        assert_eq!(app.selected_index, 0);

        let down = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        app.handle_event(down).unwrap();
        assert_eq!(app.selected_index, 1);

        let up = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        app.handle_event(up).unwrap();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_enter_to_detail_view() {
        let mut app = App::new();
        assert_eq!(app.current_view, View::Dashboard);

        let enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_event(enter).unwrap();
        assert_eq!(app.current_view, View::Detail);
        assert_eq!(app.previous_view, Some(View::Dashboard));
    }

    #[test]
    fn test_esc_to_previous_view() {
        let mut app = App::new();
        app.current_view = View::Detail;
        app.previous_view = Some(View::Dashboard);

        let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        app.handle_event(esc).unwrap();
        assert_eq!(app.current_view, View::Dashboard);
        assert_eq!(app.previous_view, None);
    }
}
