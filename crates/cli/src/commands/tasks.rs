//! Tasks command implementation.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Executes the tasks command to split a specification into tasks.
pub fn execute(spec_id: &str, create_github_issues: bool) -> anyhow::Result<()> {
    let spec_file = format!(".aad/specs/{}.md", spec_id);

    if !Path::new(&spec_file).exists() {
        anyhow::bail!("エラー: {} が見つかりません", spec_file);
    }

    println!("{}をタスクに分割します...\n", spec_id);

    // 1. Read SPEC file
    let spec_content = fs::read_to_string(&spec_file)?;

    // 2. Extract MoSCoW requirements (simple implementation)
    let must_have_count = count_requirements(&spec_content, "### Must Have");
    let should_have_count = count_requirements(&spec_content, "### Should Have");

    println!("📋 要件分析:");
    println!("  - Must Have: {} 項目", must_have_count);
    println!("  - Should Have: {} 項目\n", should_have_count);

    // 3. Create tasks directory
    let tasks_dir = format!(".aad/tasks/{}", spec_id);
    fs::create_dir_all(&tasks_dir)?;

    // 4. Generate task files (sample)
    let total_tasks = must_have_count + should_have_count + 1; // +1 for quality check
    for i in 1..=total_tasks {
        let task_id = format!("{}-T{:02}", spec_id, i);
        let task_file = format!("{}/{}.md", tasks_dir, task_id);

        let task_content = generate_task_template(&task_id, spec_id);
        fs::write(&task_file, task_content)?;

        println!("✓ {} を作成しました", task_file);
    }

    // 5. Create GitHub Issues
    if create_github_issues {
        println!("\n🔗 GitHub Issues を作成しています...");
        create_github_issues_for_tasks(spec_id, total_tasks)?;
    }

    println!("\n✅ タスク分割が完了しました");
    Ok(())
}

fn count_requirements(content: &str, section: &str) -> usize {
    content
        .lines()
        .skip_while(|line| !line.starts_with(section))
        .take_while(|line| !line.starts_with("###") || line.starts_with(section))
        .filter(|line| line.trim_start().starts_with("- [ ]"))
        .count()
}

fn generate_task_template(task_id: &str, spec_id: &str) -> String {
    format!(
        r#"# {}: [タスク名]

## 基本情報

| 項目 | 内容 |
|------|------|
| タスクID | {} |
| SPEC | {} |
| 複雑度 | S（1-4時間） |
| 優先度 | Must |
| 依存 | なし |
| 担当 | 未アサイン |

---

## 概要

[タスクの概要]

---

## 作業内容

[実装内容を記述]

---

## 変更ファイル

| ファイル | 操作 | 説明 |
|----------|------|------|
| [ファイルパス] | 新規/変更 | [説明] |

---

## 受け入れ基準

- [ ] AC-1: [受け入れ基準]

---

## テストコマンド

```bash
[テストコマンド]
```
"#,
        task_id, task_id, spec_id
    )
}

fn create_github_issues_for_tasks(spec_id: &str, total_tasks: usize) -> anyhow::Result<()> {
    for i in 1..=total_tasks {
        let task_id = format!("{}-T{:02}", spec_id, i);
        let title = format!("{}: タスク実装", task_id);

        let output = Command::new("gh")
            .args([
                "issue",
                "create",
                "--title",
                &title,
                "--body",
                &format!("詳細: .aad/tasks/{}/{}.md", spec_id, task_id),
            ])
            .output()?;

        if output.status.success() {
            println!("  ✓ Issue作成: {}", task_id);
        } else {
            eprintln!("  ⚠ Issue作成失敗: {}", task_id);
        }
    }

    Ok(())
}
