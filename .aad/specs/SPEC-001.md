# SPEC-001: プロジェクト構造 + Domain基盤

**作成日**: 2026-01-18

**完了日**: 2026-01-18

**担当者**: Claude Code

**ステータス**: Completed

**関連Issue**: N/A

---

## 📋 概要

Rust ワークスペースと Domain 層の基盤を構築する。クリーンアーキテクチャの中核となるドメインモデルを定義し、ビジネスロジックの基盤を整備する。このフェーズでは、Entities、Value Objects、Repository トレイトを定義し、テスト駆動開発の基盤を確立する。

---

## 🎯 目的

### ビジネス目標
AI駆動開発ツールの中核となるドメインモデルを確立し、後続フェーズでの機能開発の基盤を整備する。クリーンアーキテクチャを採用することで、保守性と拡張性の高いコードベースを実現する。

### ユーザーストーリー
```
As a 開発者
I want to クリーンアーキテクチャに基づいたドメインモデルを定義する
So that ビジネスロジックを明確に分離し、後続フェーズの実装を効率化できる
```

---

## 🔍 要件定義（MoSCoW）

### Must Have（必須）
これらがないとリリースできない機能

- [ ] **REQ-1: Rust ワークスペース初期化** - ルート `Cargo.toml` でワークスペースメンバーを定義し、`crates/domain` を含める。`cargo build` がエラーなく実行できること。
- [ ] **REQ-2: domain クレート実装** - `crates/domain/` ディレクトリに domain クレートを作成し、Domain 層の基盤を提供する。
- [ ] **REQ-3: Entities 定義** - `Spec`, `Task`, `Session`, `Workflow`, `Style` の5つのドメインエンティティを定義する。各エンティティは `Clone`, `Debug` トレイトを実装する。
- [ ] **REQ-4: Value Objects 定義** - `SpecId`, `TaskId`, `Phase`, `Status`, `Priority`, `StyleName`, `TokenMap` を定義し、適切なバリデーションロジックを実装する。
- [ ] **REQ-5: Repository トレイト定義** - `SpecRepository`, `TaskRepository`, `SessionRepository` のトレイトを定義し、各メソッド（find_by_id, save, delete等）を持つ。
- [ ] **REQ-6: 単体テスト実装** - 各エンティティと Value Object に対して単体テストを実装し、テストカバレッジ80%以上を達成する。

### Should Have（重要）
できるだけ含めるべき機能

- なし

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- なし

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] **具象実装（Infrastructure 層）** - 理由: Phase 5 で実装予定
- [ ] **外部テストファイル** - 理由: `#[cfg(test)]` モジュール内に記述する方針

---

## 🎨 UI/UX要件

### 画面構成
N/A（このフェーズはライブラリ層のため、UIは提供しない）

### 主要な操作フロー
N/A

### レスポンシブ対応
N/A

---

## 🔧 技術要件

### フロントエンド
N/A

### バックエンド
- **言語**: Rust (Edition 2021)
- **ワークスペース構成**: Cargo Workspace
- **アーキテクチャ**: クリーンアーキテクチャ（Domain駆動設計）
- **テストフレームワーク**: 標準の `cargo test`

### データベース
N/A（このフェーズでは永続化層は実装しない）

### パフォーマンス要件
- ビルド時間: `cargo build --all` が10秒以内に完了
- テスト実行時間: `cargo test --all` が5秒以内に完了

---

## 📊 データモデル

### Entities

#### `Spec` エンティティ
仕様を表すドメインエンティティ

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| id | SpecId | 仕様ID |
| name | String | 仕様名 |
| description | String | 説明 |
| phase | Phase | 現在のフェーズ |
| status | Status | ステータス |
| created_at | DateTime | 作成日時 |

#### `Task` エンティティ
タスクを表すドメインエンティティ

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| id | TaskId | タスクID |
| spec_id | SpecId | 紐づく仕様ID |
| title | String | タスク名 |
| status | Status | ステータス |
| priority | Priority | 優先度 |

#### `Session` エンティティ
セッションを表すドメインエンティティ

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| id | String | セッションID |
| spec_id | Option<SpecId> | 紐づく仕様ID |
| phase | Phase | 現在のフェーズ |
| started_at | DateTime | 開始日時 |

#### `Workflow` エンティティ
ワークフローを表すドメインエンティティ

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| phases | Vec<Phase> | フェーズのリスト |
| current_phase | Phase | 現在のフェーズ |

#### `Style` エンティティ
スタイル変換を表すドメインエンティティ

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| name | StyleName | スタイル名 |
| token_map | TokenMap | トークンマッピング |

### Value Objects

#### `SpecId`, `TaskId`
型安全なID

#### `Phase` enum
フェーズを表す列挙型
- `SPEC`: 仕様フェーズ
- `TASKS`: タスク分割フェーズ
- `TDD`: 開発フェーズ
- `REVIEW`: レビューフェーズ
- `RETRO`: 振り返りフェーズ
- `MERGE`: 統合フェーズ

#### `Status` enum
ステータスを表す列挙型
- `pending`: 未着手
- `in_progress`: 進行中
- `done`: 完了
- `failed`: 失敗

#### `Priority` enum
優先度を表す列挙型（MoSCoW形式）
- `Must`: 必須
- `Should`: 重要
- `Could`: あれば良い
- `Won't`: 対象外

#### `StyleName`
スタイル名を表す型

#### `TokenMap`
トークンマッピングを表す型

---

## 🔗 API仕様

### SpecRepository トレイト

