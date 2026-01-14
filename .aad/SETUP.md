# セットアップガイド

このテンプレートを使い始めるための詳細なセットアップ手順です。

---

## 📋 前提条件

### 必須

- **Claude Code**: v2.1.1以上
  ```bash
  npm install -g @anthropic-ai/claude-code@latest
  ```

- **Git**: 2.25以上（worktree機能）
  ```bash
  git --version
  ```

- **Node.js**: 20.x以上
  ```bash
  node --version
  ```

- **GitHub CLI**: 2.x以上
  ```bash
  gh --version
  gh auth login  # 初回のみ
  ```

### 推奨

- **tmux**: 3.x以上（セッション管理用）
  ```bash
  tmux -V
  ```

---

## 🚀 クイックセットアップ（5分）

### 1. テンプレートの取得

新規プロジェクトを作成する方法は2つあります：

#### オプションA: スクリプトで作成（推奨）

```bash
# テンプレートをクローン
git clone <このテンプレートのURL> /tmp/aad-template

# スクリプトで新規プロジェクトを作成
/tmp/aad-template/.claude/scripts/install-to-new.sh ~/workspace/my-new-project

# 作成されたプロジェクトに移動
cd ~/workspace/my-new-project
```

このスクリプトは以下を自動で実行します：
- テンプレートファイルのコピー
- Git初期化（未初期化の場合）
- デフォルトブランチの検出
- 初回コミットの作成（オプション）

#### オプションB: 手動でコピー

```bash
# このテンプレートをクローン
git clone <このテンプレートのURL>
cd terminal-claude-code-demo

# または、新規プロジェクトとして使用
cp -r terminal-claude-code-demo my-new-project
cd my-new-project
rm -rf .git
git init
git add .
git commit -m "chore: initial commit from template"
```

### 2. 初期化

```bash
# Claude Codeを起動
claude

# プロンプトで初期化ウィザードを実行
/aad:init
```

ウィザードに従ってプロジェクト情報を入力します。

### 3. 最初のSPECを作成

```bash
# テンプレートをコピー
cp .aad/templates/SPEC-TEMPLATE.md .aad/specs/SPEC-001.md

# 仕様書を編集
vim .aad/specs/SPEC-001.md  # or code, nano, etc.
```

### 4. 開発開始

```bash
# Claude Codeで
/aad:tasks SPEC-001
/aad:worktree SPEC-001-T01

# または全自動
/aad:orchestrate SPEC-001
```

---

## 🔧 既存プロジェクトへの導入

既存のプロジェクトにAADテンプレートを導入する場合、専用の導入スクリプトを使用します。

### 導入スクリプトの実行

```bash
# 方法1: テンプレートから直接実行
/path/to/template/.claude/scripts/install-to-existing.sh /path/to/your-project

# 方法2: スクリプトをコピーして実行
cp /path/to/template/.claude/scripts/install-to-existing.sh /tmp/
cd /path/to/your-project
/tmp/install-to-existing.sh .
```

### スクリプトの動作フロー

1. **差分確認フェーズ**
   ```
   📋 既存ファイルをチェック中...
     ⚠️  CLAUDE.md が存在します
     ⚠️  .gitignore が存在します
     ⚠️  docs が存在します
     ✅ .claude は新規作成されます

   続行しますか？ (y/n)
   ```

2. **バックアップフェーズ**
   ```
   📦 バックアップを作成: .aad-backup-20260112001234
   ```

3. **導入フェーズ**
   - 既存ファイルを上書きせず、必要な部分のみ追記
   - 衝突するファイルは別名で配置

### ファイル別の処理

| ファイル/フォルダ | 既存時の処理 |
|------------------|-------------|
| `CLAUDE.md` | AAD必須セクションを末尾に追記 |
| `.gitignore` | AADエントリを追記（重複チェック） |
| `.claude/` | `commands/aad/` と `scripts/` をマージ |
| `docs/` | `.aad/` として配置 |
| `.aad/templates/` | 全テンプレート（SPEC/TASK/RETRO/TEMPLATE.md）を配置 |
| `.aad/specs/` | ディレクトリ作成のみ（テンプレートは`.aad/templates/`） |
| `.aad/tasks/` | ディレクトリ作成のみ（テンプレートは`.aad/templates/`） |
| `.aad/retrospectives/` | ディレクトリ作成のみ（テンプレートは`.aad/templates/`） |
| `.aad/worktrees/` | ディレクトリ作成のみ |
| `HANDOFF.md` | 新規作成（既存なら警告） |
| `README.md` | **コピーしない**（既存を維持） |

