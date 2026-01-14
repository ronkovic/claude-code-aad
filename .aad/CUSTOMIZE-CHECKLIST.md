# カスタマイズチェックリスト

このテンプレートをプロジェクトに合わせてカスタマイズする手順です。

---

## 🎯 カスタマイズレベル

### レベル1: 必須カスタマイズ（5-10分）

最低限必要な設定です。すぐに使い始められます。

### レベル2: 推奨カスタマイズ（30-60分）

プロジェクトの特性に合わせた調整です。より効果的に使えます。

### レベル3: オプションカスタマイズ（1-2時間）

高度な機能や最適化です。チームの成熟度に応じて実施します。

---

## レベル1: 必須カスタマイズ ⚠️

### 1. プロジェクト情報の設定

#### `/aad:init`ウィザードを実行

```bash
claude
/aad:init
```

ウィザードに従って入力：
- [ ] プロジェクト名
- [ ] プロジェクトの目的
- [ ] 技術スタック（言語・フレームワーク）
- [ ] GitHubリポジトリURL

#### 手動で設定する場合

**CLAUDE.md** を編集：

```markdown
## 📋 プロジェクト概要

**プロジェクト名**: my-awesome-app

**目的**: ユーザー管理システムの開発

**開始日**: 2026-01-11

**現在のフェーズ**: SPEC
```

### 2. 認証情報の設定

#### Option A: OAuth Token（Max Plan推奨）

```bash
# ホストマシンで
claude setup-token

# 表示されたトークンをコピー
```

#### Option B: API Key（API利用者）

Anthropic Consoleで生成されたAPIキーを使用します。

**注**: OAuth TokenまたはAPIキーは環境変数として設定するか、Claude Codeの設定で管理します。

### 3. GitHubリポジトリの接続

```bash
# リモートを追加
git remote add origin https://github.com/your-org/your-repo.git

# 初回プッシュ（デフォルトブランチへ）
git add .
git commit -m "chore: initialize from template"
git push -u origin <default-branch>

# GitHub CLI認証
gh auth login

# Issuesを有効化
gh repo edit --enable-issues=true
```

### 4. 動作確認

```bash
# Claude Codeを起動
claude

# コンテキスト確認
/aad:context

# ステータス確認
/aad:status
```

**✅ レベル1完了**: これで基本的な使用が可能です。

---

## レベル2: 推奨カスタマイズ 🔧

### 1. コーディングルールの設定

**CLAUDE.md** の「コーディングルール」セクションを編集：

```markdown
### 命名規則
- **ファイル名**: [プロジェクトの規則]
- **クラス名**: [プロジェクトの規則]
- **関数名**: [プロジェクトの規則]

### コードスタイル
- インデント: [2 or 4スペース]
- 最大行長: [80 or 100 or 120文字]
- セミコロン: [必須 or 不要]
```

### 2. 品質基準の調整

**CLAUDE.md** の「品質ゲート」セクションに追加：

```markdown
## 📊 品質ゲート（プロジェクト固有）

### TDDフェーズ
- カバレッジ: [80% or 85% or 90%]
- Lint: [strict or normal]
- パフォーマンス基準: [具体的な数値]
```

### 3. コミットメッセージ規約

**CLAUDE.md** の「コミットメッセージ規約」を確認・調整：

```markdown
### Type
- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメント
- [プロジェクト固有のtype]

### Scope
- [プロジェクト固有のscope一覧]
```

### 4. テストフレームワークの設定

**package.json** または相当のファイルに追加：

```json
{
  "scripts": {
    "test": "[your-test-command]",
    "test:coverage": "[your-coverage-command]",
    "lint": "[your-lint-command]",
    "type-check": "[your-type-check-command]"
  }
}
```

### 5. CI/CD設定

**.github/workflows/ci.yml** を作成：

```yaml
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
      - uses: actions/setup-node@v4  # または他の環境
        with:
          node-version: '20'
      - run: [your-install-command]
      - run: [your-test-command]
      - run: [your-lint-command]

      # カバレッジチェック
      - name: Check coverage
        run: [your-coverage-check-command]
```

