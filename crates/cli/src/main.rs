mod app;
mod commands;

use clap::Parser;
use commands::{Commands, StyleAction};

#[derive(Parser)]
#[command(name = "aad")]
#[command(version, about = "AI駆動開発ツール", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::execute()?,
        Commands::Spec { spec_id } => commands::spec::execute(&spec_id)?,
        Commands::Tasks { spec_id, github } => commands::tasks::execute(&spec_id, github)?,
        Commands::Style { action } => match action {
            StyleAction::List => commands::style::list()?,
            StyleAction::Apply { style_name } => commands::style::apply(&style_name)?,
        },
        Commands::Worktree { spec_id } => commands::worktree::execute(&spec_id)?,
    }

    Ok(())
}
