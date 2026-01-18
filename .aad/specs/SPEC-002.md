# SPEC-002: 設定管理 + ワークフロー

**作成日**: 2026-01-18

**担当者**: Claude Code

**ステータス**: Approved

**関連Issue**: N/A

---

## 📋 概要

TOML 設定ファイルの読み込みとワークフロー定義を実装する。スタイルシステムの設定パーサーも含め、アプリケーション層とインフラストラクチャ層の基盤を確立する。Phase間の状態遷移ロジックを実装し、設定値のバリデーションを提供する。

---

## 🎯 目的

### ビジネス目標
設定ファイルによる柔軟なカスタマイズを可能にし、ワークフロー管理の基盤を構築する。スタイルシステムにより、出力形式の変換機能を提供し、多様なユーザーニーズに対応する。

### ユーザーストーリー
```
As a 開発者
I want to TOML設定ファイルでアプリケーションの動作をカスタマイズする
So that プロジェクトの要件に合わせた柔軟な開発フローを実現できる
```

---

## 🔍 要件定義(MoSCoW)

### Must Have（必須）
これらがないとリリースできない機能

- [ ] **REQ-1: application クレート作成** - `crates/application/` ディレクトリに application クレートを作成し、ユースケース層の基盤を提供する。`domain` クレートへの依存を定義する。
- [ ] **REQ-2: infrastructure/config 実装** - `crates/infrastructure/` クレートを作成し、TOML パーサーを実装する。`toml`, `serde`, `serde_derive` を依存に含める。
- [ ] **REQ-3: AadConfig 構造体実装** - `config/aad.toml` ファイルを読み込み、`AadConfig` 構造体にデシリアライズする。デフォルト値と適切なエラーハンドリングを提供する。
- [ ] **REQ-4: StyleConfig 構造体実装** - `config/styles.toml` ファイルを読み込み、スタイル定義を `StyleName` と `TokenMap` にマッピングする。未定義スタイルやトークン重複を検出する。
- [ ] **REQ-5: ワークフロー状態遷移ロジック実装** - Phase 間の状態遷移ルールを実装し、不正な遷移を防ぐ。正当な遷移（SPEC → TASKS → TDD → REVIEW → RETRO → MERGE）のみを許可する。
- [ ] **REQ-6: バリデーション実装** - 設定ファイル読み込み時に、必須フィールドの欠落、数値範囲、パス指定の妥当性をチェックする。日本語でユーザーフレンドリーなエラーメッセージを表示する。

### Should Have（重要）
できるだけ含めるべき機能

- なし

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- なし

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] **GUI設定エディタ** - 理由: CLIツールのため不要
- [ ] **設定ファイルの自動生成ウィザード** - 理由: サンプルファイルで十分

---

## 🎨 UI/UX要件

### 画面構成
N/A（設定ファイルベース）

### 主要な操作フロー
1. ユーザーが `config/aad.toml` を編集
2. アプリケーション起動時に設定を読み込み
3. バリデーションエラーがあればわかりやすいメッセージを表示
4. 設定に従ってワークフローを実行

### レスポンシブ対応
N/A

---

## 🔧 技術要件

### フロントエンド
N/A

### バックエンド
- **言語**: Rust (Edition 2021)
- **設定フォーマット**: TOML
- **パーサー**: `toml` クレート（最新安定版）
- **シリアライズ**: `serde`, `serde_derive`
- **エラーハンドリング**: `anyhow` または `thiserror`

### データベース
N/A

### パフォーマンス要件
- 設定ファイル読み込み時間: 100ms以内
- バリデーション実行時間: 50ms以内

---

## 📊 データモデル

### 設定構造体

#### `AadConfig` 構造体
アプリケーション全体の設定

| フィールド名 | 型 | 制約 | 説明 |
|------------|-----|------|------|
| version | String | 必須 | 設定ファイルバージョン |
| context_threshold | u8 | 0-100 | コンテキスト使用率の警告閾値 |
| default_branch | Option<String> | - | デフォルトブランチ名 |
| workflow | WorkflowConfig | 必須 | ワークフロー設定 |

#### `StyleConfig` 構造体
スタイル変換設定

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| styles | HashMap<StyleName, TokenMap> | スタイル名とトークンマッピング |

#### `WorkflowConfig` 構造体
ワークフロー設定

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| phases | Vec<Phase> | 有効なフェーズのリスト |
| auto_transition | bool | 自動遷移の有効化 |

---

## 🔗 API仕様

### AadConfig API

```rust
impl AadConfig {
    /// 設定ファイルを読み込む
    pub fn load(path: &Path) -> Result<Self, Error>;

    /// デフォルト設定を取得
    pub fn default() -> Self;

    /// 設定を検証する
    pub fn validate(&self) -> Result<(), ValidationError>;
}
```

### StyleConfig API

