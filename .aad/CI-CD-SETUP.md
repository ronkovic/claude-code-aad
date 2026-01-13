# CI/CD セットアップガイド

GitHub Actions を使用した継続的インテグレーション（CI）と継続的デリバリー（CD）の設定方法を説明します。

---

## 📋 目次

- [GitHub Actions の基本](#github-actions-の基本)
- [テスト自動実行](#テスト自動実行)
- [Lint 自動チェック](#lint-自動チェック)
- [カバレッジレポート](#カバレッジレポート)
- [PR チェック](#pr-チェック)
- [デプロイ自動化](#デプロイ自動化)
- [よくある質問](#よくある質問)

---

## GitHub Actions の基本

### ディレクトリ構造

```
.github/
└── workflows/
    ├── ci.yml          # CI パイプライン
    ├── pr-check.yml    # PR チェック
    └── deploy.yml      # デプロイ
```

### 基本的なワークフロー

`.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]  # ⚠️ CLAUDE.md のデフォルトブランチに合わせて変更
  pull_request:
    branches: [main]  # ⚠️ CLAUDE.md のデフォルトブランチに合わせて変更

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run tests
        run: npm test
```

---

## テスト自動実行

### Node.js プロジェクト

`.github/workflows/test.yml`:

```yaml
name: Test

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest

    strategy:
      matrix:
        node-version: [18, 20, 22]

    steps:
      - uses: actions/checkout@v4

      - name: Use Node.js ${{ matrix.node-version }}
        uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run tests
        run: npm test

      - name: Run coverage
        run: npm run test:coverage
```

### TypeScript プロジェクト

```yaml
- name: Type check
  run: npm run type-check

- name: Build
  run: npm run build

- name: Run tests
  run: npm test
```

---

## Lint 自動チェック

`.github/workflows/lint.yml`:

```yaml
name: Lint

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  lint:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run ESLint
        run: npm run lint

      - name: Check formatting (Prettier)
        run: npm run format:check
```

`package.json` にスクリプトを追加：

```json
{
  "scripts": {
    "lint": "eslint . --ext .js,.jsx,.ts,.tsx",
    "format:check": "prettier --check ."
  }
}
```

---

## カバレッジレポート

### Codecov 連携

`.github/workflows/coverage.yml`:

```yaml
name: Coverage

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  coverage:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run tests with coverage
        run: npm run test:coverage

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v4
        with:
          file: ./coverage/lcov.info
          fail_ci_if_error: true
```

### GitHub Actions カバレッジサマリー

```yaml
- name: Generate coverage report
  run: npm run test:coverage

- name: Coverage summary
  run: |
    echo "## Test Coverage" >> $GITHUB_STEP_SUMMARY
    echo "" >> $GITHUB_STEP_SUMMARY
    cat coverage/coverage-summary.json | jq -r '
      .total |
      "| Metric | Coverage |",
      "|--------|----------|",
      "| Lines | \(.lines.pct)% |",
      "| Statements | \(.statements.pct)% |",
      "| Functions | \(.functions.pct)% |",
      "| Branches | \(.branches.pct)% |"
    ' >> $GITHUB_STEP_SUMMARY
```

---

## PR チェック

`.github/workflows/pr-check.yml`:

```yaml
name: PR Check

on:
  pull_request:
    branches: [main]

jobs:
  pr-check:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # 全履歴を取得（差分確認用）

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      # Lint チェック
      - name: Lint
        run: npm run lint

      # Type チェック（TypeScript の場合）
      - name: Type check
        run: npm run type-check

      # テスト実行
      - name: Test
        run: npm test

      # カバレッジチェック
      - name: Coverage
        run: npm run test:coverage

      # ビルドチェック
      - name: Build
        run: npm run build

      # PR コメントにカバレッジを投稿
      - name: Comment coverage
        uses: romeovs/lcov-reporter-action@v0.3.1
        with:
          lcov-file: ./coverage/lcov.info
          github-token: ${{ secrets.GITHUB_TOKEN }}
```

---

## デプロイ自動化

### Vercel デプロイ

`.github/workflows/deploy.yml`:

```yaml
name: Deploy to Vercel

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Deploy to Vercel
        uses: amondnet/vercel-action@v25
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
          vercel-args: '--prod'
```

### AWS S3 + CloudFront デプロイ

```yaml
- name: Build
  run: npm run build

- name: Configure AWS credentials
  uses: aws-actions/configure-aws-credentials@v4
  with:
    aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
    aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
    aws-region: ap-northeast-1

- name: Deploy to S3
  run: |
    aws s3 sync ./dist s3://your-bucket-name --delete

- name: Invalidate CloudFront
  run: |
    aws cloudfront create-invalidation --distribution-id YOUR_DISTRIBUTION_ID --paths "/*"
```

---

## 統合ワークフロー例

`.github/workflows/ci-cd.yml`:

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  # フェーズ 1: コード品質チェック
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Lint
        run: npm run lint

      - name: Type check
        run: npm run type-check

  # フェーズ 2: テスト
  test:
    runs-on: ubuntu-latest
    needs: quality

    strategy:
      matrix:
        node-version: [18, 20]

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js ${{ matrix.node-version }}
        uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run tests
        run: npm test

      - name: Coverage
        run: npm run test:coverage

      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          file: ./coverage/lcov.info

  # フェーズ 3: ビルド
  build:
    runs-on: ubuntu-latest
    needs: test

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Build
        run: npm run build

      - name: Upload build artifacts
        uses: actions/upload-artifact@v4
        with:
          name: build
          path: dist/

  # フェーズ 4: デプロイ（main ブランチのみ）
  deploy:
    runs-on: ubuntu-latest
    needs: build
    if: github.ref == 'refs/heads/main'

    steps:
      - uses: actions/checkout@v4

      - name: Download build artifacts
        uses: actions/download-artifact@v4
        with:
          name: build
          path: dist/

      - name: Deploy
        run: echo "Deploy to production"
        # 実際のデプロイコマンドをここに追加
```

---

## よくある質問

### Q: ワークフローが失敗した場合の通知

A: Slack や Discord に通知を送ることができます：

```yaml
- name: Notify Slack
  if: failure()
  uses: rtCamp/action-slack-notify@v2
  env:
    SLACK_WEBHOOK: ${{ secrets.SLACK_WEBHOOK }}
    SLACK_MESSAGE: 'CI failed on ${{ github.ref }}'
```

### Q: プライベートリポジトリでの利用

A: GitHub Actions は無料枠があります：
- Public: 無制限
- Private: 月2000分まで無料

### Q: キャッシュを活用したい

A: Node.js の依存関係をキャッシュできます：

```yaml
- uses: actions/setup-node@v4
  with:
    node-version: '20'
    cache: 'npm'  # npm のキャッシュを有効化
```

### Q: ワークフローをスキップしたい

A: コミットメッセージに `[skip ci]` を含めます：

```bash
git commit -m "docs: update README [skip ci]"
```

---

## 🔗 参考リンク

- [GitHub Actions 公式ドキュメント](https://docs.github.com/ja/actions)
- [Marketplace (Actions)](https://github.com/marketplace?type=actions)
- [Codecov](https://about.codecov.io/)
- [Vercel](https://vercel.com/)

---

## 📝 次のステップ

1. ✅ `.github/workflows/` ディレクトリを作成
2. ✅ CI ワークフローを追加
3. ✅ PR チェックを設定
4. ⏭️ チーム全体で CI/CD の運用を開始

---

## 🎯 AAD ワークフローとの連携

AAD テンプレートでは、以下のタイミングで CI/CD が実行されます：

1. **PR 作成時** (`gh pr create --draft`)
   - Lint チェック
   - Type チェック
   - テスト実行

2. **PR マージ時** (`/aad:integrate`)
   - 全テスト実行
   - カバレッジレポート
   - 本番デプロイ（オプション）

3. **品質ゲート** (`/aad:gate TDD`)
   - CI ステータスの確認
   - カバレッジ 80% 以上

---

**最終更新**: 2026-01-12
