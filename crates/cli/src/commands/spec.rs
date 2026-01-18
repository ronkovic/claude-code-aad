//! Spec command implementation.

use std::fs;
use std::path::Path;

/// Executes the spec command to create a new specification file.
pub fn execute(spec_id: &str) -> anyhow::Result<()> {
    let file_path = format!(".aad/specs/{}.md", spec_id);

    if Path::new(&file_path).exists() {
        anyhow::bail!("エラー: {} は既に存在します", file_path);
    }

    // Generate template
    let template = generate_spec_template(spec_id);

    fs::write(&file_path, template)?;
    println!("✓ {} を作成しました", file_path);

    Ok(())
}

fn generate_spec_template(spec_id: &str) -> String {
    let current_date = chrono::Local::now().format("%Y-%m-%d");

    format!(
        r#"# {}: [タイトル]

**作成日**: {}

**担当者**: 未定

**ステータス**: Draft

**関連Issue**: N/A

---

## 📋 概要

[仕様の概要を記述]

---

## 🎯 目的

### ビジネス目標
[ビジネス目標を記述]

### ユーザーストーリー
```
As a [ユーザー]
I want to [やりたいこと]
So that [得られる価値]
```

---

## 🔍 要件定義（MoSCoW）

### Must Have（必須）
これらがないとリリースできない機能

- [ ] **REQ-1**: [要件名] - [要件説明]

### Should Have（重要）
できるだけ含めるべき機能

- なし

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- なし

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] **[機能名]** - 理由: [理由]

---

## ✅ 受け入れ基準

テスト可能な形式で記述すること

### 機能テスト

#### REQ-1: [要件名]
- [ ] AC-1.1: [受け入れ基準]
- [ ] AC-1.2: [受け入れ基準]

### 非機能テスト
- [ ] パフォーマンス要件を満たす
- [ ] セキュリティ要件を満たす

---

## 📝 変更履歴

| 日付 | バージョン | 変更内容 | 変更者 |
|------|-----------|----------|--------|
| {} | 1.0 | 初版作成 | 未定 |

---

## ✅ 承認

- [ ] 技術レビュー完了（担当: 未定、日付: 未定）
- [ ] ビジネスレビュー完了（担当: 未定、日付: 未定）
- [ ] 最終承認（担当: 未定、日付: 未定）
"#,
        spec_id, current_date, current_date
    )
}
