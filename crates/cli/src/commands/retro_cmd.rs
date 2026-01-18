//! Retrospective command implementation for generating retrospective templates.

use chrono::Utc;
use std::fs;
use std::path::Path;

/// Executes the retro command to generate a retrospective template.
///
/// # Arguments
///
/// * `spec_id` - The specification ID (e.g., "SPEC-001")
/// * `no_append` - If true, do not append to CLAUDE.md
///
/// # Errors
///
/// Returns an error if:
/// - The retrospective template cannot be created
/// - CLAUDE.md cannot be updated (when no_append is false)
pub fn execute(spec_id: &str, no_append: bool) -> anyhow::Result<()> {
    // Generate retrospective template
    let retro_content = generate_template(spec_id)?;

    // Save to .aad/retrospectives/
    let retro_dir = Path::new(".aad/retrospectives");
    if !retro_dir.exists() {
        fs::create_dir_all(retro_dir)?;
    }

    let date = Utc::now().format("%Y%m%d");
    let retro_file = retro_dir.join(format!("RETRO-{}-{}.md", spec_id, date));

    fs::write(&retro_file, &retro_content)?;

    println!("✓ 振り返りテンプレートを生成しました: {}", retro_file.display());

    // Append to CLAUDE.md if not disabled
    if !no_append {
        append_to_claude_md(spec_id, &retro_content)?;
        println!("✓ CLAUDE.md に学びを追記しました");
    }

    Ok(())
}

/// Generates a retrospective template for the given spec ID.
fn generate_template(spec_id: &str) -> anyhow::Result<String> {
    let date = Utc::now().format("%Y-%m-%d");

    let template = format!(
        r#"# 振り返り: {}

**日付**: {}
**SPEC ID**: {}
**担当**: Claude Code
**ステータス**: 🚧 作業中

---

## 📋 概要

[SPEC の概要を記述]

### 完了したタスク

| タスクID | 内容 | ステータス |
|---------|------|-----------|
| {}-T01 | [タスク内容] | ✅ 完了 |

---

## 🎯 達成したこと (Keep)

### 1. [成功したこと]

**説明**:

**効果**:
-

---

## ❌ 課題・問題 (Problem)

### 1. [問題点]

**問題**:

**影響**:
-

**教訓**:

---

## 🚀 次回への改善案 (Try)

### 1. [改善策]

**実施内容**:
-

**効果**:
-

---

## 📊 品質メトリクス

### テスト結果
- [ ] Domain層: X テスト
- [ ] Application層: X テスト
- [ ] Infrastructure層: X テスト
- [ ] CLI層: X テスト

### コード量
- 新規ファイル: X ファイル
- コミット: X 個

### 品質指標
- [ ] Clippy警告: 0件
- [ ] Rustfmt通過
- [ ] ビルド成功率: 100%

---

## 🎓 技術的学び

### 1. [学んだこと]

**学習内容**:

**教訓**:

---

## 💡 CLAUDE.md更新提案

以下の学びをCLAUDE.mdに追加することを推奨します:

### 提案1: [タイトル]

```markdown
### {} - {}: [学びのタイトル]

**状況**: [何をしていたか]

**問題**: [何が起きたか]

**解決策**: [どう解決したか]

**学び**:
- [次回に活かすこと]
```

---

## 📝 まとめ

[まとめを記述]

**成功要因**:
-

**改善点**:
-

次のSPECでは、これらの学びを活かし、より効率的かつ高品質な実装を目指します。

---

**次のアクション**:
- [ ] CLAUDE.mdに学びを反映
- [ ] [その他のアクション]
"#,
        spec_id,
        date,
        spec_id,
        spec_id,
        date,
        spec_id,
    );

    Ok(template)
}

