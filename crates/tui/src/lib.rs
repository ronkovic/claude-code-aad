//! TUI (Terminal User Interface) クレート
//!
//! Ratatui によるリアルタイムダッシュボードを提供します。

pub mod app;
pub mod events;
pub mod views;
pub mod widgets;

pub use app::App;
