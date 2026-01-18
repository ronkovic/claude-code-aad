# Product Steering: claude-code-aad v2

## Purpose

自律型AI駆動開発（AAD）を実現するRust製CLIツール。claude-code-monitorとralph-tuiの機能を統合。

## Core Workflow (6 Phases)

| Phase | 目的 | 承認 |
|-------|------|------|
| SPEC | 仕様書作成 | 人間 |
| TASKS | タスク分割 | 人間 |
| TDD | テスト駆動開発 | CI |
| REVIEW | コードレビュー | 人間 |
| RETRO | 振り返り | - |
| MERGE | 統合 | CI |

## Orchestration Architecture

**3層オーケストレーション**:
1. **Master**: 全体進行管理（SPEC → TASKS → TDD → ...）
2. **Worker**: フェーズ実行（例: SPEC Worker）
3. **Claude Code**: 実際の作業実行（MCP経由）

## CLI Commands

- `aad init` - プロジェクト初期化
- `aad spec` - 仕様作成
- `aad tasks` - タスク分割
- `aad worktree` - worktree作成
- `aad orchestrate` - 全自動実行
- `aad monitor` - TUIダッシュボード
- `aad loop` - タスクループ
- `aad gate` - 品質ゲート
- `aad integrate` - PR作成
- `aad retro` - 振り返り

## Implementation Timeline

**期間**: 約10週間

### Phase 1: Domain Model & Config (Week 1-2)
- Domain層の完成
- 設定ファイル読み込み
- テンプレートシステム

### Phase 2: Application Layer (Week 2-3)
- ユースケース実装
- ポート定義

### Phase 3: CLI Commands (Week 3-4)
- 基本コマンド（init, spec, tasks, worktree）
- CLAUDE.md更新機能

### Phase 4: Orchestration (Week 4-6)
- Master/Worker実装
- Claude Code MCP連携
- セッション管理

### Phase 5: Persistence (Week 6-7)
- JSON Repository
- ワークフロー永続化

### Phase 6: TUI Dashboard (Week 7-8)
- Ratatui統合
- リアルタイム監視

### Phase 7: Task Loop (Week 8-9)
- ポーリング実装
- エラーハンドリング

### Phase 8: Quality & Integration (Week 9-10)
- 品質ゲート
- GitHub連携

## Milestones

| M | Phase | 成果物 |
|---|-------|--------|
| M1 | 1-2 | Domain + Config基盤 |
| M2 | 3 | CLI基本コマンド |
| M3 | 4 | オーケストレーション |
| M4 | 5 | 永続化 |
| M5 | 6 | TUIダッシュボード |
| M6 | 7 | タスクループ |
| M7 | 8 | 品質ゲート + GitHub連携 |

## Quality Gates

- **テストカバレッジ**: 80%以上
- **Lint**: cargo clippy合格
- **フォーマット**: rustfmt準拠
- **CI**: GitHub Actions green