**✅ レベル2完了**: プロジェクトに最適化された状態です。

---

## レベル3: オプションカスタマイズ 🚀

### 1. カスタムスラッシュコマンドの追加

**.claude/commands/** にMarkdownファイルを作成：

```bash
# 例: デプロイコマンド
cat > .claude/commands/deploy.md <<EOF
# デプロイ実行

staging環境またはproduction環境にデプロイします。

## 使用方法

\`\`\`
/deploy staging
/deploy production
\`\`\`

## 実行内容

1. 最新のデフォルトブランチを確認
2. ビルド実行
3. テスト実行
4. デプロイ実行
5. Smoke test実行

[詳細な手順を記載]
EOF
```

### 2. カスタムスクリプトの追加

**.claude/scripts/** にシェルスクリプトを作成：

```bash
# 例: デプロイスクリプト
cat > .claude/scripts/deploy.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

ENV=${1:-staging}

echo "Deploying to $ENV..."
# [デプロイロジック]
EOF

chmod +x .claude/scripts/deploy.sh
```

### 3. プロジェクト固有のスキルの追加

**.claude/skills/** にスキルを作成：

```bash
mkdir -p .claude/skills/my-custom-skill
cat > .claude/skills/my-custom-skill/SKILL.md <<EOF
# My Custom Skill

[スキルの説明と使用方法]
EOF
```

### 4. プロジェクト固有の品質チェック

**CLAUDE.md** に追加：

```markdown
## 🔍 プロジェクト固有のチェック

### パフォーマンス
- ページロード: 2秒以内
- API応答: 500ms以内
- バンドルサイズ: 500KB以下

### アクセシビリティ
- Lighthouse Score: 90以上
- WCAG 2.1 AA準拠

### セキュリティ
- OWASP Top 10チェック
- 依存関係の脆弱性: 0件
```

### 6. エスカレーションルールのカスタマイズ

**CLAUDE.md** の「エスカレーションルール」を調整：

```markdown
## 🚨 エスカレーションルール（プロジェクト固有）

### 🔴 即時エスカレーション
- [プロジェクト固有の条件]
- 例: 決済機能のバグ

### 🟡 警告エスカレーション
- [プロジェクト固有の条件]
- 例: パフォーマンス劣化10%以上

### 🟢 情報エスカレーション
- [プロジェクト固有の条件]
```

### 7. マルチリポジトリ対応

複数のリポジトリを管理する場合：

```bash
# モノレポ構造の場合
my-project/
├── packages/
│   ├── frontend/
│   │   └── .claude/
│   ├── backend/
│   │   └── .claude/
│   └── shared/
└── .claude/  # 共通設定

# 各パッケージで独立したCLAUDE.md
# 共通設定は親ディレクトリで管理
```

### 8. IDE統合（VSCode例）

**.vscode/tasks.json** を作成：

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Claude Code: Init",
      "type": "shell",
      "command": "claude",
      "args": ["-c", "/aad:init"],
      "problemMatcher": []
    },
    {
      "label": "Claude Code: Status",
      "type": "shell",
      "command": "claude",
      "args": ["-c", "/aad:status"],
      "problemMatcher": []
    }
  ]
}
```

**✅ レベル3完了**: 高度にカスタマイズされた環境です。

---

## チーム開発向けカスタマイズ

### 1. チーム固有のルール追加

**CLAUDE.md** に追加：

```markdown
## 👥 チームルール

### コードレビュー
- 必須レビュワー: 2名以上
- レビュー時間: 24時間以内
- 承認後のセルフマージ: 禁止

### コミュニケーション
- 進捗報告: 毎日 [時間]
- ブロッカー報告: 即座に
- 質問: [Slack channel名]
```

### 2. ロール別の設定

**CLAUDE.md** に追加：

```markdown
## 🎭 ロール別ガイド

### ジュニアエンジニア
- タスク: S（Small）のみ担当
- レビュワー: シニアエンジニア必須
- ペアプログラミング推奨

### シニアエンジニア
- タスク: すべてのサイズ
- アーキテクチャ判断権限あり
- ジュニアのメンター役

### テックリード
- SPEC承認権限
- アーキテクチャ決定権限
- 最終レビュー担当
```

### 3. ブランチ戦略の明確化

**CLAUDE.md** に追加：

```markdown
## 🌿 ブランチ戦略

- `<default-branch>`: 本番環境（保護ブランチ） ※通常は main または master
- `develop`: 開発統合（保護ブランチ）
- `feature/*`: 機能開発
- `hotfix/*`: 緊急修正
- `release/*`: リリース準備

### ルール
- デフォルトブランチへの直接プッシュ: 禁止
- PRマージ: Squash merge
- ブランチ削除: 自動
```

---

## カスタマイズ後の確認

### チェックリスト

#### 設定ファイル
- [ ] CLAUDE.md が更新されている
- [ ] .github/workflows/ が設定されている
- [ ] package.json（または相当）が設定されている

#### 動作確認
- [ ] `/aad:init` が正常に実行できる
- [ ] `/aad:context` でコンテキストが表示される
- [ ] `/aad:status` で進捗が表示される
- [ ] GitHub連携が動作する

#### ドキュメント
- [ ] README.md が更新されている
- [ ] CLAUDE.md が自プロジェクト用に書き換えられている
- [ ] チームメンバーと合意されている

### 動作確認コマンド

```bash
# 1. 基本動作
claude
/aad:context
/aad:status

# 2. GitHub連携
gh auth status
gh repo view
gh issue create --title "Test" --body "Test"

# 3. テスト実行
npm test
npm run test:coverage
npm run lint
```

---

## トラブルシューティング

### カスタマイズが反映されない

```bash
# Claude Codeのキャッシュをクリア
rm -rf ~/.claude/cache/

# 再起動
claude
```

### コマンドが認識されない

```bash
# コマンドファイルの確認
ls -la .claude/commands/

# 権限確認
chmod +x .claude/scripts/*.sh

# Claude Code再起動
```

---

## カスタマイズのベストプラクティス

### DO ✅

- **段階的にカスタマイズ**: レベル1→2→3の順で
- **チームと合意**: 特にルールやプロセス
- **ドキュメント更新**: 変更を記録
- **動作確認**: カスタマイズ後は必ずテスト
- **バージョン管理**: 設定ファイルをgitで管理

### DON'T ❌

- **一度に大量変更**: 問題の特定が困難に
- **秘密情報をコミット**: .envファイルは.gitignore
- **テンプレート構造の破壊**: 基本構造は維持
- **ドキュメントの削除**: 参考として残す
- **独自判断で変更**: チームと合意してから

---

## サンプル設定集

### TypeScript + React プロジェクト

```json
// package.json
{
  "scripts": {
    "test": "jest",
    "test:coverage": "jest --coverage",
    "lint": "eslint src/**/*.{ts,tsx}",
    "type-check": "tsc --noEmit"
  }
}
```

### Python + FastAPI プロジェクト

```toml
# pyproject.toml
[tool.pytest.ini_options]
minversion = "6.0"
addopts = "--cov=src --cov-report=json"

[tool.ruff]
select = ["E", "F", "I"]
```

### Go プロジェクト

```makefile
# Makefile
test:
\tgo test ./... -v -cover

lint:
\tgolangci-lint run

coverage:
\tgo test ./... -coverprofile=coverage.out
```

---

## 参考リンク

- [WORKFLOW.md](WORKFLOW.md) - 6フェーズワークフロー
- [COMMANDS.md](COMMANDS.md) - コマンドリファレンス
- [QUALITY-GATES.md](QUALITY-GATES.md) - 品質ゲート定義
- [SETUP.md](SETUP.md) - 初期セットアップ
- [CLAUDE.md](../CLAUDE.md) - プロジェクト指示書

---

## 質問・サポート

困ったときは：

1. このドキュメントを再確認
2. SETUP.mdのトラブルシューティング
3. GitHub Issuesで質問
4. チームメンバーに相談

カスタマイズを楽しんでください！🚀
