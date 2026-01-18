# Phase 3: CLI 基本コマンド - 要件定義

## プロジェクト概要

**目標**: 基本的な CLI コマンドを実装し、ユーザーがコマンドラインから操作できるようにする。

**期間**: 1週間

**依存関係**: Phase 1, 2 完了（Domain + Config 層が必要）

## 要件

### REQ-1: cli クレート作成

**要件文 (EARS形式)**:
The system shall `crates/cli/` ディレクトリに cli クレートを作成し、CLI 基盤を提供すること。

**受入基準 (AC)**:
- AC-1.1: `crates/cli/Cargo.toml` が存在する
- AC-1.2: `crates/cli/src/main.rs` が存在する
- AC-1.3: `clap` クレートが依存に含まれている（derive feature 有効）
- AC-1.4: `cargo build -p cli` が成功する

**優先度**: Must

---

### REQ-2: clap によるコマンド定義

**要件文 (EARS形式)**:
The system shall `clap` を使用してサブコマンド構造を設計し、各コマンドの引数・オプションを定義すること。

**受入基準 (AC)**:
- AC-2.1: `cli/src/commands/` モジュールが存在する
- AC-2.2: `#[derive(Parser)]` を使用したコマンド構造体が定義されている
- AC-2.3: `aad --help` が適切なヘルプメッセージを表示する
- AC-2.4: サブコマンド一覧（init, spec, tasks, style, worktree）が表示される

**優先度**: Must

---

### REQ-3: DI コンテナ実装

**要件文 (EARS形式)**:
The system shall 依存性注入の仕組みを構築し、リポジトリ実装を切り替え可能にすること。

**受入基準 (AC)**:
- AC-3.1: `cli/src/app.rs` に `App` 構造体が定義されている
- AC-3.2: `App::new()` でリポジトリ実装が注入される
- AC-3.3: テスト時にモックリポジトリへの切り替えが可能である
- AC-3.4: `App::run()` メソッドでコマンド実行が行われる

**優先度**: Must

---

### REQ-4: init コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad init` を実行した場合、the system shall プロジェクトを初期化し、`.aad/` ディレクトリとテンプレートファイルを配置すること。

**受入基準 (AC)**:
- AC-4.1: `aad init` 実行後、`.aad/` ディレクトリが作成される
- AC-4.2: `.aad/specs/`, `.aad/sessions/`, `.aad/retrospectives/` ディレクトリが作成される
- AC-4.3: `config/aad.toml` と `config/styles.toml` のテンプレートが配置される
- AC-4.4: 既存のプロジェクトでは上書き確認が表示される
- AC-4.5: 成功メッセージが日本語で表示される

**優先度**: Must

---

### REQ-5: spec コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad spec <spec-id>` を実行した場合、the system shall 仕様ファイルを作成し、受け入れ基準のテンプレートを生成すること。

**受入基準 (AC)**:
- AC-5.1: `aad spec SPEC-001` 実行後、`.aad/specs/SPEC-001.md` が作成される
- AC-5.2: テンプレートに MoSCoW 優先度セクションが含まれている
- AC-5.3: 受け入れ基準のプレースホルダーが含まれている
- AC-5.4: 既存の spec ファイルがある場合は上書き確認が表示される

**優先度**: Must

---

### REQ-6: tasks コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad tasks <spec-id>` を実行した場合、the system shall タスクを分割し、タスクIDを自動採番すること。

**受入基準 (AC)**:
- AC-6.1: `aad tasks SPEC-001` 実行後、`.aad/tasks/SPEC-001-tasks.md` が作成される
- AC-6.2: タスクIDが `SPEC-001-T01`, `SPEC-001-T02`, ... と自動採番される
- AC-6.3: `--github` オプション指定時、GitHub Issues が作成される（`gh` コマンド使用）
- AC-6.4: 依存関係フィールドがテンプレートに含まれている

**優先度**: Must

---

### REQ-7: style コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad style <style-name>` を実行した場合、the system shall スタイルを切り替え、トークンを適用すること。

**受入基準 (AC)**:
- AC-7.1: `aad style list` でスタイル一覧が表示される
- AC-7.2: `aad style apply <style-name>` でスタイルが `CLAUDE.md` に適用される
- AC-7.3: トークン置換が正しく動作する（例: `{{role}}` → `賢者`）
- AC-7.4: 未定義のスタイル指定時にエラーメッセージが表示される

**優先度**: Must

---

### REQ-8: worktree コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad worktree <spec-id>` を実行した場合、the system shall Git worktree とブランチを作成すること。

**受入基準 (AC)**:
- AC-8.1: `aad worktree SPEC-001` 実行後、`../aad-SPEC-001/` に worktree が作成される
- AC-8.2: ブランチ名が `feature/SPEC-001` となる
- AC-8.3: Git リポジトリでない場合はエラーメッセージが表示される
- AC-8.4: 既存の worktree がある場合は上書き確認が表示される

**優先度**: Must

---

## 完了条件

Phase 3 は以下の条件をすべて満たした場合に完了とする:

1. ✅ `aad --help` が正しく表示される
2. ✅ 各コマンド（init, spec, tasks, style, worktree）が基本動作する
3. ✅ エラーハンドリングが適切に実装されている
4. ✅ ユーザーフィードバックメッセージが日本語で明確である
5. ✅ `cargo test -p cli` が全て pass する
6. ✅ すべての要件（REQ-1 〜 REQ-8）の受入基準が満たされている

## 成果物

- `crates/cli/` クレート
  - `src/main.rs`
  - `src/app.rs` (DI コンテナ)
  - `src/commands/init.rs`
  - `src/commands/spec.rs`
  - `src/commands/tasks.rs`
  - `src/commands/style.rs`
  - `src/commands/worktree.rs`
- テンプレートファイル
  - `templates/spec.md.hbs`
  - `templates/tasks.md.hbs`
  - `templates/aad.toml.hbs`
  - `templates/styles.toml.hbs`

## 備考

- Git 操作には `git2` クレートまたは `std::process::Command` を使用する
- テンプレートエンジンには `handlebars` を使用する
- エラーメッセージは日本語でユーザーフレンドリーに記述する
- `--help` メッセージは日本語と英語の両方を考慮する

---

**最終更新**: 2026-01-18
