pub mod init;
pub mod orchestrate;
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
    },
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