/// Appends retrospective learnings to CLAUDE.md.
fn append_to_claude_md(spec_id: &str, _retro_content: &str) -> anyhow::Result<()> {
    let claude_md_path = Path::new("CLAUDE.md");

    if !claude_md_path.exists() {
        anyhow::bail!("CLAUDE.md が見つかりません");
    }

    let date = Utc::now().format("%Y-%m-%d");

    // Generate learning entry template
    let learning_entry = format!(
        r#"
---

### {} - {}: [学びのタイトル]

**状況**: [何をしていたか]

**問題**: [何が起きたか]

**解決策**: [どう解決したか]

**学び**:
- [次回に活かすこと]
"#,
        date,
        spec_id
    );

    // Read existing content
    let content = fs::read_to_string(claude_md_path)?;

    // Find the "学びの蓄積" section
    if let Some(pos) = content.find("## 🧠 学びの蓄積") {
        // Find the next section after "学びの蓄積"
        let after_section = &content[pos..];

        // Look for the next "##" that marks a new section
        if let Some(next_section_pos) = after_section[20..].find("\n## ") {
            // Insert before the next section
            let insert_pos = pos + 20 + next_section_pos;
            let mut new_content = String::new();
            new_content.push_str(&content[..insert_pos]);
            new_content.push_str(&learning_entry);
            new_content.push('\n');
            new_content.push_str(&content[insert_pos..]);

            fs::write(claude_md_path, new_content)?;
        } else {
            // No next section, append to the end
            let mut new_content = content;
            new_content.push_str(&learning_entry);
            new_content.push('\n');
            fs::write(claude_md_path, new_content)?;
        }
    } else {
        anyhow::bail!("CLAUDE.md に '## 🧠 学びの蓄積' セクションが見つかりません");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_template() {
        let template = generate_template("SPEC-001").unwrap();

        assert!(template.contains("# 振り返り: SPEC-001"));
        assert!(template.contains("**SPEC ID**: SPEC-001"));
        assert!(template.contains("## 📋 概要"));
        assert!(template.contains("## 🎯 達成したこと (Keep)"));
        assert!(template.contains("## ❌ 課題・問題 (Problem)"));
        assert!(template.contains("## 🚀 次回への改善案 (Try)"));
        assert!(template.contains("## 📊 品質メトリクス"));
        assert!(template.contains("## 💡 CLAUDE.md更新提案"));
    }

    #[test]
    fn test_execute_creates_retro_file() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Execute retro command with no_append=true to skip CLAUDE.md update
        let result = execute("SPEC-001", true);

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());

        // Check that retrospective file was created
        let retro_dir = temp_dir.path().join(".aad/retrospectives");
        assert!(retro_dir.exists());

        // Find the created file
        let entries: Vec<_> = fs::read_dir(&retro_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

        assert_eq!(entries.len(), 1);

        let file_name = entries[0].file_name();
        let file_name_str = file_name.to_str().unwrap();
        assert!(file_name_str.starts_with("RETRO-SPEC-001-"));
        assert!(file_name_str.ends_with(".md"));

        // Verify content
        let content = fs::read_to_string(entries[0].path()).unwrap();
        assert!(content.contains("# 振り返り: SPEC-001"));
    }

    #[test]
    fn test_append_to_claude_md() {
        let temp_dir = TempDir::new().unwrap();
        let claude_md_path = temp_dir.path().join("CLAUDE.md");

        // Create a minimal CLAUDE.md
        let initial_content = r#"# プロジェクト指示書

## 🧠 学びの蓄積

このセクションはプロジェクトを通じて得た学びを記録します。

---

### 2026-01-15 - SPEC-001: 既存の学び

**状況**: テスト中

**問題**: なし

**解決策**: なし

**学び**: テストデータ

---

## 🔧 プロジェクト固有の設定

その他の設定
"#;

        fs::write(&claude_md_path, initial_content).unwrap();

        // Change to temp directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Append learning
        let result = append_to_claude_md("SPEC-999", "dummy content");

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());

        // Verify content was appended
        let updated_content = fs::read_to_string(&claude_md_path).unwrap();

        // Should contain the new entry
        assert!(updated_content.contains("### 2026-01-15 - SPEC-001: 既存の学び"));
        assert!(updated_content.contains("SPEC-999"));
        assert!(updated_content.contains("**状況**: [何をしていたか]"));

        // Should preserve the next section
        assert!(updated_content.contains("## 🔧 プロジェクト固有の設定"));

        // Verify order: original learning -> new learning -> next section
        let spec_001_pos = updated_content.find("SPEC-001").unwrap();
        let spec_999_pos = updated_content.find("SPEC-999").unwrap();
        let settings_pos = updated_content.find("## 🔧 プロジェクト固有の設定").unwrap();

        assert!(spec_001_pos < spec_999_pos);
        assert!(spec_999_pos < settings_pos);
    }

    #[test]
    fn test_append_to_claude_md_no_file() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = append_to_claude_md("SPEC-001", "dummy");

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("CLAUDE.md が見つかりません"));
    }

    #[test]
    fn test_append_to_claude_md_no_section() {
        let temp_dir = TempDir::new().unwrap();
        let claude_md_path = temp_dir.path().join("CLAUDE.md");

        // Create CLAUDE.md without the required section
        fs::write(&claude_md_path, "# プロジェクト指示書\n\n内容\n").unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = append_to_claude_md("SPEC-001", "dummy");

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("学びの蓄積"));
    }
}
