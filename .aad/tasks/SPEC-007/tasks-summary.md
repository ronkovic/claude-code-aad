# SPEC-007: タスクループ + 完了検出 - タスク一覧

**作成日**: 2026-01-18

**SPEC**: [SPEC-007](../../specs/SPEC-007.md)

---

## 📋 タスク概要

| タスクID | タイトル | 複雑度 | 依存 | ステータス | Issue |
|---------|---------|--------|------|-----------|-------|
| SPEC-007-T01 | LoopEngine基盤実装 | M | なし | 🟡 未着手 | [#57](https://github.com/ronkovic/claude-code-aad/issues/57) |
| SPEC-007-T02 | CompletionDetector実装 | M | なし | 🟡 未着手 | [#58](https://github.com/ronkovic/claude-code-aad/issues/58) |
| SPEC-007-T03 | 依存関係に基づくタスク進行ロジック実装 | M | T01, T02 | 🟡 未着手 | [#59](https://github.com/ronkovic/claude-code-aad/issues/59) |
| SPEC-007-T04 | loop コマンド実装 | M | T01, T02, T03 | 🟡 未着手 | [#60](https://github.com/ronkovic/claude-code-aad/issues/60) |
| SPEC-007-T05 | TUI統合（ループ状態の可視化） | M | T01, T04, SPEC-006 | 🟡 未着手 | [#61](https://github.com/ronkovic/claude-code-aad/issues/61) |
| SPEC-007-T06 | 品質チェック | S | T01-T05 | 🟡 未着手 | [#62](https://github.com/ronkovic/claude-code-aad/issues/62) |

---

## 🎯 並列実行可能グループ（Wave分割）

### Wave 1: 基盤実装（並列可能）
- **SPEC-007-T01**: LoopEngine基盤実装
- **SPEC-007-T02**: CompletionDetector実装

**推定時間**: 4-6時間（並列実行で最大6時間）

### Wave 2: タスク進行ロジック
- **SPEC-007-T03**: 依存関係に基づくタスク進行ロジック実装

**推定時間**: 5-7時間

### Wave 3: CLIコマンド
- **SPEC-007-T04**: loop コマンド実装

**推定時間**: 4-6時間

### Wave 4: TUI統合
- **SPEC-007-T05**: TUI統合（ループ状態の可視化）

**推定時間**: 5-7時間

### Wave 5: 品質保証
- **SPEC-007-T06**: 品質チェック

**推定時間**: 2-3時間

---

## ⏱️ 推定総時間

### Must Have（REQ-1〜4）
- **Wave 1**: 4-6時間（並列）
- **Wave 2**: 5-7時間
- **Wave 3**: 4-6時間
- **合計**: 13-19時間

### Should Have（REQ-5）
- **Wave 4**: 5-7時間

### 品質保証
- **Wave 5**: 2-3時間

### 総合計
- **最小**: 20時間（約3日）
- **最大**: 29時間（約4日）

---

## 🔗 依存関係グラフ

```
T01 (LoopEngine基盤)
 ├─→ T03 (タスク進行ロジック)
 │    └─→ T04 (loop コマンド)
 │         └─→ T05 (TUI統合)
 │              └─→ T06 (品質チェック)
 └─→ (同時実行可能) ─┐
                     │
T02 (CompletionDetector) ─┘
 └─→ T03 (タスク進行ロジック)
      └─→ T04 (loop コマンド)
           └─→ T05 (TUI統合)
                └─→ T06 (品質チェック)
```

---

## 📝 実装時の注意事項

### 過去の学びからの適用

1. **Domain層の型設計先行**（SPEC-004の学び）
   - `LoopState`、`CompletionPattern` などの型を最初に完全定義
   - newtypeパターンで型安全性を最大化

2. **Clippy即時実行**（SPEC-006の学び）
   - 各タスク完了時に `cargo clippy -p [crate] -- -D warnings` を実行
   - 警告を見つけたら即座に修正

3. **save()メソッドの統合テスト**（SPEC-005の学び）
   - ループ状態保存機能は必ず統合テストで確認
   - ファイルが実際に作成されることを検証

4. **作業前のディレクトリ確認**（SPEC-006の学び）
   - `pwd` でディレクトリを確認してからコマンド実行
   - 新規ファイル作成時は特に注意

5. **Builder パターンの活用**（SPEC-004の学び）
   - テストでは `Default::default()` やBuilderパターンを活用
   - Config構造体の拡張に備える

---

## 🚀 次のステップ

### 1️⃣ 最初のタスクに着手
```bash
/aad:worktree SPEC-007-T01
```
独立した作業環境を構築します。新セーブスロットのようなものです。

### 2️⃣ または全自動実行
```bash
/aad:orchestrate SPEC-007
```
全タスクを自動実行できます。放置してお茶が飲めます。

---

**作成者**: Claude Code

**最終更新**: 2026-01-18