### 導入後の確認

```bash
# 既存ファイルが保持されているか
cat README.md  # 変更なし

# AADエントリが追記されているか
tail -n 10 .gitignore

# aadコマンドが追加されているか
ls .claude/commands/aad/

# .aad/ ディレクトリが作成されているか
ls .aad/
```

### バックアップからの復元

問題が発生した場合、バックアップから復元できます：

```bash
# バックアップフォルダを確認
ls -la .aad-backup-*

# 特定のファイルを復元
cp .aad-backup-YYYYMMDDHHMMSS/CLAUDE.md .

# 全体を復元（慎重に）
cp -r .aad-backup-YYYYMMDDHHMMSS/* .
```

---

## 🔧 詳細セットアップ

### ステップ1: 認証設定

#### オプションA: OAuth Token（Max Plan - 推奨）

```bash
# ホストマシンでトークン取得
claude setup-token

# 表示されたトークンをコピー
# 出力例: sk-ant-oat01-XXXXXXXXXX
```

#### オプションB: API Key（API利用者）

1. https://console.anthropic.com/ にアクセス
2. "API Keys" から新しいキーを作成
3. キーをコピー（sk-ant-api-XXXXXXXXXX）

### ステップ2: GitHubリポジトリ設定

```bash
# 1. GitHubでリポジトリを作成
gh repo create my-project --public

# 2. リモートを追加
git remote add origin https://github.com/your-org/my-project.git

# 3. 初回プッシュ（デフォルトブランチへ）
git push -u origin <default-branch>

# 4. Issuesを有効化
gh repo edit --enable-issues=true
```

### ステップ3: プロジェクトローカル設定

#### .claude/settings.json の確認

既に設定済みですが、カスタマイズする場合：

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/scripts/context-bar.sh"
  },
  "model": "claude-sonnet-4-5",
  "maxTokens": 200000
}
```

#### CLAUDE.md のカスタマイズ

プロジェクト固有の設定を記入：

- プロジェクト名・目的
- 技術スタック
- コーディングルール
- コミットメッセージ規約

### ステップ4: CI/CD設定（オプション）

GitHub Actionsの設定例：

```yaml
# .github/workflows/ci.yml
name: CI

on:
  pull_request:
    branches: [main]  # ⚠️ CLAUDE.mdのデフォルトブランチに合わせて変更してください
  push:
    branches: [main]  # ⚠️ CLAUDE.mdのデフォルトブランチに合わせて変更してください

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm ci
      - run: npm test
      - run: npm run lint
      - run: npm run test:coverage

      # カバレッジチェック
      - name: Check coverage
        run: |
          COVERAGE=$(jq '.total.lines.pct' coverage/coverage-summary.json)
          if (( $(echo "$COVERAGE < 80" | bc -l) )); then
            echo "Coverage $COVERAGE% is below 80%"
            exit 1
          fi
```

---

## 🎨 プロジェクトタイプ別セットアップ

### TypeScript + React

```bash
# package.json に追加
{
  "scripts": {
    "test": "jest",
    "test:coverage": "jest --coverage",
    "lint": "eslint src/**/*.{ts,tsx}",
    "type-check": "tsc --noEmit"
  }
}

# .eslintrc.js
module.exports = {
  extends: ['react-app', 'react-app/jest'],
  rules: {
    // プロジェクト固有のルール
  }
};
```

### Python + FastAPI

```bash
# pyproject.toml に追加
[tool.pytest.ini_options]
minversion = "6.0"
addopts = "--cov=src --cov-report=json --cov-fail-under=80"

[tool.ruff]
select = ["E", "F", "I"]
ignore = []
```

### Go

```bash
# Makefile
test:
\tgo test ./... -v -cover -coverprofile=coverage.out

lint:
\tgolangci-lint run

