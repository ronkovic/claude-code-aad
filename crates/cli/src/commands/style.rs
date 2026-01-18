//! Style command implementation.

use domain::value_objects::StyleName;
use infrastructure::config::StyleConfig;
use std::fs;
use std::path::Path;

/// Lists all available styles.
pub fn list() -> anyhow::Result<()> {
    let config = StyleConfig::load(Path::new("config/styles.toml"))?;

    println!("📋 利用可能なスタイル:\n");

    for name in config.style_names() {
        println!("  • {}", name.as_str());
    }

    Ok(())
}

/// Applies a style to CLAUDE.md.
pub fn apply(style_name: &str) -> anyhow::Result<()> {
    let config = StyleConfig::load(Path::new("config/styles.toml"))?;
    let style_name_obj = StyleName::new(style_name)?;

    if !config.has_style(&style_name_obj) {
        anyhow::bail!("エラー: スタイル '{}' が見つかりません", style_name);
    }

    // Load CLAUDE.md template
    let claude_md_path = "CLAUDE.md";
    if !Path::new(claude_md_path).exists() {
        anyhow::bail!("エラー: CLAUDE.md が見つかりません。先に 'aad init' を実行してください");
    }

    let template = fs::read_to_string(claude_md_path)?;

    // Apply token substitution
    let token_map = config
        .get_token_map(&style_name_obj)
        .ok_or_else(|| anyhow::anyhow!("トークンマップが取得できません"))?;

    let result = token_map.replace_tokens(&template)?;

    // Save result
    fs::write(claude_md_path, result)?;

    println!("✓ スタイル '{}' を CLAUDE.md に適用しました", style_name);

    Ok(())
}
