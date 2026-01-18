# Phase 1: プロジェクト構造 + Domain基盤 - 要件定義

## プロジェクト概要

**目標**: Rust ワークスペースと Domain 層の基盤を構築する。クリーンアーキテクチャの中核となるドメインモデルを定義し、ビジネスロジックの基盤を整備する。

**期間**: 1週間

**依存関係**: なし（最初のフェーズ）

## 要件

### REQ-1: Rust ワークスペース初期化

**要件文 (EARS形式)**:
When プロジェクトが初期化される場合、the system shall Rust ワークスペースを作成し、ルート `Cargo.toml` でワークスペースメンバーを定義すること。

**受入基準 (AC)**:
- AC-1.1: ルート `Cargo.toml` が存在し、`[workspace]` セクションが定義されている
- AC-1.2: `members` フィールドに `crates/domain` が含まれている
- AC-1.3: `cargo build` がエラーなく実行できる

**優先度**: Must

---

### REQ-2: domain クレート実装

**要件文 (EARS形式)**:
The system shall `crates/domain/` ディレクトリに domain クレートを作成し、Domain 層の基盤を提供すること。

**受入基準 (AC)**:
- AC-2.1: `crates/domain/Cargo.toml` が存在する
- AC-2.2: `crates/domain/src/lib.rs` が存在する
- AC-2.3: `cargo build -p domain` が成功する

**優先度**: Must

---

### REQ-3: Entities 定義

**要件文 (EARS形式)**:
The system shall 以下の5つのドメインエンティティを定義すること:
- `Spec`: 仕様エンティティ
- `Task`: タスクエンティティ
- `Session`: セッションエンティティ
- `Workflow`: ワークフローエンティティ
- `Style`: スタイルエンティティ

**受入基準 (AC)**:
- AC-3.1: `domain/src/entities/spec.rs` が存在し、`Spec` 構造体が定義されている
- AC-3.2: `domain/src/entities/task.rs` が存在し、`Task` 構造体が定義されている
- AC-3.3: `domain/src/entities/session.rs` が存在し、`Session` 構造体が定義されている
- AC-3.4: `domain/src/entities/workflow.rs` が存在し、`Workflow` 構造体が定義されている
- AC-3.5: `domain/src/entities/style.rs` が存在し、`Style` 構造体が定義されている
- AC-3.6: 各エンティティが `Clone`, `Debug` トレイトを実装している

**優先度**: Must

---

### REQ-4: Value Objects 定義

**要件文 (EARS形式)**:
The system shall 以下の Value Objects を定義すること:
- `SpecId`, `TaskId`: ID型
- `Phase`: フェーズ列挙型（SPEC, TASKS, TDD, REVIEW, RETRO, MERGE）
- `Status`: ステータス列挙型（pending, in_progress, done, failed）
- `Priority`: 優先度列挙型（Must, Should, Could, Won't）
- `StyleName`: スタイル名
- `TokenMap`: トークンマッピング

**受入基準 (AC)**:
- AC-4.1: `domain/src/value_objects/ids.rs` に `SpecId`, `TaskId` が定義されている
- AC-4.2: `domain/src/value_objects/phase.rs` に `Phase` enum が定義され、6つのバリアントを持つ
- AC-4.3: `domain/src/value_objects/status.rs` に `Status` enum が定義され、4つのバリアントを持つ
- AC-4.4: `domain/src/value_objects/priority.rs` に `Priority` enum が定義され、MoSCoW形式の4つのバリアントを持つ
- AC-4.5: `domain/src/value_objects/style.rs` に `StyleName` と `TokenMap` が定義されている
- AC-4.6: 各 Value Object が適切なバリデーションロジックを持つ

**優先度**: Must

---

### REQ-5: Repository トレイト定義

**要件文 (EARS形式)**:
The system shall 以下の Repository トレイトを定義すること:
- `SpecRepository`
- `TaskRepository`
- `SessionRepository`

**受入基準 (AC)**:
- AC-5.1: `domain/src/repositories/spec_repository.rs` に `SpecRepository` トレイトが定義されている
- AC-5.2: `SpecRepository` が `find_by_id`, `save`, `delete` メソッドを持つ
- AC-5.3: `domain/src/repositories/task_repository.rs` に `TaskRepository` トレイトが定義されている
- AC-5.4: `TaskRepository` が `find_by_id`, `find_by_spec_id`, `save`, `delete` メソッドを持つ
- AC-5.5: `domain/src/repositories/session_repository.rs` に `SessionRepository` トレイトが定義されている
- AC-5.6: `SessionRepository` が `find_by_id`, `save`, `delete` メソッドを持つ
- AC-5.7: 各メソッドが適切な戻り値型（`Result<T, E>`）を持つ

**優先度**: Must

---

### REQ-6: 単体テスト実装

**要件文 (EARS形式)**:
The system shall 各エンティティと Value Object に対して単体テストを実装すること。

**受入基準 (AC)**:
- AC-6.1: `domain/src/entities/` 配下の各ファイルに `#[cfg(test)]` モジュールが存在する
- AC-6.2: `domain/src/value_objects/` 配下の各ファイルに `#[cfg(test)]` モジュールが存在する
- AC-6.3: `cargo test -p domain` が全てパスする
- AC-6.4: テストカバレッジが80%以上である

**優先度**: Must

---

## 完了条件

Phase 1 は以下の条件をすべて満たした場合に完了とする:

1. ✅ `cargo build --all` が成功する
2. ✅ `cargo test --all` が全て pass する（80%以上のカバレッジ）
3. ✅ `cargo clippy --all` がエラーなく完了する
4. ✅ `cargo fmt --all -- --check` がエラーなく完了する
5. ✅ すべての要件（REQ-1 〜 REQ-6）の受入基準が満たされている
6. ✅ ドメインモデルのドキュメント（rustdoc）が記述されている

## 成果物

- `Cargo.toml` (ワークスペース定義)
- `crates/domain/` クレート
  - `src/entities/` (5つのエンティティ)
  - `src/value_objects/` (6つの Value Objects)
  - `src/repositories/` (3つの Repository トレイト)
- 単体テスト

## 備考

- このフェーズでは具象実装（Infrastructure 層）は含まれない
- Repository はトレイト定義のみで、JSON や DB への保存ロジックは Phase 5 で実装
- テストは `#[cfg(test)]` モジュール内に記述し、外部テストファイルは使用しない

---

**最終更新**: 2026-01-18