coverage:
\tgo tool cover -func=coverage.out
```

---

## 🔐 認証のベストプラクティス

### Max Planユーザー

1. **OAuth Token使用を推奨**
   - 毎回の認証不要
   - CI/CD対応

2. **トークンの管理**
   ```bash
   # .envファイルに保存
   echo "CLAUDE_CODE_OAUTH_TOKEN=sk-ant-oat01-XXX" > ~/.claude-oauth.env

   # 権限を制限
   chmod 600 ~/.claude-oauth.env
   ```

3. **トークンの更新**
   - 定期的に再取得を推奨（3ヶ月ごと）
   - 漏洩の疑いがある場合は即座に再発行

### API利用者

1. **APIキーの管理**
   ```bash
   # 環境変数に設定
   export ANTHROPIC_API_KEY="sk-ant-api-XXX"

   # またはファイルで管理
   echo "ANTHROPIC_API_KEY=sk-ant-api-XXX" > ~/.claude-api.env
   chmod 600 ~/.claude-api.env
   ```

2. **使用量の監視**
   - https://console.anthropic.com/ で使用量を確認
   - アラート設定を推奨

---

## 📁 ディレクトリ構造の確認

セットアップ完了後、以下のディレクトリ構造になっているはずです：

```bash
find . -type d -maxdepth 2 | sort

# 期待される出力:
# .
# ./.aad
# ./.aad/progress
# ./.aad/specs
# ./.aad/tasks
# ./.aad/templates
# ./.claude
# ./.claude/commands
# ./.claude/scripts
# ./.git
```

---

## 🧪 動作確認

### 1. コマンド確認

```bash
claude

# プロンプトで
/aad:context     # コンテキスト確認
/aad:init --help # ヘルプ表示
```

### 2. スクリプト確認

```bash
# ステータスバー動作確認
.claude/scripts/context-bar.sh

# 期待される出力（mainはデフォルトブランチ名）:
# Sonnet 4.5 | 📁project-name | 🔀main | ░░░░░░░░░░ 0%
```

### 3. GitHub連携確認

```bash
# 認証状態確認
gh auth status

# リポジトリ情報確認
gh repo view

# Issue作成テスト
gh issue create --title "Test" --body "Test issue"
```

---

## 🚨 トラブルシューティング

### Claude Codeが起動しない

```bash
# バージョン確認
claude --version

# 再インストール
npm uninstall -g @anthropic-ai/claude-code
npm install -g @anthropic-ai/claude-code@latest

# キャッシュクリア
rm -rf ~/.claude/cache/
```

### GitHub CLIが動作しない

```bash
# 再認証
gh auth logout
gh auth login

# 権限確認
gh auth status

# 必要なスコープ: repo, workflow
```

### worktreeが作成できない

```bash
# Gitバージョン確認（2.25以上必要）
git --version

# 既存worktreeを確認
git worktree list

# 不要なworktreeを削除
git worktree remove ../old-worktree
git worktree prune
```

---

## 🔄 アップグレード

### テンプレートの更新

```bash
# 元のテンプレートをremoteに追加
git remote add template <template-repo-url>

# 最新を取得
git fetch template

# 特定のファイルのみマージ（mainはテンプレートリポジトリのデフォルトブランチ）
git checkout template/<template-default-branch> -- docs/WORKFLOW.md

# または全体マージ
git merge template/<template-default-branch>
```

### Claude Codeの更新

```bash
# 最新版にアップグレード
npm update -g @anthropic-ai/claude-code

# バージョン確認
claude --version
```

---

## 📚 次のステップ

セットアップ完了後：

1. [WORKFLOW.md](WORKFLOW.md) を読んで開発フローを理解
2. [COMMANDS.md](COMMANDS.md) でコマンドを確認
3. 最初のSPECを作成して開発開始
4. [CUSTOMIZE-CHECKLIST.md](CUSTOMIZE-CHECKLIST.md) でカスタマイズ

---

## 💬 ヘルプ

困ったときは：

- [GitHub Issues](https://github.com/your-org/your-repo/issues)
- [CLAUDE.md](../CLAUDE.md) のプロジェクトルールを確認
- `/aad:context` でコンテキスト状況を確認
- `/aad:handoff` で現在の状態を記録
