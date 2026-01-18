# SPEC-005: セッション管理 + 永続化 - タスク一覧

## 概要

**SPEC-ID**: SPEC-005
**タイトル**: セッション管理 + 永続化
**タスク数**: 7

---

## タスク一覧

### Must Have（6タスク）

| タスクID | タスク名 | 複雑度 | 推定時間 | 依存関係 | GitHub Issue |
|---------|---------|--------|---------|---------|--------------|
| SPEC-005-T01 | persistence/モジュール基盤実装 | S | 2-4h | なし | #41 |
| SPEC-005-T02 | SpecJsonRepo実装 | M | 4-6h | T01 | #42 |
| SPEC-005-T03 | TaskJsonRepo・SessionJsonRepo実装 | M | 4-6h | T01, T02 | #43 |
| SPEC-005-T04 | StyleFileAdapter実装 | M | 4-6h | T01 | #44 |
| SPEC-005-T05 | TokenReplacer実装 | S | 2-4h | T01 | #45 |
| SPEC-005-T06 | BackupAdapter実装 | M | 4-6h | T01 | #46 |

### Should Have（1タスク）

| タスクID | タスク名 | 複雑度 | 推定時間 | 依存関係 | GitHub Issue |
|---------|---------|--------|---------|---------|--------------|
| SPEC-005-T07 | persistコマンド実装 | M | 4-6h | T02, T03, T06 | #47 |

---

## 依存関係グラフ

```
T01 (persistence基盤)
 ├─→ T02 (SpecJsonRepo)
 │    └─→ T03 (TaskJsonRepo, SessionJsonRepo)
 │         └─→ T07 (persistコマンド)
 ├─→ T04 (StyleFileAdapter)
 ├─→ T05 (TokenReplacer)
 └─→ T06 (BackupAdapter)
           └─→ T07 (persistコマンド)
```

---

## 実行順序

### 第1波（単独実行）
- T01: persistence/モジュール基盤実装

### 第2波（並列実行可能）
- T02: SpecJsonRepo実装
- T04: StyleFileAdapter実装
- T05: TokenReplacer実装
- T06: BackupAdapter実装

### 第3波（並列実行可能）
- T03: TaskJsonRepo・SessionJsonRepo実装

### 第4波（並列実行可能）
- T07: persistコマンド実装

---

## 推定総時間

- **Must Have**: 20-32時間（約3-4日）
- **Should Have**: 4-6時間
- **合計**: 24-38時間（約3-5日）

---

## 注意事項

- T01完了後、T02, T04, T05, T06は並列実行可能
- 各タスクは1日以内で完了可能なサイズに分割済み
- JSON永続化にはserde_jsonを使用
- バックアップファイルはISO 8601形式のタイムスタンプを使用

---

**作成日**: 2026-01-18
**作成者**: Claude Code
