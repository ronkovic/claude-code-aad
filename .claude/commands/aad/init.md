# テンプレート初期化ウィザード

## 🔴 重要: 出力指示

初期化完了後、**必ず以下の形式で「次のステップ」を目立つように表示すること**:

### 必須出力フォーマット

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
成功しました：初期化が完了しました。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

次のステップ:

1️⃣ CLAUDE.md を確認
解：プロジェクトのルールブックです。
ゲームの説明書に相当します。

2️⃣ 最初のSPECを作成
cp .aad/templates/SPEC-TEMPLATE.md .aad/specs/SPEC-001.md
解：「何を作るか」の設計図です。
料理のレシピに相当します。

3️⃣ タスクに分割
/aad:tasks SPEC-001
解：大きな仕事を小さく分けます。
パズルのピース分けです。

4️⃣ 開発を開始
/aad:worktree SPEC-001-T01
告：最初のタスクから着手を推奨します。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 理由

- 初期化完了後、ユーザーは次のアクションを知る必要がある
- 大賢者スタイルで一貫した体験を提供する
- ファイル後半の「出力例」だけでは見逃される可能性がある

---

このテンプレートをプロジェクトに合わせてカスタマイズするウィザードです。

## 🔴 重要: 実装指示

このコマンドを実行する際は、**必ず `AskUserQuestion` ツールを使用して**ユーザーに選択肢を提示すること。テキストベースの質問は使用しないこと。

### ウィザードの流れ

#### Step 1: 環境検出
まず以下を検出する:
- `git remote -v` でGitHubリポジトリURL
- `git branch --show-current` でデフォルトブランチ
- `package.json` または `pyproject.toml` から技術スタック
- `README.md` からプロジェクト名

#### Step 2: プロジェクト情報（AskUserQuestion使用）
`AskUserQuestion` で以下を質問（最大4つまで同時に質問可能）:

```json
{
  "questions": [
    {
      "question": "使用する言語・フレームワークを選択してください",
      "header": "技術スタック",
      "options": [
        {"label": "TypeScript + Node.js (推奨)", "description": "Node.js環境でTypeScriptを使用。Webアプリケーション、API、CLIツールなど"},
        {"label": "Python", "description": "Python環境。データ分析、機械学習、Webアプリケーションなど"},
        {"label": "Go", "description": "Go言語環境。高速なバックエンドAPI、CLIツールなど"}
      ],
      "multiSelect": false
    },
    {
      "question": "テストフレームワークを選択してください",
      "header": "テスト",
      "options": [
        {"label": "Jest (推奨)", "description": "JavaScript/TypeScript向けのテストフレームワーク。高速で設定が簡単"},
        {"label": "Vitest", "description": "Vite環境向けのテストフレームワーク。Jest互換でさらに高速"},
        {"label": "pytest", "description": "Python向けのテストフレームワーク。シンプルで強力"},
        {"label": "後で設定する", "description": "今は設定せず、後でセットアップする"}
      ],
      "multiSelect": false
    }
  ]
}
```

#### Step 3: 品質基準（AskUserQuestion使用）
`AskUserQuestion` で以下を質問:

```json
{
  "questions": [
    {
      "question": "Lintツールを設定しますか？",
      "header": "Lint",
      "options": [
        {"label": "ESLint (推奨)", "description": "JavaScript/TypeScript向けのLintツール。業界標準"},
        {"label": "Biome", "description": "高速なLint・Formatter。ESLint + Prettierの代替"},
        {"label": "Ruff", "description": "Python向けの高速Lintツール"},
        {"label": "後で設定する", "description": "今は設定しない"}
      ],
      "multiSelect": false
    },
    {
      "question": "テストカバレッジの目標を設定してください",
      "header": "カバレッジ",
      "options": [
        {"label": "80% (推奨)", "description": "バランスの取れた目標。AADのデフォルト"},
        {"label": "90%", "description": "高い品質基準。より厳格なテスト"},
        {"label": "70%", "description": "柔軟な基準。プロトタイプや実験的プロジェクト向け"},
        {"label": "100%", "description": "完全なカバレッジ。クリティカルなプロジェクト向け"}
      ],
      "multiSelect": false
    }
  ]
}
```

#### Step 4: GitHub連携（AskUserQuestion使用）
検出したGitHubリポジトリがない場合のみ質問:

```json
{
  "questions": [
    {
      "question": "GitHubリポジトリを連携しますか？",
      "header": "GitHub",
      "options": [
        {"label": "後で設定する (推奨)", "description": "ローカルで開発を開始し、後でGitHubリポジトリを作成"},
        {"label": "既存のリポジトリを指定", "description": "既に作成済みのGitHubリポジトリURLを入力"},
        {"label": "連携しない", "description": "GitHubを使用せず、ローカルのみで開発"}
      ],
      "multiSelect": false
    }
  ]
}
```

#### Step 5: 確認（AskUserQuestion使用）
収集した情報を表示し、最終確認:

```json
{
  "questions": [
    {
      "question": "この内容で初期化しますか？",
      "header": "最終確認",
      "options": [
        {"label": "はい、初期化する", "description": "設定を反映してCLAUDE.mdを更新します"},
        {"label": "いいえ、やり直す", "description": "最初からやり直します"}
      ],
      "multiSelect": false
    }
  ]
}
```

#### Step 6: 実行
- CLAUDE.md を更新
- デフォルトブランチをCLAUDE.mdに反映
- 初回コミット作成（AskUserQuestionで確認）

