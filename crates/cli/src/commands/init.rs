//! Init command implementation.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Executes the init command to initialize a new AAD project.
pub fn execute() -> anyhow::Result<()> {
    println!("プロジェクトを初期化しています...\n");

    // 1. Create .aad/ directory structure
    create_directory(".aad")?;
    create_directory(".aad/specs")?;
    create_directory(".aad/tasks")?;
    create_directory(".aad/sessions")?;
    create_directory(".aad/retrospectives")?;
    create_directory(".aad/progress")?;
    println!("✓ .aad/ ディレクトリを作成しました");

    // 2. Create config/ directory and template files
    create_directory("config")?;

    create_template_file("config/aad.toml", include_str!("../../templates/aad.toml"))?;

    create_template_file(
        "config/styles.toml",
        include_str!("../../templates/styles.toml"),
    )?;
    println!("✓ テンプレートファイルを配置しました");

    // 3. Create CLAUDE.md if it doesn't exist
    if !Path::new("CLAUDE.md").exists() {
        create_template_file("CLAUDE.md", include_str!("../../templates/CLAUDE.md"))?;
        println!("✓ CLAUDE.md を作成しました");
    } else {
        println!("⚠ CLAUDE.md は既に存在します（スキップ）");
    }

    println!("\n✅ プロジェクトの初期化が完了しました");
    Ok(())
}

fn create_directory(path: &str) -> anyhow::Result<()> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn create_template_file(path: &str, content: &str) -> anyhow::Result<()> {
    if Path::new(path).exists() {
        print!(
            "ファイル '{}' は既に存在します。上書きしますか? [y/N]: ",
            path
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("スキップしました");
            return Ok(());
        }
    }

    fs::write(path, content)?;
    Ok(())
}
