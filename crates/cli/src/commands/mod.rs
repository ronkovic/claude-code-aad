pub mod init;
pub mod loop_cmd;
pub mod monitor;
pub mod orchestrate;
pub mod persist;
pub mod spec;
pub mod style;
pub mod tasks;
pub mod worktree;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// プロジェクトを初期化
    Init,

    /// 仕様ファイルを作成
    Spec {
        /// 仕様ID（例: SPEC-001）
        spec_id: String,
    },

    /// タスクを分割
    Tasks {
        /// 仕様ID
        spec_id: String,

        /// GitHub Issues を作成
        #[arg(long)]
        github: bool,
    },

    /// スタイル操作
    Style {
        #[command(subcommand)]
        action: StyleAction,
    },

    /// Git worktree を作成
    Worktree {
        /// 仕様ID
        spec_id: String,
    },

    /// 複数の仕様を並列実行
    Orchestrate {
        /// 実行する仕様ID（複数指定可能）
        #[arg(long, value_delimiter = ' ', num_args = 1..)]
        specs: Vec<String>,

        /// 中断したオーケストレーションを再開
        #[arg(long)]
        resume: bool,

        /// 実行計画を表示（実際には実行しない）
        #[arg(long)]
        dry_run: bool,
    },

    /// セッション状態の永続化操作
    Persist {
        #[command(subcommand)]
        action: PersistAction,
    },

    /// TUIダッシュボードを起動
    Monitor(monitor::MonitorArgs),

    /// タスクをループで実行
    Loop {
        /// 仕様ID（例: SPEC-001）
        spec_id: String,

        /// 中断したループを再開
        #[arg(long)]
        resume: bool,
    },
}

#[derive(Subcommand)]
pub enum PersistAction {
    /// 全セッション状態を保存
    Save,

    /// 指定時刻の状態に復元
    Restore {
        /// タイムスタンプ（ISO 8601形式、例: 2026-01-18T10:30:00）
        timestamp: String,
    },

    /// バックアップ一覧を表示
    List,
}

#[derive(Subcommand)]
pub enum StyleAction {
    /// スタイル一覧を表示
    List,

    /// スタイルを適用
    Apply {
        /// スタイル名
        style_name: String,
    },
}
