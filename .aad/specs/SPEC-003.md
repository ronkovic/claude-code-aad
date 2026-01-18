# SPEC-003: CLI基本コマンド

**作成日**: 2026-01-18

**担当者**: Claude Code

**ステータス**: Approved

**関連Issue**: N/A

---

## 📋 概要

基本的な CLI コマンドを実装し、ユーザーがコマンドラインから操作できるようにする。clap ライブラリを使用したサブコマンド構造を設計し、init、spec、tasks、style、worktree コマンドを提供する。依存性注入の仕組みを構築し、テスト可能な設計を実現する。

---

## 🎯 目的

### ビジネス目標
ユーザーフレンドリーなCLIインターフェースを提供し、開発ワークフローの各フェーズをコマンドラインから簡単に操作できるようにする。Git worktreeやスタイル変換など、開発者の生産性を向上させる機能を統合する。

### ユーザーストーリー
```
As a 開発者
I want to コマンドラインから簡単に仕様やタスクを管理する
So that 効率的な開発ワークフローを実現できる
```

---

## 🔍 要件定義（MoSCoW）

### Must Have（必須）
これらがないとリリースできない機能

- [ ] **REQ-1: cli クレート作成** - `crates/cli/` ディレクトリに cli クレートを作成し、CLI 基盤を提供する。`clap` クレートをderive feature有効で依存に含める。
- [ ] **REQ-2: clap によるコマンド定義** - `clap` を使用してサブコマンド構造を設計し、各コマンドの引数・オプションを定義する。`aad --help` で適切なヘルプメッセージを表示する。
- [ ] **REQ-3: DI コンテナ実装** - 依存性注入の仕組みを構築し、リポジトリ実装を切り替え可能にする。テスト時にモックリポジトリへの切り替えが可能である。
- [ ] **REQ-4: init コマンド実装** - `aad init` でプロジェクトを初期化し、`.aad/` ディレクトリとテンプレートファイルを配置する。既存プロジェクトでは上書き確認を表示する。
- [ ] **REQ-5: spec コマンド実装** - `aad spec <spec-id>` で仕様ファイルを作成し、受け入れ基準のテンプレートを生成する。MoSCoW 優先度セクションを含める。
- [ ] **REQ-6: tasks コマンド実装** - `aad tasks <spec-id>` でタスクを分割し、タスクIDを自動採番する（SPEC-XXX-TXX形式）。`--github` オプションでGitHub Issuesを作成する。
- [ ] **REQ-7: style コマンド実装** - `aad style list` でスタイル一覧を表示し、`aad style apply <style-name>` でスタイルを `CLAUDE.md` に適用する。トークン置換が正しく動作する。
- [ ] **REQ-8: worktree コマンド実装** - `aad worktree <spec-id>` で Git worktree とブランチを作成する。`../aad-SPEC-XXX/` に worktree を配置し、ブランチ名は `feature/SPEC-XXX` とする。

### Should Have（重要）
できるだけ含めるべき機能

- なし

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- なし

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] **対話的ウィザード** - 理由: 後続フェーズで実装予定
- [ ] **コマンド補完スクリプト** - 理由: 優先度が低い

---

## 🎨 UI/UX要件

### 画面構成
コマンドラインインターフェース

### 主要な操作フロー

#### 1. プロジェクト初期化
```bash
$ aad init
✓ .aad/ ディレクトリを作成しました
✓ テンプレートファイルを配置しました
✓ プロジェクトの初期化が完了しました
```

#### 2. 仕様作成
```bash
$ aad spec SPEC-001
✓ .aad/specs/SPEC-001.md を作成しました
```

#### 3. タスク分割
```bash
$ aad tasks SPEC-001 --github
✓ .aad/tasks/SPEC-001-tasks.md を作成しました
✓ GitHub Issues を作成しました (SPEC-001-T01, SPEC-001-T02, ...)
```

#### 4. worktree作成
```bash
$ aad worktree SPEC-001
✓ ../aad-SPEC-001/ に worktree を作成しました
✓ ブランチ feature/SPEC-001 を作成しました
```

