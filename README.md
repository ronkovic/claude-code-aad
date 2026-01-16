# 自律型AI駆動開発テンプレート

**Autonomous AI-Driven Development (AAD) Template**

Claude Codeを使用した自律型AI駆動開発のためのテンプレートです。

## 🎯 特徴

- **SPEC駆動開発**: MoSCoW形式の仕様書からタスクを自動分割
- **マルチワーカー**: Git worktreeによる並列開発（元フォルダに影響なし）
- **品質ゲート**: カバレッジ80%、人間承認必須（SPEC/TASKS/REVIEWフェーズ）
- **コンテキスト管理**: 70%ルールで自動警告・ハンドオフ推奨
- **GitHub Issues連携**: タスク管理との統合

## 🚀 クイックスタート

### 前提条件

- Claude Code（Max PlanまたはAPI）
- Git
- GitHub CLI（`gh`）

### セットアップ方法

新規プロジェクトにAADテンプレートを導入する方法は2つあります：

#### 方法1: スクリプトで新規作成（推奨）

```bash
# テンプレートをクローン
git clone <your-repo-url> /tmp/aad-template

# スクリプトで新規プロジェクトを作成
/tmp/aad-template/.claude/scripts/install-to-new.sh ~/workspace/my-new-project

# 作成されたプロジェクトに移動
cd ~/workspace/my-new-project

# Claude Codeで初期化
claude
/aad:init
```

#### 方法2: 手動でクローン

```bash
# このテンプレートをクローンまたはダウンロード
git clone <your-repo-url>
cd terminal-claude-code-demo

# Claude Codeを起動
claude

# 初期化ウィザードを実行
/aad:init
```

### 基本的な開発フロー

```bash
# 1. 仕様書を作成
# .aad/specs/SPEC-001.md を作成（SPEC-TEMPLATEを参照）

# 2. タスクに分割
/aad:tasks SPEC-001

# 3. ワーカーを起動（手動モード）
/aad:worktree SPEC-001-T01

# 4. 進捗確認
/aad:status

# 5. 統合
/aad:integrate SPEC-001-T01

# 6. 振り返り
/aad:retro
```

## 📁 ディレクトリ構造

```
.
├── README.md                        # このファイル
├── CLAUDE.md                        # プロジェクト指示書（学びを蓄積）
├── HANDOFF.md                       # セッション間引き継ぎテンプレート
├── .aad/                            # AADメインディレクトリ
│   ├── specs/                       # 仕様書（MoSCoW形式）
│   ├── tasks/                       # タスク定義（GitHub Issues連携）
│   ├── retrospectives/              # 振り返りログ
│   ├── templates/                   # テンプレートファイル
│   ├── worktrees/                   # 並列開発用（自動生成）
│   ├── progress/                    # オーケストレーション状態管理
│   ├── WORKFLOW.md                  # 6フェーズワークフロー
│   ├── SETUP.md                     # セットアップガイド
│   ├── COMMANDS.md                  # コマンドリファレンス
│   ├── QUALITY-GATES.md             # 品質ゲート定義
│   ├── NAMING-CONVENTIONS.md        # 命名規則
│   ├── LINTER-SETUP.md              # Linterセットアップ
│   ├── CI-CD-SETUP.md               # CI/CDセットアップ
│   └── CUSTOMIZE-CHECKLIST.md       # カスタマイズ手順
└── .claude/                         # Claude Code設定
    ├── commands/aad/                # 12個のスラッシュコマンド
    ├── scripts/                     # シェルスクリプト
    └── settings.json                # ステータスライン設定
```

## 🔧 既存プロジェクトへの導入

既存のプロジェクトにAADテンプレートを導入する場合、差分確認・追記方式のスクリプトを使用できます。

```bash
# 方法1: テンプレートから直接実行
/path/to/template/.claude/scripts/install-to-existing.sh /path/to/your-project

# 方法2: スクリプトをコピーして実行
cp /path/to/template/.claude/scripts/install-to-existing.sh /tmp/
cd /path/to/your-project
/tmp/install-to-existing.sh .
```

**スクリプトの動作**:
- 既存ファイル/フォルダの有無をチェック
- 既存ファイルを上書きせず、必要な部分のみ追記
- 自動的にバックアップを作成（`.aad-backup-YYYYMMDDHHMMSS/`）
- 対話形式でユーザーに確認を求める

