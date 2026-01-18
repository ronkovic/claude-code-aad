use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

/// イベントハンドラ
pub struct EventHandler;

impl EventHandler {
    /// イベントを読み取る（タイムアウト付き）
    pub fn read_event(timeout: Duration) -> Result<Option<Event>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    /// キーイベントに変換
    pub fn as_key_event(event: Event) -> Option<KeyEvent> {
        match event {
            Event::Key(key_event) => Some(key_event),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

    #[test]
    fn test_as_key_event_with_key() {
        let key_event = KeyEvent::new(KeyCode::Char('a'), crossterm::event::KeyModifiers::NONE);
        let event = Event::Key(key_event);
        assert!(EventHandler::as_key_event(event).is_some());
    }

    #[test]
    fn test_as_key_event_with_non_key() {
        let event = Event::Resize(80, 24);
        assert!(EventHandler::as_key_event(event).is_none());
    }
}
