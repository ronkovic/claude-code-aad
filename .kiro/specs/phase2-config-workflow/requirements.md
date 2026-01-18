# Phase 2: 設定管理 + ワークフロー - 要件定義

## プロジェクト概要

**目標**: TOML 設定ファイルの読み込みとワークフロー定義を実装する。スタイルシステムの設定パーサーも含む。

**期間**: 1週間

**依存関係**: Phase 1 完了（Domain層が必要）

## 要件

### REQ-1: application クレート作成

**要件文 (EARS形式)**:
The system shall `crates/application/` ディレクトリに application クレートを作成し、ユースケース層の基盤を提供すること。

**受入基準 (AC)**:
- AC-1.1: `crates/application/Cargo.toml` が存在する
- AC-1.2: `crates/application/src/lib.rs` が存在する
- AC-1.3: `cargo build -p application` が成功する
- AC-1.4: `domain` クレートへの依存が定義されている

**優先度**: Must

---

### REQ-2: infrastructure/config 実装

**要件文 (EARS形式)**:
The system shall `crates/infrastructure/` クレートを作成し、TOML パーサーを実装すること。

**受入基準 (AC)**:
- AC-2.1: `crates/infrastructure/Cargo.toml` が存在し、`toml` クレートが依存に含まれている
- AC-2.2: `crates/infrastructure/src/config/` モジュールが存在する
- AC-2.3: `serde`, `serde_derive` が依存に含まれている
- AC-2.4: `cargo build -p infrastructure` が成功する

**優先度**: Must

---

### REQ-3: AadConfig 構造体実装

**要件文 (EARS形式)**:
When `config/aad.toml` ファイルが存在する場合、the system shall ファイルを読み込み、`AadConfig` 構造体にデシリアライズすること。

**受入基準 (AC)**:
- AC-3.1: `infrastructure/src/config/aad_config.rs` に `AadConfig` 構造体が定義されている
- AC-3.2: `AadConfig::load(path)` メソッドが実装されている
- AC-3.3: デフォルト値が `Default` トレイトで提供されている
- AC-3.4: 必須フィールドの欠落時に適切なエラーを返す
- AC-3.5: 不正な TOML 構文に対してエラーを返す

**優先度**: Must

---

### REQ-4: StyleConfig 構造体実装

**要件文 (EARS形式)**:
When `config/styles.toml` ファイルが存在する場合、the system shall スタイル定義を読み込み、`TokenMap` を構築すること。

**受入基準 (AC)**:
- AC-4.1: `infrastructure/src/config/style_config.rs` に `StyleConfig` 構造体が定義されている
- AC-4.2: `StyleConfig::load(path)` メソッドが実装されている
- AC-4.3: 各スタイル定義が `StyleName` と `TokenMap` にマッピングされる
- AC-4.4: 未定義のスタイルへのアクセス時にエラーを返す
- AC-4.5: トークン名の重複が検出される

**優先度**: Must

---

### REQ-5: ワークフロー状態遷移ロジック実装

**要件文 (EARS形式)**:
The system shall Phase 間の状態遷移ルールを実装し、不正な遷移を防ぐこと。

**受入基準 (AC)**:
- AC-5.1: `application/src/workflow/transition.rs` に遷移ロジックが実装されている
- AC-5.2: 正当な遷移（SPEC → TASKS → TDD → REVIEW → RETRO → MERGE）が許可される
- AC-5.3: 不正な遷移（例: SPEC → REVIEW）が拒否される
- AC-5.4: `can_transition(from: Phase, to: Phase) -> bool` メソッドが実装されている
- AC-5.5: `next_phase(current: Phase) -> Option<Phase>` メソッドが実装されている

**優先度**: Must

---

### REQ-6: バリデーション実装

**要件文 (EARS form式)**:
The system shall 設定ファイルの読み込み時に、設定値の妥当性をチェックすること。

**受入基準 (AC)**:
- AC-6.1: 必須フィールドが欠落している場合にエラーメッセージを表示する
- AC-6.2: 数値範囲のバリデーションが実装されている（例: `context_threshold` が 0-100 の範囲内）
- AC-6.3: パス指定のバリデーションが実装されている
- AC-6.4: エラーメッセージが日本語でユーザーフレンドリーである

**優先度**: Must

---

## 完了条件

Phase 2 は以下の条件をすべて満たした場合に完了とする:

1. ✅ `cargo build --all` が成功する
2. ✅ `cargo test --all` が全て pass する
3. ✅ `config/aad.toml` と `config/styles.toml` のサンプルファイルが作成されている
4. ✅ ワークフロー状態遷移が正しく機能する
5. ✅ 設定ファイルのバリデーションエラーが適切に表示される
6. ✅ すべての要件（REQ-1 〜 REQ-6）の受入基準が満たされている

## 成果物

- `crates/application/` クレート
- `crates/infrastructure/` クレート
  - `src/config/aad_config.rs`
  - `src/config/style_config.rs`
- `application/src/workflow/transition.rs`
- `config/aad.toml` (サンプル)
- `config/styles.toml` (サンプル)

## 備考

- `toml` クレートのバージョンは最新の安定版を使用する
- エラーハンドリングには `anyhow` または `thiserror` を使用する
- 設定ファイルのスキーマ変更に備え、バージョンフィールドを含める

---

**最終更新**: 2026-01-18