**衝突回避の仕組み**:
| ファイル | 既存時の処理 |
|----------|-------------|
| `README.md` | コピーしない（既存を維持） |
| `.gitignore` | AADエントリを追記 |
| `CLAUDE.md` | AADセクションを追記 |
| `docs/` | `.aad/` サブフォルダとして配置 |
| `.claude/` | `commands/aad/` と `scripts/` をマージ |

詳細は [.aad/SETUP.md](./.aad/SETUP.md) の「既存プロジェクトへの導入」を参照してください。

## 📝 利用可能なコマンド

| コマンド | 説明 |
|----------|------|
| `/aad:init` | テンプレート初期化ウィザード |
| `/aad:tasks SPEC-ID` | 仕様書からタスクを分割 + GitHub Issues作成 |
| `/aad:worktree TASK-ID` | worktree + ブランチ作成 |
| `/aad:status` | 全タスク/ワーカー進捗表示 |
| `/aad:integrate TASK-ID` | PRマージ + worktree削除 |
| `/aad:orchestrate SPEC-ID` | 全自動オーケストレーション |
| `/aad:gate PHASE` | 品質ゲートチェック |
| `/aad:context` | コンテキスト使用状況確認 |
| `/aad:handoff` | 引き継ぎドキュメント作成 |
| `/aad:retro` | 振り返り実行 |
| `/aad:clone` | 会話クローン |
| `/aad:half-clone` | ハーフクローン（コンテキスト削減） |

詳細は[COMMANDS.md](.aad/COMMANDS.md)を参照してください。

## 🔄 6フェーズワークフロー

1. **SPEC**: 仕様書作成（MoSCoW形式） → 人間承認必須
2. **TASKS**: タスク分割 + GitHub Issues作成 → 人間承認必須
3. **TDD**: テスト駆動開発（カバレッジ80%以上）
4. **REVIEW**: コードレビュー + CI → 人間承認必須
5. **RETRO**: 振り返りログ作成 + CLAUDE.md更新
6. **MERGE**: mainマージ + Issue閉鎖 + worktree削除

詳細は[WORKFLOW.md](.aad/WORKFLOW.md)を参照してください。

## 🎭 出力スタイル切替

AIの出力スタイルを切り替えることができます。

| スタイル | 説明 |
|----------|------|
| `standard` | 標準スタイル（デフォルト）：「完了：」「結果：」「通知：」「エラー：」 |
| `sage` | 大賢者風：「成功しました：」「解：」「告：」「否：」 |

### 使用方法

```bash
# スタイル切替
.claude/scripts/switch-style.sh standard   # 標準スタイルに変換
.claude/scripts/switch-style.sh sage       # 大賢者スタイルに変換

# 確認・管理
.claude/scripts/switch-style.sh --current       # 現在のスタイル表示
.claude/scripts/switch-style.sh --list-backups  # バックアップ一覧
.claude/scripts/switch-style.sh --restore       # 最新バックアップから復元
.claude/scripts/switch-style.sh --cleanup       # 古いバックアップを削除
```

詳細は [COMMANDS.md](.aad/COMMANDS.md#switch-stylesh---出力スタイル切替) を参照してください。

## 🎨 カスタマイズ

このテンプレートは汎用的に設計されています。プロジェクトに合わせてカスタマイズする手順は[CUSTOMIZE-CHECKLIST.md](.aad/CUSTOMIZE-CHECKLIST.md)を参照してください。

## 📚 詳細ドキュメント

- [WORKFLOW.md](.aad/WORKFLOW.md) - 6フェーズワークフロー詳細
- [SETUP.md](.aad/SETUP.md) - 初期セットアップガイド
- [COMMANDS.md](.aad/COMMANDS.md) - コマンドリファレンス
- [QUALITY-GATES.md](.aad/QUALITY-GATES.md) - 品質ゲート定義
- [CUSTOMIZE-CHECKLIST.md](.aad/CUSTOMIZE-CHECKLIST.md) - カスタマイズ手順

## 🔗 参考

このテンプレートは以下のリソースに基づいています：

- [ykdojo/claude-code-tips](https://github.com/ykdojo/claude-code-tips)
- AI・SPEC駆動開発ワークフロー案
- Claude Code Tips - 詳細ガイド

## 📄 ライセンス

MIT License

## 🤝 コントリビューション

Issue・PRを歓迎します。

---

**作成日**: 2026年1月11日
**対応Claude Codeバージョン**: 2.1.1+
**推奨認証方式**: CLAUDE_CODE_OAUTH_TOKEN（Max Plan）
