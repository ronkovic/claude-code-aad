# コマンドリファレンス

利用可能な全コマンドの詳細リファレンスです。

---

## 📑 目次

- [初期化](#初期化)
- [ワークフロー](#ワークフロー)
- [コンテキスト管理](#コンテキスト管理)
- [品質管理](#品質管理)

---

## 初期化

### `/aad:init`

テンプレートを初期化し、プロジェクトに合わせてカスタマイズします。

**基本使用法**:
```
/aad:init
```

**オプション**:
```
/aad:init --quick              # デフォルト値で高速初期化
/aad:init --reconfigure        # 再設定
/aad:init --export=config.json # 設定エクスポート
/aad:init --import=config.json # 設定インポート
```

**実行内容**:
1. プロジェクト情報収集
2. CLAUDE.mdカスタマイズ
3. 品質基準設定
4. GitHub連携設定
5. 初回コミット

**出力**:
- CLAUDE.md更新
- .github/workflows/追加

**関連コマンド**: なし

**参考**: [.claude/commands/aad/init.md](../.claude/commands/aad/init.md)

---

## ワークフロー

### `/aad:tasks`

SPEC仕様書を分析し、実装可能なタスクに分割します（GitHub Issues作成はオプション）。

**基本使用法**:
```
/aad:tasks SPEC-001              # タスク分割 + GitHub Issues作成
/aad:tasks SPEC-001 --no-issues  # Issues作成をスキップ
```

**実行内容**:
1. .aad/specs/SPEC-001.md読み込み
2. MoSCoW要件を分析
3. タスク分割（SPEC-001-T01, T02...）
4. 複雑度推定（S/M/L）
5. タスクファイル作成（.aad/tasks/SPEC-001/）
6. GitHub Issues作成（`--no-issues`未指定時）
7. HANDOFF.md更新

**出力例**:
```
✅ Must Have:
  - SPEC-001-T01: データベーススキーマ (S) → #12
  - SPEC-001-T02: 認証API実装 (M) → #13

📂 タスクファイル作成完了: .aad/tasks/SPEC-001/
🔗 GitHub Issues作成完了
```

**完了条件**:
- [ ] 全タスクにID付与
- [ ] 依存関係明記
- [ ] 複雑度設定
- [ ] GitHub Issues作成（`--no-issues`未指定時）
- [ ] ⚠️ 人間承認必須

**関連コマンド**: `/aad:worktree`, `/aad:status`, `/aad:orchestrate`

**参考**: [.claude/commands/aad/tasks.md](../.claude/commands/aad/tasks.md)

---

### `/aad:worktree`

Git worktreeを使用して、元のフォルダに影響を与えずに並列開発環境を構築します。

**基本使用法**:
```
/aad:worktree SPEC-001-T01
```

**実行内容**:
1. タスク情報読み込み
2. ブランチ作成（feature/SPEC-001-T01）
3. worktree作成（../project-name-T01/）
4. 依存関係インストール
5. 環境変数コピー
6. HANDOFF.md更新

**出力例**:
```
🌿 ブランチ作成: feature/SPEC-001-T01
📂 worktree作成: /Users/user/workspace/my-project-T01/
⚙️  セットアップ完了

次のステップ:
1. cd ../my-project-T01
2. claude --dangerously-skip-permissions
```

**アーキテクチャ**:
```
my-project/        # デフォルトブランチ - 調整役
my-project-T01/    # worktree - Worker 1
my-project-T02/    # worktree - Worker 2
```

**関連コマンド**: `/aad:tasks`, `/aad:integrate`, `/aad:status`

**参考**: [.claude/commands/aad/worktree.md](../.claude/commands/aad/worktree.md)

---

### `/aad:status`

現在のプロジェクト全体の進捗状況を一覧表示します。

**基本使用法**:
```
/aad:status
```

**特定SPECのみ**:
```
/aad:status SPEC-001
```

**フィルタオプション**:
```
/aad:status --active           # 進行中のみ
/aad:status --worktrees        # worktree一覧のみ
/aad:status --quality          # 品質メトリクスのみ
```

**出力例**:
```
📊 プロジェクト進捗状況

SPEC-001: ユーザー認証機能 [In Progress]
  📊 タスク: 3/5 完了 (60%)
  ├─ ✅ T01: データベーススキーマ - Merged
  ├─ ✅ T02: 認証API実装 - Merged
  ├─ 🚧 T03: フロントエンドUI - In Progress
  ├─ ⏸️  T04: パスワードリセット - Open
  └─ ⏸️  T05: ソーシャルログイン - Open

🌿 アクティブなworktree:
1. ../my-project-T03 [feature/SPEC-001-T03]
   状態: 開発中, PR: #18 (Draft), CI: ✅ Pass

📈 全体サマリー:
タスク: 8件 (完了: 3, 進行中: 2, 未着手: 3)
```

**関連コマンド**: `/aad:tasks`, `/aad:worktree`, `/aad:integrate`

**参考**: [.claude/commands/aad/status.md](../.claude/commands/aad/status.md)

---

### `/aad:integrate`

タスク完了後、PRをデフォルトブランチにマージしてworktreeを削除します。

**基本使用法**:
```
/aad:integrate SPEC-001-T01
```

**オプション**:
```
/aad:integrate SPEC-001-T01 --merge-strategy=merge  # Merge commit
/aad:integrate SPEC-001-T01 --no-delete-branch      # ブランチ保持
```

**実行内容**:
1. 品質チェック（/aad:gate TDD）
2. PRステータス確認
3. マージ実行（Squash merge）
4. Issue閉鎖
5. worktree削除
6. HANDOFF.md更新

**出力例**:
```
🔍 品質チェック:
   ✅ テスト: green
   ✅ カバレッジ: 85%
   ✅ Lint: 0 errors

⚠️  デフォルトブランチにマージします。よろしいですか？ (y/n)

🔀 マージ実行:
   ✅ PR #18 をマージしました

✅ 統合完了！
```

**マージ戦略**:
- **Squash Merge**（デフォルト）: 履歴を1つに
- **Merge Commit**: 履歴を保持
- **Rebase Merge**: リニアな履歴

**関連コマンド**: `/aad:worktree`, `/aad:gate`, `/aad:status`

**参考**: [.claude/commands/aad/integrate.md](../.claude/commands/aad/integrate.md)

---

### `/aad:orchestrate`

SPECからタスク分割、並列開発、統合まで全て自動実行します（3層アーキテクチャ）。

**アーキテクチャ概要**:
```
親 Claude Code (このセッション)
    │
    ├─→ 子 Claude Code (SPEC-001担当) ──→ サブエージェント (T01, T02, T03...)
    └─→ 子 Claude Code (SPEC-002担当) ──→ サブエージェント (T01, T02...)
```

- **親**: 複数SPECの管理、人間とのインターフェース、エスカレーション処理
- **子**: SPEC単位のブランチ管理、Wave計画、軽微な判断を自律実行
- **サブエージェント**: 個々のタスクの実装・テスト

**基本使用法**:
```
/aad:orchestrate SPEC-001
```

**オプション**:
```
/aad:orchestrate SPEC-001 --dry-run      # 実行前確認
/aad:orchestrate SPEC-001 --workers=3    # 並列度指定
/aad:orchestrate SPEC-001 --from=TDD     # 途中から開始
/aad:orchestrate SPEC-001 --pause-on-error
```

**実行フロー**:
```
Phase 1: SPEC確認
Phase 2: タスク分割
Phase 3: 依存関係解析
Phase 4: 並列ワーカー起動
Phase 5: 振り返り
```

**Wave実行の例**:
```
Wave 1: T01 (1ワーカー)
Wave 2: T02 (1ワーカー)
Wave 3: T03, T04, T05 (3並列)
```

**出力例**:
```
🚀 Wave 1 起動:
   Worker-T01: 起動完了

✅ Wave 1 完了:
   Worker-T01: ✅ 完了 (25分)

✅ オーケストレーション完了！
   総所要時間: 3時間10分
   完了タスク: 5/5
```

**関連コマンド**: `/aad:tasks`, `/aad:worktree`, `/aad:integrate`

**参考**: [.claude/commands/aad/orchestrate.md](../.claude/commands/aad/orchestrate.md)

---

### `/aad:retro`

SPEC完了後に振り返りを実行し、学びを蓄積します。

**基本使用法**:
```
/aad:retro SPEC-001
```

**実行内容**:
1. 振り返りログ作成（.aad/retrospectives/）
2. Keep/Problem/Try記録
3. 品質ゲート分析
4. CLAUDE.md更新提案

**出力例**:
```
✅ 振り返りログ作成: .aad/retrospectives/RETRO-SPEC-001-20260111.md

📊 サマリー:
- 完了タスク: 5/5
- カバレッジ: 85%

💡 CLAUDE.md更新提案:
1. テストデータのクリーンアップ対策
2. API設計時の型定義先行

承認しますか？ (y/n)
```

**完了条件**:
- [ ] .aad/retrospectives/にログ作成
- [ ] Keep/Problem/Try記載
- [ ] 技術的な学び明記
- [ ] CLAUDE.md更新提案

**関連コマンド**: `/aad:tasks`, `/aad:status`

**参考**: [.claude/commands/aad/retro.md](../.claude/commands/aad/retro.md)

---

## コンテキスト管理

### `/aad:context`

現在のコンテキスト使用率を確認し、70%ルールに基づく推奨アクションを提示します。

**基本使用法**:
```
/aad:context
```

**詳細表示**:
```
/aad:context --verbose
/aad:context --history
```

**出力例**:
```
📊 コンテキスト使用状況

使用率: 🟠 78% (156,000 / 200,000 tokens)

ステータス: 🟠 警告

推奨アクション:
  🔔 /aad:handoff の実行を強く推奨
  ❌ 新しい大きなタスクの開始は避けてください
```

**70%ルール**:

| 使用率 | ステータス | アクション |
|--------|------------|------------|
| 0-50% | 🟢 快適 | 通常作業 |
| 50-70% | 🟡 注意 | 大タスクは新セッション |
| 70-85% | 🟠 警告 | /aad:handoff推奨 |
| 85-95% | 🔴 危険 | 即座に/aad:handoff |
| 95%+ | ⛔ 限界 | 自動圧縮 |

**ステータスバー**: `context-bar.sh`で常時表示

**関連コマンド**: `/aad:handoff`, `/aad:clone`, `/aad:half-clone`

**参考**: [.claude/commands/aad/context.md](../.claude/commands/aad/context.md)

---

### `/aad:handoff`

新しい会話セッションに引き継ぐ情報を記載したドキュメントを作成します。

**基本使用法**:
```
/aad:handoff
```

**実行内容**:
1. HANDOFF.mdを更新
2. 完了したタスク記録
3. 進行中のタスク状態
4. 成功・失敗したアプローチ
5. 次のセッションで取り組むこと

**出力例**:
```
✅ HANDOFF.md を更新しました

📝 記録内容:
- 完了タスク: 3件
- 進行中: 2件
- 次のステップ: T04の実装

新セッションでの再開プロンプト:
  HANDOFF.md を読んで、T04の実装を開始してください
```

**再開方法**:
```
# 新セッションで
HANDOFF.md を読んで現在の状況を把握してください。
次のタスクに取り組んでください: SPEC-001-T04
```

**関連コマンド**: `/aad:context`, `/aad:clone`, `/aad:half-clone`

**参考**: [.claude/commands/aad/handoff.md](../.claude/commands/aad/handoff.md)

---

### `/aad:clone`

会話全体を複製して別のアプローチを試します。

**基本使用法**:
```
/aad:clone
```

**実行内容**:
1. 現在の会話をクローン
2. タグ付き新会話作成: `[CLONED from ...]`
3. 全メッセージ保持

**使用シーン**:
- 別のアプローチを試したい
- 実験的な変更をテスト
- 複数の解決策を比較

**関連コマンド**: `/aad:half-clone`, `/aad:handoff`

**参考**: [.claude/commands/aad/clone.md](../.claude/commands/aad/clone.md)

---

### `/aad:half-clone`

会話の後半だけ保持してコンテキストを削減します。

**基本使用法**:
```
/aad:half-clone
```

**実行内容**:
1. 会話の後半50%を保持
2. タグ付き新会話作成: `[HALF-CLONE from ...]`
3. トークン使用量約半分

**使用シーン**:
- コンテキストが大きくなりすぎた
- 古い議論は不要
- 新鮮な状態で継続したい

**関連コマンド**: `/aad:clone`, `/aad:handoff`

**参考**: [.claude/commands/aad/half-clone.md](../.claude/commands/aad/half-clone.md)

---

## 品質管理

### `/aad:gate`

各フェーズの完了条件をチェックし、次フェーズへの移行可否を判定します。

**基本使用法**:
```
/aad:gate SPEC      # SPEC品質ゲート
/aad:gate TASKS     # TASKS品質ゲート
/aad:gate TDD       # TDD品質ゲート
/aad:gate REVIEW    # REVIEW品質ゲート
/aad:gate RETRO     # RETRO品質ゲート
/aad:gate MERGE     # MERGE品質ゲート
```

**全フェーズチェック**:
```
/aad:gate --all
```

**厳格モード**:
```
/aad:gate TDD --strict         # カバレッジ90%等
/aad:gate TDD --lenient        # カバレッジ70%等
```

**出力例（TDD）**:
```
品質ゲートチェック: TDD (SPEC-001-T01)

✅ 全テストgreen (25 passed)
✅ カバレッジ85% (目標: 80%以上)
✅ Lint通過 (0 errors)
✅ PR作成完了 (#18)

判定: ✅ 合格

次のフェーズに進めます: /aad:gate REVIEW
```

**各フェーズの完了条件**:

#### SPEC
- [ ] 受け入れ基準がテスト可能
- [ ] MoSCoW設定済み
- [ ] ⚠️ 人間承認必須

#### TASKS
- [ ] 全タスクにID付与
- [ ] 依存関係明記
- [ ] ⚠️ 人間承認必須

#### TDD
- [ ] 全テストgreen
- [ ] カバレッジ80%以上
- [ ] Lint通過

#### REVIEW
- [ ] AI自己レビュー完了
- [ ] CI green
- [ ] ⚠️ 人間承認必須

#### RETRO
- [ ] .aad/retrospectives/にログ作成
- [ ] Keep/Problem/Try記載

#### MERGE
- [ ] mainマージ完了
- [ ] Issue閉鎖
- [ ] worktree削除

**関連コマンド**: `/aad:tasks`, `/aad:worktree`, `/aad:integrate`

**参考**: [.claude/commands/aad/gate.md](../.claude/commands/aad/gate.md)

---

## コマンド実行順序

### 標準フロー（手動）

```
1. /aad:init               # 初期化
2. /aad:gate SPEC          # SPEC承認確認
3. /aad:tasks SPEC-001     # タスク分割
4. /aad:gate TASKS         # タスク承認確認
5. /aad:worktree T01       # ワーカー起動
6. (開発...)           # TDD実装
7. /aad:gate TDD           # 品質確認
8. /aad:gate REVIEW        # レビュー確認
9. /aad:integrate T01      # 統合
10. /aad:retro SPEC-001    # 振り返り
```

### 自動フロー

```
1. /aad:init               # 初期化
2. /aad:gate SPEC          # SPEC承認確認
3. /aad:orchestrate SPEC-001  # 全自動実行
4. /aad:retro SPEC-001     # 振り返り（自動実行済み）
```

### コンテキスト管理フロー

```
定期的:
  /aad:context             # 使用率確認

70%到達時:
  /aad:handoff             # 引き継ぎ作成

新セッション:
  HANDOFF.mdを読み込んで再開
```

---

## Tips

### コマンドのヘルプ

各コマンドの詳細は `.claude/commands/` 配下のMarkdownファイルを参照：

```bash
cat .claude/commands/aad/tasks.md
cat .claude/commands/aad/worktree.md
```

### エイリアスの設定

頻繁に使うコマンドはエイリアスを設定：

```bash
# Claude Code起動時のエイリアス例
/aad:init → プロジェクト初期化
/t → /aad:tasks の短縮形（将来対応予定）
/w → /aad:worktree の短縮形（将来対応予定）
```

### スクリプトとの連携

シェルスクリプトから実行：

```bash
#!/bin/bash
# auto-dev.sh

# SPECを作成
vim .aad/specs/SPEC-001.md

# Claude Codeで自動実行
claude <<EOF
/aad:tasks SPEC-001
/aad:orchestrate SPEC-001
EOF
```

---

## 🔧 ユーティリティスクリプト

`.claude/scripts/` 配下のスクリプト一覧：

### 導入スクリプト

| スクリプト | 説明 | 使用方法 |
|-----------|------|----------|
| `install-to-new.sh` | 新規プロジェクトへの導入 | `.claude/scripts/install-to-new.sh /path/to/new-project` |
| `install-to-existing.sh` | 既存プロジェクトへの導入 | `.claude/scripts/install-to-existing.sh /path/to/existing-project` |

詳細は [SETUP.md](SETUP.md) の導入セクションを参照。

**使用例**:
```bash
# 新規プロジェクト作成
.claude/scripts/install-to-new.sh ~/workspace/my-new-project

# 既存プロジェクトへ導入
.claude/scripts/install-to-existing.sh ~/workspace/existing-project
```

### 内部スクリプト

| スクリプト | 説明 |
|-----------|------|
| `context-bar.sh` | ステータスラインのコンテキスト表示 |
| `detect-default-branch.sh` | デフォルトブランチの自動検出 |
| `clone-conversation.sh` | `/aad:clone` コマンド用 |
| `half-clone-conversation.sh` | `/aad:half-clone` コマンド用 |
| `switch-style.sh` | 出力スタイル切替（sage ↔ standard） |

### switch-style.sh - 出力スタイル切替

AIの出力スタイルを「sage（大賢者）」と「standard（標準）」の間で切り替えます。

**使用方法**:
```bash
.claude/scripts/switch-style.sh standard        # 標準スタイルに変換
.claude/scripts/switch-style.sh sage            # 大賢者スタイルに変換
.claude/scripts/switch-style.sh --current       # 現在のスタイル表示
.claude/scripts/switch-style.sh --list          # 利用可能なスタイル一覧
.claude/scripts/switch-style.sh --list-backups  # バックアップ一覧
.claude/scripts/switch-style.sh --restore [name] # バックアップから復元
.claude/scripts/switch-style.sh --dry-run <style> # 変換プレビュー
```

**スタイル変換マッピング**:

| sage | standard |
|------|----------|
| `成功しました：` | `完了：` |
| `解：` | `結果：` |
| `告：` | `通知：` |
| `否：` | `エラー：` |

**変換対象ファイル**:
- `.claude/commands/aad/*.md`
- `.claude/scripts/context-bar.sh`

**注意**:
- 変換前に自動でバックアップが作成されます
- バックアップは `.claude/styles/backups/` に保存されます
- スクリプトは配置されているプロジェクト専用です

---

## 参考リンク

- [WORKFLOW.md](WORKFLOW.md) - 6フェーズワークフロー詳細
- [QUALITY-GATES.md](QUALITY-GATES.md) - 品質ゲート定義
- [NAMING-CONVENTIONS.md](NAMING-CONVENTIONS.md) - 命名規則
- [SETUP.md](SETUP.md) - 初期セットアップ
- [CUSTOMIZE-CHECKLIST.md](CUSTOMIZE-CHECKLIST.md) - カスタマイズ手順
