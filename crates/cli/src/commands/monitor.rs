use anyhow::Result;
use clap::Args;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{stdout, Stdout},
    time::Duration,
};
use tui::{events::EventHandler, App};

#[derive(Debug, Args)]
pub struct MonitorArgs {
    /// 更新間隔（秒）
    #[arg(short, long, default_value = "1")]
    interval: u64,
}

pub fn execute(args: MonitorArgs) -> Result<()> {
    println!("TUIダッシュボードを起動します...");

    // ターミナルセットアップ
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // アプリ初期化
    let mut app = App::new();

    // メインループ
    let result = run_app(&mut terminal, &mut app, args.interval);

    // ターミナルクリーンアップ
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    interval_secs: u64,
) -> Result<()> {
    let timeout = Duration::from_millis(100);
    let update_interval = Duration::from_secs(interval_secs);
    let mut last_update = std::time::Instant::now();

    loop {
        // 画面描画
        terminal.draw(|f| app.render(f))?;

        // イベント処理
        if let Some(event) = EventHandler::read_event(timeout)? {
            app.handle_event(event)?;
        }

        // 終了チェック
        if app.should_quit() {
            break;
        }

        // 定期更新
        if last_update.elapsed() >= update_interval {
            app.update();
            last_update = std::time::Instant::now();
        }
    }

    Ok(())
}
