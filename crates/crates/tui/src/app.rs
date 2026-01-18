//! TUIアプリケーション状態管理

/// TUIアプリケーションの状態を管理する構造体
pub struct App {
    /// アプリケーションが終了すべきかどうか
    should_quit: bool,
}

impl App {
    /// 新しいAppインスタンスを作成
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    /// 終了すべきかどうかを返す
    pub fn should_quit(&self) -> bool {
        self.should_quit
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
    fn test_new_app() {
        let app = App::new();
        assert!(!app.should_quit());
    }

    #[test]
    fn test_default_app() {
        let app = App::default();
        assert!(!app.should_quit());
    }
}
