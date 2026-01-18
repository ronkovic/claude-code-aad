//! Worktree command implementation.

use std::path::Path;
use std::process::Command;

/// Executes the worktree command to create a Git worktree and branch.
pub fn execute(spec_id: &str) -> anyhow::Result<()> {
    // 1. Check if Git repository exists
    if !Path::new(".git").exists() {
        anyhow::bail!("エラー: Git リポジトリではありません");
    }

    let worktree_path = format!("../aad-{}", spec_id);
    let branch_name = format!("feature/{}", spec_id);

    // 2. Check if worktree already exists
    if Path::new(&worktree_path).exists() {
        anyhow::bail!("エラー: worktree '{}' は既に存在します", worktree_path);
    }

    println!("Git worktree を作成しています...\n");

    // 3. Create branch
    let create_branch = Command::new("git")
        .args(["checkout", "-b", &branch_name])
        .output()?;

    if !create_branch.status.success() {
        // Branch already exists, checkout instead
        Command::new("git")
            .args(["checkout", &branch_name])
            .output()?;
    }

    println!("✓ ブランチ '{}' を作成しました", branch_name);

    // 4. Create worktree
    let worktree_output = Command::new("git")
        .args(["worktree", "add", &worktree_path, &branch_name])
        .output()?;

    if !worktree_output.status.success() {
        let error = String::from_utf8_lossy(&worktree_output.stderr);
        anyhow::bail!("worktree 作成に失敗しました: {}", error);
    }

    println!("✓ worktree '{}' を作成しました", worktree_path);

    // 5. Return to original branch
    Command::new("git").args(["checkout", "-"]).output()?;

    println!("\n✅ worktree 作成が完了しました");
    println!("\n次のステップ:");
    println!("  cd {}", worktree_path);

    Ok(())
}