### レスポンシブ対応
N/A

---

## 🔧 技術要件

### フロントエンド
N/A

### バックエンド
- **言語**: Rust (Edition 2021)
- **CLIフレームワーク**: clap (derive feature有効)
- **テンプレートエンジン**: handlebars
- **Git操作**: git2 クレートまたは std::process::Command
- **DI**: 手動実装（構造体ベース）

### データベース
N/A

### パフォーマンス要件
- コマンド起動時間: 500ms以内
- ヘルプメッセージ表示: 100ms以内

---

## 📊 データモデル

### CLI構造

#### `Cli` 構造体
メインコマンド

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| command | Commands | サブコマンド |

#### `Commands` enum
サブコマンド一覧

- `Init`: プロジェクト初期化
- `Spec { spec_id: String }`: 仕様作成
- `Tasks { spec_id: String, github: bool }`: タスク分割
- `Style { action: StyleAction }`: スタイル操作
- `Worktree { spec_id: String }`: worktree作成

#### `App` 構造体（DI コンテナ）

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| spec_repository | Box<dyn SpecRepository> | 仕様リポジトリ |
| task_repository | Box<dyn TaskRepository> | タスクリポジトリ |
| config | AadConfig | アプリケーション設定 |

---

## 🔗 API仕様

### コマンドライン API

#### init コマンド
```bash
aad init
```
プロジェクトを初期化し、`.aad/` ディレクトリとテンプレートファイルを配置する。

#### spec コマンド
```bash
aad spec <SPEC-ID>
```
仕様ファイルを作成する。

**引数**:
- `SPEC-ID`: 仕様ID（例: SPEC-001）

#### tasks コマンド
```bash
aad tasks <SPEC-ID> [--github]
```
タスクを分割し、タスクファイルを作成する。

**引数**:
- `SPEC-ID`: 仕様ID

**オプション**:
- `--github`: GitHub Issuesを作成する

#### style コマンド
```bash
aad style list
aad style apply <STYLE-NAME>
```
スタイルを管理する。

**サブコマンド**:
- `list`: スタイル一覧を表示
- `apply <STYLE-NAME>`: スタイルを適用

#### worktree コマンド
```bash
aad worktree <SPEC-ID>
```
Git worktree とブランチを作成する。

**引数**:
- `SPEC-ID`: 仕様ID

---

## ✅ 受け入れ基準

テスト可能な形式で記述すること

### 機能テスト

#### REQ-1: cli クレート作成
- [ ] AC-1.1: `crates/cli/Cargo.toml` が存在する
- [ ] AC-1.2: `crates/cli/src/main.rs` が存在する
- [ ] AC-1.3: `clap` クレートが依存に含まれている（derive feature 有効）
- [ ] AC-1.4: `cargo build -p cli` が成功する

#### REQ-2: clap によるコマンド定義
- [ ] AC-2.1: `cli/src/commands/` モジュールが存在する
- [ ] AC-2.2: `#[derive(Parser)]` を使用したコマンド構造体が定義されている
- [ ] AC-2.3: `aad --help` が適切なヘルプメッセージを表示する
- [ ] AC-2.4: サブコマンド一覧（init, spec, tasks, style, worktree）が表示される

#### REQ-3: DI コンテナ実装
- [ ] AC-3.1: `cli/src/app.rs` に `App` 構造体が定義されている
- [ ] AC-3.2: `App::new()` でリポジトリ実装が注入される
- [ ] AC-3.3: テスト時にモックリポジトリへの切り替えが可能である
- [ ] AC-3.4: `App::run()` メソッドでコマンド実行が行われる

#### REQ-4: init コマンド実装
- [ ] AC-4.1: `aad init` 実行後、`.aad/` ディレクトリが作成される
- [ ] AC-4.2: `.aad/specs/`, `.aad/sessions/`, `.aad/retrospectives/` ディレクトリが作成される
- [ ] AC-4.3: `config/aad.toml` と `config/styles.toml` のテンプレートが配置される
- [ ] AC-4.4: 既存のプロジェクトでは上書き確認が表示される
- [ ] AC-4.5: 成功メッセージが日本語で表示される