### 注意事項
- **必ず AskUserQuestion を使用すること**
- 1回のAskUserQuestionで最大4つまで質問可能
- multiSelect: false（単一選択）を使用
- label は短く（12文字以内推奨）
- description で詳細を説明

## 実行内容

1. **プロジェクト情報の収集**
   - プロジェクト名
   - 目的
   - 技術スタック

2. **CLAUDE.md のカスタマイズ**
   - プロジェクト概要の記入
   - 技術スタックの設定
   - コーディングルールの調整
   - コミットメッセージ規約の確認

3. **品質基準の設定**
   - カバレッジ目標（デフォルト: 80%）
   - Lint設定
   - CI/CD設定

4. **GitHub連携の設定**
   - リポジトリURL
   - デフォルトブランチ（自動検出 + 確認）
   - Issue/PRテンプレート

5. **初回コミット**
   - 設定を反映してgit commit

## 使用方法

```
/aad:init
```

対話形式でプロジェクトをセットアップします。

## 出力例

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚀 テンプレート初期化ウィザード
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

このウィザードでプロジェクトをセットアップします。

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Step 1/5: プロジェクト情報
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

プロジェクト名を入力してください:
> my-awesome-app

プロジェクトの目的を簡潔に入力してください:
> ユーザー管理システムの開発

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Step 2/5: 技術スタック
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

使用する言語・フレームワークを選択してください:
1. TypeScript + React
2. Python + FastAPI
3. Go
4. その他（手動入力）

> 1

テストフレームワークを選択してください:
1. Jest
2. Vitest
3. pytest
4. その他

> 1

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Step 3/5: 品質基準
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

カバレッジ目標を設定してください (デフォルト: 80%):
> 85

Lintツールを選択してください:
1. ESLint
2. Ruff (Python)
3. golangci-lint
4. その他

> 1

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Step 4/5: GitHub連携
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

GitHubリポジトリURLを入力してください:
> https://github.com/myorg/my-awesome-app

デフォルトブランチを自動検出しています...
検出結果: main

このブランチを使用しますか？ (y/n または別のブランチ名を入力):
> y

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Step 5/5: 確認
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

以下の内容で初期化します:

プロジェクト名: my-awesome-app
目的: ユーザー管理システムの開発
技術スタック: TypeScript + React
テストフレームワーク: Jest
カバレッジ目標: 85%
Lintツール: ESLint
GitHubリポジトリ: https://github.com/myorg/my-awesome-app
デフォルトブランチ: main

この内容で初期化しますか？ (y/n)
> y

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⚙️  初期化中...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ CLAUDE.md を更新しました（デフォルトブランチ: main）
✅ .github/workflows/ にCI設定を追加しました
✅ README.md のリポジトリURLを更新しました

初回コミットを作成しますか？ (y/n)
> y

✅ 初回コミットを作成しました

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🎉 初期化完了！
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

成功しました：初期化が完了しました。

次のステップ:

1️⃣ CLAUDE.md を確認
解：プロジェクトのルールブックです。
ゲームの説明書に相当します。

2️⃣ 最初のSPECを作成
cp .aad/templates/SPEC-TEMPLATE.md .aad/specs/SPEC-001.md
解：「何を作るか」の設計図です。
料理のレシピに相当します。

3️⃣ タスクに分割
/aad:tasks SPEC-001
解：大きな仕事を小さく分けます。
パズルのピース分けです。

4️⃣ 開発を開始
/aad:worktree SPEC-001-T01
告：最初のタスクから着手を推奨します。
```

## 設定項目

### プロジェクト情報
- プロジェクト名
- 目的
- 開始日（自動設定）

### 技術スタック
- 言語・フレームワーク
- テストフレームワーク
- Lintツール
- Formatter

### 品質基準
- カバレッジ目標（70-100%）
- Lint設定
- Type check有無
- パフォーマンス基準

### GitHub連携
- リポジトリURL
- デフォルトブランチ
- Issue/PRラベル
- マイルストーン設定

### コーディングルール
- 命名規則
- コードスタイル
- コメント規約
- ディレクトリ構造

## スキップモード

デフォルト値で初期化：

```
/aad:init --quick
```

## 再初期化

設定を変更したい場合：

```
/aad:init --reconfigure
```

## 設定のエクスポート/インポート

設定を保存：

```
/aad:init --export=project-config.json
```

設定を読み込み：

```
/aad:init --import=project-config.json
```

## 関連コマンド

- `/aad:context` - コンテキスト確認
- `/aad:tasks` - タスク分割
- `/aad:status` - 進捗確認

## カスタマイズチェックリスト

初期化後に確認すべき項目：

- [ ] CLAUDE.mdのプロジェクト概要を確認
- [ ] コーディングルールをチームと合意
- [ ] 品質基準が適切か確認
- [ ] GitHub Actionsの設定を確認
- [ ] 最初のSPECファイルを作成

詳細は [CUSTOMIZE-CHECKLIST.md](../../../.aad/CUSTOMIZE-CHECKLIST.md) を参照してください。

## 注意事項

- 初期化は一度だけ実行してください（再設定は `--reconfigure`）
- プロジェクトに合わせて柔軟にカスタマイズできます
- デフォルト値は汎用的な設定ですが、必ず確認してください
- 初回コミット前にすべての設定を確認してください
- チーム開発の場合は、チーム全体で設定を合意してください
