# Linter セットアップガイド

コードの品質と一貫性を保つため、Linter (ESLint) の導入を推奨します。

---

## 📋 目次

- [ESLint のインストール](#eslint-のインストール)
- [設定ファイルの作成](#設定ファイルの作成)
- [package.json への追加](#packagejson-への追加)
- [VS Code 連携](#vs-code-連携)
- [pre-commit フック](#pre-commit-フック)
- [よくある質問](#よくある質問)

---

## ESLint のインストール

### JavaScript/TypeScript プロジェクト

```bash
# ESLint 本体のインストール
npm install --save-dev eslint

# TypeScript プロジェクトの場合
npm install --save-dev @typescript-eslint/parser @typescript-eslint/eslint-plugin

# React プロジェクトの場合
npm install --save-dev eslint-plugin-react eslint-plugin-react-hooks
```

### 初期化

```bash
npm init @eslint/config
```

対話形式で設定を選択できます：
1. **How would you like to use ESLint?** → To check syntax and find problems
2. **What type of modules does your project use?** → JavaScript modules (import/export)
3. **Which framework does your project use?** → React / None
4. **Does your project use TypeScript?** → Yes / No
5. **Where does your code run?** → Browser / Node

---

## 設定ファイルの作成

### JavaScript プロジェクト (.eslintrc.js)

```javascript
module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
  },
  rules: {
    // 推奨ルール
    'no-console': 'warn',
    'no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    'prefer-const': 'error',
    'no-var': 'error',

    // プロジェクト固有ルール（カスタマイズ可能）
    'indent': ['error', 2],
    'quotes': ['error', 'single'],
    'semi': ['error', 'always'],
  },
};
```

### TypeScript プロジェクト (.eslintrc.js)

```javascript
module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: './tsconfig.json',
  },
  plugins: [
    '@typescript-eslint',
  ],
  rules: {
    // 推奨ルール
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    '@typescript-eslint/no-explicit-any': 'warn',
    '@typescript-eslint/explicit-function-return-type': 'off',

    // プロジェクト固有ルール
    'indent': ['error', 2],
    'quotes': ['error', 'single'],
    'semi': ['error', 'always'],
  },
};
```

### React + TypeScript プロジェクト (.eslintrc.js)

```javascript
module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:react/recommended',
    'plugin:react-hooks/recommended',
    'plugin:@typescript-eslint/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: './tsconfig.json',
  },
  plugins: [
    'react',
    '@typescript-eslint',
  ],
  rules: {
    'react/react-in-jsx-scope': 'off', // React 17+では不要
    'react/prop-types': 'off', // TypeScriptを使用する場合は不要
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
  },
  settings: {
    react: {
      version: 'detect',
    },
  },
};
```

### .eslintignore ファイル

```
# 依存関係
node_modules/

# ビルド成果物
dist/
build/
out/
.next/

# カバレッジ
coverage/

# AAD テンプレート
aad/worktrees/
.aad/retrospectives/
.aad-backup-*/

# その他
*.min.js
*.bundle.js
```

---

## package.json への追加

`package.json` に以下のスクリプトを追加します：

```json
{
  "scripts": {
    "lint": "eslint . --ext .js,.jsx,.ts,.tsx",
    "lint:fix": "eslint . --ext .js,.jsx,.ts,.tsx --fix"
  }
}
```

### 実行方法

```bash
# Lint チェック
npm run lint

# 自動修正
npm run lint:fix
```

---

## VS Code 連携

### ESLint 拡張機能のインストール

1. VS Code の拡張機能パネルを開く（Cmd+Shift+X / Ctrl+Shift+X）
2. "ESLint" を検索
3. Microsoft の ESLint 拡張機能をインストール

### settings.json 設定

`.vscode/settings.json` を作成：

```json
{
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "eslint.validate": [
    "javascript",
    "javascriptreact",
    "typescript",
    "typescriptreact"
  ],
  "editor.formatOnSave": true
}
```

この設定により、ファイル保存時に自動的に ESLint が実行され、修正可能な問題が自動修正されます。

---

## pre-commit フック

コミット前に自動的に Lint チェックを実行するには、`lint-staged` と `husky` を使用します。

### インストール

```bash
npm install --save-dev husky lint-staged

# husky の初期化
npx husky install
npm pkg set scripts.prepare="husky install"
```

### package.json に追加

```json
{
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": [
      "eslint --fix",
      "git add"
    ]
  }
}
```

### pre-commit フックの作成

```bash
npx husky add .husky/pre-commit "npx lint-staged"
```

これにより、コミット前に変更されたファイルに対して自動的に ESLint が実行されます。

---

## よくある質問

### Q: ESLint と Prettier を併用できますか？

A: はい。以下のパッケージをインストールしてください：

```bash
npm install --save-dev prettier eslint-config-prettier eslint-plugin-prettier
```

`.eslintrc.js` の extends に追加：

```javascript
extends: [
  'eslint:recommended',
  'plugin:prettier/recommended', // 最後に追加
],
```

### Q: 特定のファイルやディレクトリを無視したい

A: `.eslintignore` ファイルを使用するか、コメントで無視できます：

```javascript
// ファイル全体を無視
/* eslint-disable */

// 特定の行を無視
// eslint-disable-next-line no-console
console.log('debug');

// 特定のルールを無視
/* eslint-disable no-unused-vars */
```

### Q: 既存プロジェクトに導入する場合

A: 段階的に導入することを推奨します：

1. まず warning のみ有効にする
2. 徐々に error に変更
3. チーム全体で合意を取る

```javascript
rules: {
  'no-console': 'warn', // まず warning から
  // 'no-console': 'error', // 後で error に変更
}
```

### Q: CI/CD で Lint チェックを実行したい

A: [CI-CD-SETUP.md](CI-CD-SETUP.md) を参照してください。

---

## 🔗 参考リンク

- [ESLint 公式ドキュメント](https://eslint.org/docs/latest/)
- [TypeScript ESLint](https://typescript-eslint.io/)
- [eslint-plugin-react](https://github.com/jsx-eslint/eslint-plugin-react)
- [Prettier](https://prettier.io/)

---

## 📝 次のステップ

1. ✅ ESLint をインストール
2. ✅ 設定ファイルを作成
3. ✅ VS Code 連携を設定
4. ⏭️ [CI/CD セットアップ](CI-CD-SETUP.md) に進む

---

**最終更新**: 2026-01-12