```rust
pub trait SpecRepository {
    fn find_by_id(&self, id: &SpecId) -> Result<Option<Spec>, Error>;
    fn save(&self, spec: &Spec) -> Result<(), Error>;
    fn delete(&self, id: &SpecId) -> Result<(), Error>;
}
```

### TaskRepository トレイト

```rust
pub trait TaskRepository {
    fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>, Error>;
    fn find_by_spec_id(&self, spec_id: &SpecId) -> Result<Vec<Task>, Error>;
    fn save(&self, task: &Task) -> Result<(), Error>;
    fn delete(&self, id: &TaskId) -> Result<(), Error>;
}
```

### SessionRepository トレイト

```rust
pub trait SessionRepository {
    fn find_by_id(&self, id: &str) -> Result<Option<Session>, Error>;
    fn save(&self, session: &Session) -> Result<(), Error>;
    fn delete(&self, id: &str) -> Result<(), Error>;
}
```

---

## ✅ 受け入れ基準

テスト可能な形式で記述すること

### 機能テスト

#### REQ-1: Rust ワークスペース初期化
- [ ] AC-1.1: ルート `Cargo.toml` が存在し、`[workspace]` セクションが定義されている
- [ ] AC-1.2: `members` フィールドに `crates/domain` が含まれている
- [ ] AC-1.3: `cargo build` がエラーなく実行できる

#### REQ-2: domain クレート実装
- [ ] AC-2.1: `crates/domain/Cargo.toml` が存在する
- [ ] AC-2.2: `crates/domain/src/lib.rs` が存在する
- [ ] AC-2.3: `cargo build -p domain` が成功する

#### REQ-3: Entities 定義
- [ ] AC-3.1: `domain/src/entities/spec.rs` が存在し、`Spec` 構造体が定義されている
- [ ] AC-3.2: `domain/src/entities/task.rs` が存在し、`Task` 構造体が定義されている
- [ ] AC-3.3: `domain/src/entities/session.rs` が存在し、`Session` 構造体が定義されている
- [ ] AC-3.4: `domain/src/entities/workflow.rs` が存在し、`Workflow` 構造体が定義されている
- [ ] AC-3.5: `domain/src/entities/style.rs` が存在し、`Style` 構造体が定義されている
- [ ] AC-3.6: 各エンティティが `Clone`, `Debug` トレイトを実装している

#### REQ-4: Value Objects 定義
- [ ] AC-4.1: `domain/src/value_objects/ids.rs` に `SpecId`, `TaskId` が定義されている
- [ ] AC-4.2: `domain/src/value_objects/phase.rs` に `Phase` enum が定義され、6つのバリアントを持つ
- [ ] AC-4.3: `domain/src/value_objects/status.rs` に `Status` enum が定義され、4つのバリアントを持つ
- [ ] AC-4.4: `domain/src/value_objects/priority.rs` に `Priority` enum が定義され、MoSCoW形式の4つのバリアントを持つ
- [ ] AC-4.5: `domain/src/value_objects/style.rs` に `StyleName` と `TokenMap` が定義されている
- [ ] AC-4.6: 各 Value Object が適切なバリデーションロジックを持つ

#### REQ-5: Repository トレイト定義
- [ ] AC-5.1: `domain/src/repositories/spec_repository.rs` に `SpecRepository` トレイトが定義されている
- [ ] AC-5.2: `SpecRepository` が `find_by_id`, `save`, `delete` メソッドを持つ
- [ ] AC-5.3: `domain/src/repositories/task_repository.rs` に `TaskRepository` トレイトが定義されている
- [ ] AC-5.4: `TaskRepository` が `find_by_id`, `find_by_spec_id`, `save`, `delete` メソッドを持つ
- [ ] AC-5.5: `domain/src/repositories/session_repository.rs` に `SessionRepository` トレイトが定義されている
- [ ] AC-5.6: `SessionRepository` が `find_by_id`, `save`, `delete` メソッドを持つ
- [ ] AC-5.7: 各メソッドが適切な戻り値型（`Result<T, E>`）を持つ

#### REQ-6: 単体テスト実装
- [ ] AC-6.1: `domain/src/entities/` 配下の各ファイルに `#[cfg(test)]` モジュールが存在する
- [ ] AC-6.2: `domain/src/value_objects/` 配下の各ファイルに `#[cfg(test)]` モジュールが存在する
- [ ] AC-6.3: `cargo test -p domain` が全てパスする
- [ ] AC-6.4: テストカバレッジが80%以上である

### 非機能テスト
- [ ] `cargo build --all` が成功する
- [ ] `cargo test --all` が全て pass する（80%以上のカバレッジ）
- [ ] `cargo clippy --all` がエラーなく完了する
- [ ] `cargo fmt --all -- --check` がエラーなく完了する
- [ ] ドメインモデルのドキュメント（rustdoc）が記述されている

### セキュリティ
N/A（このフェーズでは外部入力を扱わない）

---

## 🚧 制約・前提条件

### 技術的制約
- このフェーズでは具象実装（Infrastructure 層）は含まれない
- Repository はトレイト定義のみで、JSON や DB への保存ロジックは Phase 5 で実装
- テストは `#[cfg(test)]` モジュール内に記述し、外部テストファイルは使用しない

### ビジネス制約
- 期間: 1週間

### 依存関係
- なし（最初のフェーズ）

---

## 🔄 マイグレーション計画

### 既存データへの影響
N/A（新規プロジェクト）

### ロールバック計画
N/A（新規プロジェクト）

---

## 📚 参考資料

- [クリーンアーキテクチャ](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Rustのモジュールシステム](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

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

**注意**: この仕様書は承認後にタスク分割（`/aad:tasks SPEC-001`）を実行してください。
