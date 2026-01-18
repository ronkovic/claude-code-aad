# ハンドオフドキュメント

## 📅 セッション情報

**作成日時**: 2026-01-18

**現在のフェーズ**: 設計完了 → 実装待ち

---

## 🎯 目標

### 最終目標
claude-code-aad を Rust + Ratatui で完全新設計し、以下の外部ツールの機能を統合する：
- **claude-code-monitor**: 複数Claude Codeセッションのリアルタイム監視
- **ralph-tui**: AIエージェントのタスクループ自動化

### 現在のマイルストーン
アーキテクチャ設計完了。Rustプロジェクトの初期構築に移行予定。

---

## ✅ 完了したこと

### 外部プロジェクト調査
- **claude-code-monitor** (https://github.com/onikan27/claude-code-monitor)
  - TypeScript + Ink (React TUI)
  - macOS専用（AppleScript使用）
  - ファイルベース監視（`~/.claude-monitor/sessions.json`）

- **ralph-tui** (https://github.com/subsy/ralph-tui)
  - Bun + OpenTUI (React)
  - タスクループエンジン
  - PRDウィザード、セッション永続化、完了検出

### 技術スタック比較
5つのオプションを詳細比較：
| オプション | 総合スコア |
|------------|------------|
| A: Bun/TS | 4.2 |
| B: Node/TS | 4.3 |
| C: Bash+TS | 3.8 |
| D: Go | 4.0 |
| E: Rust | 4.0 |

### 技術選定
**Rust + Ratatui** を選定。理由：
- 最高のパフォーマンス（サブミリ秒レンダリング）
- メモリ安全（GCなし、コンパイル時保証）
- 単一バイナリ（依存関係ゼロで配布容易）
- 堅牢性（Result型による厳格なエラーハンドリング）

### アーキテクチャ設計
クリーンアーキテクチャに基づく4層構造を設計：

```
crates/
├── domain/          # ビジネスロジック（外部依存なし）
├── application/     # ユースケース、ポート
├── infrastructure/  # アダプター実装
├── cli/             # CLI バイナリ
└── tui/             # TUI コンポーネント
```

### ドキュメント作成
`./docs/ARCHITECTURE.md` に整理した設計書を保存

### スタイルシステム設計追加（2026-01-18）
既存の `switch-style.sh` 機能を Rust アーキテクチャに統合する設計を追加：
- `docs/ARCHITECTURE.md` にスタイルシステムのセクションを追加
- `config/styles.toml` を作成（standard/sage スタイル定義）
- Domain/Application/Infrastructure 各層にスタイル関連コンポーネントを定義

### 実装フェーズ詳細ドキュメント作成（2026-01-18）
Phase 1-8 の詳細な実装計画を `docs/IMPLEMENTATION-PHASES.md` として作成：
- 各フェーズの目標、成果物、タスク一覧、完了条件を明記
- マイルストーン（M1-M7）の定義
- 検証方法とリスク管理を記載
- HANDOFF.md の次のステップを Phase 1 の詳細に更新

---

## ✅ うまくいったこと

### 段階的な技術選定プロセス
- 最初に全選択肢（Bun/TS, Node/TS, Bash+TS, Go）を提示
- ユーザーからRustの提案を受け、追加評価
- 各選択肢のメリット・デメリットを明確化し、合意形成

### クリーンアーキテクチャの採用
- ユーザーからの「クリーンアーキテクチャを採用して」という要望に即座に対応
- 4層構造と依存関係ルールを明確に定義

### クレート名のシンプル化
- `aad-domain` → `domain` のようにプレフィックスを削除
- ユーザーのフィードバックに基づき即座に修正

---

## ❌ うまくいかなかったこと

### プランファイルの肥大化
- 技術スタック比較情報を残しすぎて、プランファイルが冗長になった
- 理由: 選択済みの比較情報は不要だった
- 対策: `./docs/ARCHITECTURE.md` に整理して保存

### 複数回のExitPlanMode失敗
- ユーザーが追加の修正を求めるたびにExitPlanModeが拒否された
- 理由: プランの詳細が固まっていなかった
- 対策: ユーザーの要望を全て確認してからExitPlanModeを呼ぶ

---

## 🔄 次のステップ

### Phase 1: プロジェクト構造 + Domain基盤（1週間）

詳細は `./docs/IMPLEMENTATION-PHASES.md` の Phase 1 を参照。

#### 1. Rustワークスペースの初期化
   - ルート `Cargo.toml` でワークスペース定義
   - `cargo new --lib crates/domain`
   - `cargo new --lib crates/application`
   - `cargo new --lib crates/infrastructure`
   - `cargo new crates/cli`
   - `cargo new --lib crates/tui`

#### 2. domain クレートの実装
   - **Entities 実装**:
     - `Spec`: 仕様エンティティ
     - `Task`: タスクエンティティ
     - `Session`: セッションエンティティ
     - `Workflow`: ワークフローエンティティ
     - `Style`: スタイルエンティティ

   - **Value Objects 実装**:
     - `SpecId`, `TaskId`: ID型
     - `Phase`: フェーズ列挙型（SPEC, TASKS, TDD, REVIEW, RETRO, MERGE）
     - `Status`: ステータス列挙型（pending, in_progress, done, failed）
     - `Priority`: 優先度列挙型（Must, Should, Could, Won't）
     - `StyleName`: スタイル名
     - `TokenMap`: トークンマッピング

   - **Repository トレイト定義**:
     - `SpecRepository`
     - `TaskRepository`
     - `SessionRepository`

   - **単体テスト作成**:
     - エンティティの振る舞いテスト
     - バリューオブジェクトのバリデーションテスト

#### 完了条件
- ✅ `cargo build` が成功
- ✅ `cargo test` が全て pass
- ✅ ドメインモデルが定義済み
- ✅ リポジトリトレイトが定義済み

---

## 📝 重要な決定事項

### 2026-01-18 - 技術スタック選定
**背景**: claude-code-aad v2 の技術スタックを決定する必要があった

**選択肢**:
1. TypeScript/Bun - 高速起動、モダンTUI
2. Node.js/TypeScript - 成熟エコシステム
3. Bash + TS拡張 - 現行資産活用
4. Go - 単一バイナリ、学習曲線緩やか
5. Rust - 最高性能、メモリ安全

**決定**: Rust + Ratatui

**理由**:
- ユーザーがRustを希望
- 最高のパフォーマンスと堅牢性
- 単一バイナリで配布容易

### 2026-01-18 - アーキテクチャ選定
**背景**: フォルダ構成の設計方針を決定する必要があった

**決定**: クリーンアーキテクチャ（4層構造）

**理由**:
- 依存関係の方向を明確化
- テスト容易性
- 長期保守に適した設計

---

## 🔗 参考リソース

- [設計書](./docs/ARCHITECTURE.md) - 完全なアーキテクチャ設計
- [実装フェーズ詳細](./docs/IMPLEMENTATION-PHASES.md) - Phase 1-8 の詳細実装計画
- [claude-code-monitor](https://github.com/onikan27/claude-code-monitor) - セッション監視の参考
- [ralph-tui](https://github.com/subsy/ralph-tui) - タスクループの参考
- [Ratatui](https://ratatui.rs/) - Rust TUIフレームワーク

---

## 💡 引き継ぎメモ

- `./docs/ARCHITECTURE.md` を読めば、アーキテクチャの全体像がわかる
- 依存クレート一覧もARCHITECTURE.mdに記載済み
- CLIコマンド体系は13コマンド（init, spec, tasks, worktree, orchestrate, status, monitor, loop, persist, gate, integrate, retro, **style**）
- スタイルシステムは `config/styles.toml` で定義される（standard/sage の2スタイル）
- 既存の `.claude/scripts/switch-style.sh` のロジックを Rust に移植する

---

## 🚀 再開用プロンプト例

```
HANDOFF.md と以下のドキュメントを読んで現在の状況を把握してください：
- ./docs/ARCHITECTURE.md（アーキテクチャ設計）
- ./docs/IMPLEMENTATION-PHASES.md（Phase 1-8 の実装計画）

Phase 1: プロジェクト構造 + Domain基盤 の実装を開始してください：
1. Cargo.toml でワークスペースを定義
2. 各クレート（domain, application, infrastructure, cli, tui）を作成
3. domain クレートのエンティティを実装

開始前に以下を確認：
1. rustup がインストールされているか
2. cargo のバージョン
3. Phase 1 の完了条件を確認
```

---

**最終更新**: 2026-01-18
**次回更新推奨**: 実装開始後
