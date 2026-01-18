//! イベント処理

/// イベントハンドラ（スタブ）
pub struct EventHandler;

impl EventHandler {
    /// 新しいEventHandlerインスタンスを作成
    pub fn new() -> Self {
        Self
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_event_handler() {
        let _handler = EventHandler::new();
    }
}