```rust
impl StyleConfig {
    /// スタイル設定ファイルを読み込む
    pub fn load(path: &Path) -> Result<Self, Error>;

    /// スタイル名からTokenMapを取得
    pub fn get_token_map(&self, name: &StyleName) -> Option<&TokenMap>;
}
```

### Workflow Transition API

```rust
/// 状態遷移が可能かチェック
pub fn can_transition(from: Phase, to: Phase) -> bool;

/// 次のフェーズを取得
pub fn next_phase(current: Phase) -> Option<Phase>;
```

---

## ✅ 受け入れ基準

テスト可能な形式で記述すること

### 機能テスト

#### REQ-1: application クレート作成
- [ ] AC-1.1: `crates/application/Cargo.toml` が存在する
- [ ] AC-1.2: `crates/application/src/lib.rs` が存在する
- [ ] AC-1.3: `cargo build -p application` が成功する
- [ ] AC-1.4: `domain` クレートへの依存が定義されている

#### REQ-2: infrastructure/config 実装
- [ ] AC-2.1: `crates/infrastructure/Cargo.toml` が存在し、`toml` クレートが依存に含まれている
- [ ] AC-2.2: `crates/infrastructure/src/config/` モジュールが存在する
- [ ] AC-2.3: `serde`, `serde_derive` が依存に含まれている
- [ ] AC-2.4: `cargo build -p infrastructure` が成功する

#### REQ-3: AadConfig 構造体実装
- [ ] AC-3.1: `infrastructure/src/config/aad_config.rs` に `AadConfig` 構造体が定義されている
- [ ] AC-3.2: `AadConfig::load(path)` メソッドが実装されている
- [ ] AC-3.3: デフォルト値が `Default` トレイトで提供されている
- [ ] AC-3.4: 必須フィールドの欠落時に適切なエラーを返す
- [ ] AC-3.5: 不正な TOML 構文に対してエラーを返す

#### REQ-4: StyleConfig 構造体実装
- [ ] AC-4.1: `infrastructure/src/config/style_config.rs` に `StyleConfig` 構造体が定義されている
- [ ] AC-4.2: `StyleConfig::load(path)` メソッドが実装されている
- [ ] AC-4.3: 各スタイル定義が `StyleName` と `TokenMap` にマッピングされる
- [ ] AC-4.4: 未定義のスタイルへのアクセス時にエラーを返す
- [ ] AC-4.5: トークン名の重複が検出される

#### REQ-5: ワークフロー状態遷移ロジック実装
- [ ] AC-5.1: `application/src/workflow/transition.rs` に遷移ロジックが実装されている
- [ ] AC-5.2: 正当な遷移（SPEC → TASKS → TDD → REVIEW → RETRO → MERGE）が許可される
- [ ] AC-5.3: 不正な遷移（例: SPEC → REVIEW）が拒否される
- [ ] AC-5.4: `can_transition(from: Phase, to: Phase) -> bool` メソッドが実装されている
- [ ] AC-5.5: `next_phase(current: Phase) -> Option<Phase>` メソッドが実装されている

#### REQ-6: バリデーション実装
- [ ] AC-6.1: 必須フィールドが欠落している場合にエラーメッセージを表示する
- [ ] AC-6.2: 数値範囲のバリデーションが実装されている（例: `context_threshold` が 0-100 の範囲内）
- [ ] AC-6.3: パス指定のバリデーションが実装されている
- [ ] AC-6.4: エラーメッセージが日本語でユーザーフレンドリーである

### 非機能テスト
- [ ] `cargo build --all` が成功する
- [ ] `cargo test --all` が全て pass する
- [ ] `config/aad.toml` と `config/styles.toml` のサンプルファイルが作成されている
- [ ] ワークフロー状態遷移が正しく機能する
- [ ] 設定ファイルのバリデーションエラーが適切に表示される

### セキュリティ
- [ ] 設定ファイルのパスインジェクション対策が実装されている
- [ ] 不正な設定値による DoS 攻撃が防止されている

---

## 🚧 制約・前提条件

### 技術的制約
- `toml` クレートのバージョンは最新の安定版を使用する
- エラーハンドリングには `anyhow` または `thiserror` を使用する
- 設定ファイルのスキーマ変更に備え、バージョンフィールドを含める

### ビジネス制約
- 期間: 1週間

### 依存関係
- SPEC-001（Domain基盤）の完了が前提

---

## 🔄 マイグレーション計画

### 既存データへの影響
N/A（新規機能）

### ロールバック計画
設定ファイルのバージョンフィールドにより、将来的なスキーマ変更時の互換性を保つ。

---

## 📚 参考資料

- [TOML仕様](https://toml.io/)
- [serde公式ドキュメント](https://serde.rs/)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [SPEC-001: Domain基盤](./SPEC-001.md)

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

**注意**: この仕様書は承認後にタスク分割（`/aad:tasks SPEC-002`）を実行してください。
