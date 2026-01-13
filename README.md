# 自律型AI駆動開発テンプレート

**Autonomous AI-Driven Development (AAD) Template**

Claude Codeを使用した自律型AI駆動開発のためのテンプレートです。

## 🎯 特徴

- **SPEC駆動開発**: MoSCoW形式の仕様書からタスクを自動分割
- **高自律実行**: Docker隔離環境で`--dangerously-skip-permissions`を使用
- **マルチワーカー**: Git worktreeによる並列開発（元フォルダに影響なし）
- **品質ゲート**: カバレッジ80%、人間承認必須（SPEC/TASKS/REVIEWフェーズ）
- **コンテキスト管理**: 70%ルールで自動警告・ハンドオフ推奨
- **GitHub Issues連携**: タスク管理との統合

## 🚀 クイックスタート

### 前提条件

- Claude Code（Max PlanまたはAPI）
- Docker Desktop
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

### 認証設定（Docker使用時）

```bash
# OAuth Tokenを取得（Max Plan推奨）
claude setup-token

# .envファイルを作成
cp .aad/container/.env.example .aad/container/.env
# CLAUDE_CODE_OAUTH_TOKEN を設定
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
├── .aad/specs/                           # 仕様書（MoSCoW形式）
├── tasks/                           # タスク定義（GitHub Issues連携）
├── .aad/retrospectives/                  # 振り返りログ
├── .claude/                         # Claude Code設定
│   ├── commands/                    # 11個のスラッシュコマンド
│   ├── scripts/                     # シェルスクリプト
│   └── settings.json                # ステータスライン設定
├── .aad/container/                       # Docker隔離環境
│   ├── Dockerfile
│   ├── docker-compose.yml           # マルチワーカー構成
│   ├── setup.sh
│   └── .env.example
├── .aad/worktrees/                       # 並列開発用（自動生成）
└── docs/                            # 詳細ドキュメント
    ├── WORKFLOW.md                  # 6フェーズワークフロー
    ├── SETUP.md                     # セットアップガイド
    ├── COMMANDS.md                  # コマンドリファレンス
    ├── QUALITY-GATES.md             # 品質ゲート定義
    └── CUSTOMIZE-CHECKLIST.md       # カスタマイズ手順
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

## 🐳 Docker隔離環境

長時間・リスクタスクを安全に実行：

```bash
cd container

# シングルワーカー（手動実行）
docker build -t autonomous-dev .
docker run -it -e CLAUDE_CODE_OAUTH_TOKEN="xxx" autonomous-dev

# マルチワーカー（自動オーケストレーション）
docker-compose up -d
```

### Docker環境でのGit Worktree使用

⚠️ **重要**: Git worktreeをDocker環境で使用する場合、**同一パスでマウント**が必要です。

```bash
# .envファイルで設定（推奨）
echo 'HOST_PROJECT_PATH=/path/to/your/project' >> .aad/container/.env

# または環境変数で指定
HOST_PROJECT_PATH=/path/to/your/project docker-compose up
```

**理由**: worktreeの`.git`ファイルはホストの絶対パスを参照するため、コンテナ内で同じパスにマウントしないとgit操作が失敗します。

詳細は [.aad/container/README.md](.aad/container/README.md) を参照してください。

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
