# SPEC-008: タスク分割サマリー

## タスク一覧

### Wave 1: 基礎実装（並列可能）
- **SPEC-008-T01** (M: 4-8時間): QualityService実装
  - フェーズごとのゲート条件検証ロジック
  - 検証結果レポート生成機能
  - 依存: なし

- **SPEC-008-T03** (M: 5-7時間): GhCliAdapter実装
  - GitHub CLI ラッパー実装
  - PR作成、マージ、Issue操作
  - 依存: なし

- **SPEC-008-T05** (S: 3-4時間): retro コマンド実装 ⚠️ Should Have
  - 振り返りテンプレート生成
  - CLAUDE.md への学びの追記
  - 依存: なし

### Wave 2: コマンド実装（Wave 1完了後）
- **SPEC-008-T02** (S: 2-3時間): gate コマンド実装
  - フェーズ指定でゲートチェック実行
  - QualityServiceを呼び出してレポート表示
  - 依存: T01

- **SPEC-008-T04** (M: 4-6時間): integrate コマンド実装
  - PR作成 → マージ → worktree削除の統合フロー
  - 各フェーズのゲートチェックを実行
  - 依存: T01, T03

### Wave 3: CI/CD設定（Wave 2完了後）
- **SPEC-008-T06** (M: 4-6時間): CI/CD設定実装
  - GitHub Actions ワークフロー
  - PR作成時に自動ゲートチェック
  - 依存: T02

## 依存関係グラフ

```
        T01 (QualityService)
         |
         +-- T02 (gate コマンド)
         |    |
         |    +-- T06 (CI/CD設定)
         |
         +-- T04 (integrate コマンド)
              |
         T03 (GhCliAdapter)

T05 (retro コマンド) [独立、Should Have]
```

## 実装戦略

### 推奨アプローチ: Wave方式

**Wave 1** (並列実行可能):
- T01 + T03 + T05 を同時実行
- 所要時間: max(4-8h, 5-7h, 3-4h) = 5-8時間

**Wave 2** (Wave 1完了後):
- T02 + T04 を同時実行（T02はT01依存、T04はT01+T03依存）
- 所要時間: max(2-3h, 4-6h) = 4-6時間

**Wave 3** (Wave 2完了後):
- T06 を実行（T02依存）
- 所要時間: 4-6時間

**合計所要時間**: 13-20時間（逐次実行: 22-34時間）

### 優先度付け（MoSCoW）

**Must Have** (必須):
- T01, T02, T03, T04, T06

**Should Have** (推奨):
- T05 (retro コマンド)

**Could Have** (任意):
- なし

**Won't Have** (今回対象外):
- なし

## 注意事項

1. **T05はShould Have**: 時間が厳しい場合はスキップ可能
2. **T06はCI/CD**: GitHub Actionsの動作確認が必要
3. **Wave 1の並列実行**: T01とT03は完全に独立しているため並列実行推奨
4. **統合テスト**: T04実装後にE2Eテストを実施

## 次のステップ

1. `/aad:orchestrate SPEC-008` でWave実行
2. または `/aad:worktree SPEC-008 T01` で個別タスク実行
3. 各Wave完了後にゲートチェック実施
