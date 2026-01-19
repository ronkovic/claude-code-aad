# SPEC-009: AAD改善要件

**作成日**: 2026-01-19

**担当者**: AI Agent

**ステータス**: Approved

**関連Issue**: -

---

## 📋 概要

AAD（AI-Assisted Development）システムの改善要件を実装する。監視可視化、PR連携強化、権限エスカレーション、自律実行継続などの11項目を4つのWaveに分けて実装し、開発者体験と自動化レベルを向上させる。

---

## 🎯 目的

### ビジネス目標
AADの実用性と信頼性を向上させ、人間の介入を最小化しながらも適切なエスカレーションを維持する。

### ユーザーストーリー
```
As a 開発者
I want to AADが自律的に作業を進め、必要な時だけ確認を求めてくれる
So that 開発効率を最大化しながら品質を担保できる
```

---

## 🔍 要件定義（MoSCoW）

### Must Have（必須）
これらがないとリリースできない機能

- [x] R1: 監視状況の可視化（完了通知・ローディングUI・worktree確認）- Wave 2
- [x] R2: PRマージ先をworktree元ブランチに固定 - Wave 4
- [x] R3: 子Agent権限不足時の親通知・承認後続行 - Wave 3
- [x] R4: PRにIssue含めて自動クローズ - Wave 1
- [x] R5: PR作成前の包括レビュー（コンフリクト・CI確認）- Wave 2
- [x] R6: GitHub Issueラベル事前作成 - Wave 1
- [x] R10: retrospectives振り返り必須化 - Wave 1
- [x] R11: 自律実行継続（ユーザーリアクション最小化）- Wave 4

### Should Have（重要）
できるだけ含めるべき機能

- [x] R8: Divide and Conquer精神（エラー時問題分割）- Wave 3
- [x] R9: PRマージ方法（Web/gh CLI）の明記 - Wave 1

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- [ ] R7: プロジェクト概要からSpec分割生成 - Wave 5（オプション）

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] Rust CLIコードの変更 - 理由: コマンドテンプレートの改善に集中

---

## 🔧 技術要件

### 変更対象ファイル

**コマンドテンプレート**:
- `.claude/commands/aad/tasks.md` - ラベル確認ロジック追加
- `.claude/commands/aad/integrate.md` - Issue自動クローズ、マージ方法詳細
- `.claude/commands/aad/orchestrate.md` - 監視UI、権限処理、自律実行
- `.claude/commands/aad/gate.md` - レビュー強化
- `.claude/commands/aad/retro.md` - 振り返り必須化

**プロンプトテンプレート**:
- `.aad/templates/CHILD-PROMPT.md` - 権限エスカレーション、D&C、自律化
- `.aad/templates/WORKER-PROMPT.md` - チェックリスト追加

**新規スクリプト**:
- `.claude/scripts/ensure-labels.sh` - ラベル確認・作成

---

## ✅ 受け入れ基準

### 機能テスト
- [ ] ラベルが不足している場合に自動作成される
- [ ] PRボディに`Closes #XX`が自動追加される
- [ ] マージ方法（Web/CLI）の手順が明記されている
- [ ] 振り返りがSPEC完了時に必須化されている
- [ ] 監視UIが進捗を可視化する
- [ ] 権限エスカレーションが適切に動作する
- [ ] 自律実行モードが動作する

### 品質要件
- [ ] 既存コマンドとの互換性維持
- [ ] ドキュメントが明確で理解しやすい

---

## 📚 参考資料

- [AAD改善要件 実装計画](../../.claude/plans/)
- [.kiro/specs/phase9-aad-enhancements](../../.kiro/specs/phase9-aad-enhancements/)

---

## 📝 変更履歴

| 日付 | バージョン | 変更内容 | 変更者 |
|------|-----------|----------|--------|
| 2026-01-19 | 1.0 | 初版作成 | AI Agent |

---

## ✅ 承認

- [x] 技術レビュー完了（担当: AI Agent、日付: 2026-01-19）
- [x] 最終承認（担当: User、日付: 2026-01-19）

---

**注意**: この仕様書は承認後にタスク分割（`/aad:tasks SPEC-009`）を実行してください。