#### REQ-5: spec コマンド実装
- [ ] AC-5.1: `aad spec SPEC-001` 実行後、`.aad/specs/SPEC-001.md` が作成される
- [ ] AC-5.2: テンプレートに MoSCoW 優先度セクションが含まれている
- [ ] AC-5.3: 受け入れ基準のプレースホルダーが含まれている
- [ ] AC-5.4: 既存の spec ファイルがある場合は上書き確認が表示される

#### REQ-6: tasks コマンド実装
- [ ] AC-6.1: `aad tasks SPEC-001` 実行後、`.aad/tasks/SPEC-001-tasks.md` が作成される
- [ ] AC-6.2: タスクIDが `SPEC-001-T01`, `SPEC-001-T02`, ... と自動採番される
- [ ] AC-6.3: `--github` オプション指定時、GitHub Issues が作成される（`gh` コマンド使用）
- [ ] AC-6.4: 依存関係フィールドがテンプレートに含まれている

#### REQ-7: style コマンド実装
- [ ] AC-7.1: `aad style list` でスタイル一覧が表示される
- [ ] AC-7.2: `aad style apply <style-name>` でスタイルが `CLAUDE.md` に適用される
- [ ] AC-7.3: トークン置換が正しく動作する（例: `{{role}}` → `賢者`）
- [ ] AC-7.4: 未定義のスタイル指定時にエラーメッセージが表示される

#### REQ-8: worktree コマンド実装
- [ ] AC-8.1: `aad worktree SPEC-001` 実行後、`../aad-SPEC-001/` に worktree が作成される
- [ ] AC-8.2: ブランチ名が `feature/SPEC-001` となる
- [ ] AC-8.3: Git リポジトリでない場合はエラーメッセージが表示される
- [ ] AC-8.4: 既存の worktree がある場合は上書き確認が表示される

### 非機能テスト
- [ ] `aad --help` が正しく表示される
- [ ] 各コマンド（init, spec, tasks, style, worktree）が基本動作する
- [ ] エラーハンドリングが適切に実装されている
- [ ] ユーザーフィードバックメッセージが日本語で明確である
- [ ] `cargo test -p cli` が全て pass する

### セキュリティ
- [ ] コマンドインジェクション対策が実装されている（Git操作）
- [ ] パストラバーサル対策が実装されている

---

## 🚧 制約・前提条件

### 技術的制約
- Git 操作には `git2` クレートまたは `std::process::Command` を使用する
- テンプレートエンジンには `handlebars` を使用する
- エラーメッセージは日本語でユーザーフレンドリーに記述する
- `--help` メッセージは日本語と英語の両方を考慮する

### ビジネス制約
- 期間: 1週間

### 依存関係
- SPEC-001（Domain基盤）の完了が前提
- SPEC-002（設定管理 + ワークフロー）の完了が前提

---

## 🔄 マイグレーション計画

### 既存データへの影響
N/A（新規機能）

### ロールバック計画
N/A（新規機能）

---

## 📚 参考資料

- [clap公式ドキュメント](https://docs.rs/clap/)
- [handlebars公式ドキュメント](https://docs.rs/handlebars/)
- [git2公式ドキュメント](https://docs.rs/git2/)
- [SPEC-001: Domain基盤](./SPEC-001.md)
- [SPEC-002: 設定管理 + ワークフロー](./SPEC-002.md)

---

## 📝 変更履歴

| 日付 | バージョン | 変更内容 | 変更者 |
|------|-----------|----------|--------|
| 2026-01-18 | 1.0 | 初版作成 | Claude Code |

---

## 💬 レビューコメント

（レビュー時に追記）

---

## ✅ 承認

- [x] 技術レビュー完了（担当: ユーザー、日付: 2026-01-18）
- [x] ビジネスレビュー完了（担当: ユーザー、日付: 2026-01-18）
- [x] 最終承認（担当: ユーザー、日付: 2026-01-18）

---

**注意**: この仕様書は承認後にタスク分割（`/aad:tasks SPEC-003`）を実行してください。
