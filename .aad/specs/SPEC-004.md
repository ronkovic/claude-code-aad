# SPEC-004: オーケストレーション

**作成日**: 2026-01-18

**担当者**: Claude Code

**ステータス**: Approved

**関連Issue**: N/A

---

## 📋 概要

3層オーケストレーターの実装により、複数の Spec を並列実行し、自動エスカレーションを実現する。依存関係グラフに基づいた適切な実行順序の決定、セッション状態の監視、失敗時のエスカレーション処理を提供する。非同期ランタイム（tokio）を使用した効率的な並列実行を実現する。

---

## 🎯 目的

### ビジネス目標
複数の開発タスクを効率的に並列実行し、依存関係を自動管理することで、開発サイクルの高速化を実現する。失敗時の自動エスカレーションにより、人間の介入が必要な問題を早期に検出し、スムーズな問題解決を支援する。

### ユーザーストーリー
```
As a 開発者
I want to 複数の仕様を自動的に並列実行し、依存関係を管理する
So that 開発プロセス全体を効率化し、問題発生時には適切にエスカレーションされる
```

---

## 🔍 要件定義（MoSCoW）

### Must Have（必須）
これらがないとリリースできない機能

- [ ] **REQ-1: Orchestrator 構造体実装** - `Orchestrator` 構造体を実装し、複数のセッションを管理する機能を提供する。セッション管理用のデータ構造と非同期ランタイム（tokio）の統合を含む。
- [ ] **REQ-2: セッション登録・起動ロジック実装** - 複数の Spec を登録し、依存関係に基づいて適切な順序で起動する。トポロジカルソートで実行順序を決定し、並列実行可能な Spec を同時起動する。
- [ ] **REQ-3: モニターループ実装** - 定期的にセッション状態をチェックし、進捗を監視する。完了・失敗・タイムアウトを検出し、セッションの進捗率を計算する。
- [ ] **REQ-4: エスカレーション処理実装** - Child Session が失敗した場合、親セッションにエスカレーションし、適切なハンドリングを行う。エスカレーション情報をログに記録し、人間への通知メカニズムを提供する。
- [ ] **REQ-5: 完了・失敗ハンドリング実装** - セッションが完了または失敗した場合、適切な後処理を実行する。正常完了時に次のセッションを起動し、失敗時にリトライ可否を判定する。
- [ ] **REQ-6: orchestrate コマンド実装** - `aad orchestrate` コマンドでオーケストレーターを起動し、登録された Spec を実行する。実行ログをコンソールに表示し、完了時に結果サマリーを表示する。

### Should Have（重要）
できるだけ含めるべき機能

- [ ] **REQ-7: --resume, --dry-run オプション実装** - `--resume` オプションで中断したオーケストレーションを再開し、`--dry-run` オプションで実行計画を表示する。依存関係グラフを視覚的に表示する。

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- なし

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] **リアルタイムダッシュボード** - 理由: Phase 6 で TUI ダッシュボードとして実装予定
- [ ] **分散実行** - 理由: 単一マシンでの実行に限定

---

## 🎨 UI/UX要件

### 画面構成
コマンドラインインターフェース（進捗表示とログ出力）

### 主要な操作フロー

#### 1. 複数Spec実行
```bash
$ aad orchestrate --specs SPEC-001,SPEC-002,SPEC-003
🚀 オーケストレーターを起動しました
📊 実行計画:
  - SPEC-001 (依存なし)
  - SPEC-002 (依存: SPEC-001)
  - SPEC-003 (依存: SPEC-001)

⏳ SPEC-001 を開始...
✓ SPEC-001 が完了しました (5分12秒)

⏳ SPEC-002, SPEC-003 を並列実行...
✓ SPEC-002 が完了しました (3分45秒)
⚠ SPEC-003 でエラーが発生しました
  → エスカレーションログ: .aad/escalations/SPEC-003-error.json

📈 実行結果サマリー:
  成功: 2件 (SPEC-001, SPEC-002)
  失敗: 1件 (SPEC-003)
  合計時間: 8分57秒
```

#### 2. ドライラン
```bash
$ aad orchestrate --dry-run --specs SPEC-001,SPEC-002
📋 実行計画（ドライラン）:
  ┌─ SPEC-001
  │   ├─ SPEC-002
  │   └─ SPEC-003

  実行順序: SPEC-001 → [SPEC-002, SPEC-003 並列]
  推定時間: 約10分
```

### レスポンシブ対応
N/A

---

## 🔧 技術要件

### フロントエンド
N/A

### バックエンド
- **言語**: Rust (Edition 2021)
- **非同期ランタイム**: tokio
- **並列実行**: tokio::task::spawn
- **依存関係解決**: トポロジカルソート
- **ログ形式**: JSON

### データベース
N/A

### パフォーマンス要件
- モニターループ実行間隔: 1秒
- セッション起動時間: 500ms以内
- 並列実行数: CPU コア数まで（設定可能）

---

## 📊 データモデル

### オーケストレーション関連構造体

#### `Orchestrator` 構造体

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| sessions | HashMap<SessionId, Session> | 管理中のセッション |
| dependency_graph | DependencyGraph | 依存関係グラフ |
| config | OrchestratorConfig | オーケストレーター設定 |

#### `DependencyGraph` 構造体

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| nodes | HashMap<SpecId, Vec<SpecId>> | 依存関係マップ |

#### `EscalationLevel` enum
エスカレーションレベル
- `Warning`: 警告レベル
- `Error`: エラーレベル
- `Critical`: クリティカルレベル

#### `EscalationLog` 構造体

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| session_id | SessionId | セッションID |
| level | EscalationLevel | エスカレーションレベル |
| reason | String | エスカレーション理由 |
| timestamp | DateTime | 発生日時 |

---

## 🔗 API仕様

### Orchestrator API

```rust
impl Orchestrator {
    /// 新しいオーケストレーターを作成
    pub fn new(config: OrchestratorConfig) -> Self;

    /// Specを登録
    pub fn register_spec(&mut self, spec_id: SpecId);

    /// セッションを起動
    pub async fn start_session(&mut self, session_id: SessionId) -> Result<(), Error>;

    /// モニターループを開始
    pub async fn monitor_loop(&mut self);

    /// エスカレーション処理
    pub fn escalate(&mut self, session_id: SessionId, reason: String);

    /// セッション完了ハンドラ
    pub fn on_session_completed(&mut self, session_id: SessionId);

    /// セッション失敗ハンドラ
    pub fn on_session_failed(&mut self, session_id: SessionId, error: Error);
}
```

### DependencyGraph API

```rust
impl DependencyGraph {
    /// 依存関係を追加
    pub fn add_dependency(&mut self, spec_id: SpecId, depends_on: SpecId);

    /// トポロジカルソート
    pub fn topological_sort(&self) -> Result<Vec<SpecId>, Error>;

    /// 循環依存を検出
    pub fn detect_cycle(&self) -> Option<Vec<SpecId>>;
}
```

---

## ✅ 受け入れ基準

テスト可能な形式で記述すること

### 機能テスト

#### REQ-1: Orchestrator 構造体実装
- [ ] AC-1.1: `application/src/orchestration/orchestrator.rs` に `Orchestrator` 構造体が定義されている
- [ ] AC-1.2: セッション管理用のデータ構造（`HashMap<SessionId, Session>`）が含まれている
- [ ] AC-1.3: `new()` メソッドで初期化が可能である
- [ ] AC-1.4: 並列実行のための非同期ランタイム（tokio）が統合されている

#### REQ-2: セッション登録・起動ロジック実装
- [ ] AC-2.1: `register_spec(spec_id)` メソッドが実装されている
- [ ] AC-2.2: `start_session(session_id)` メソッドが Child Session を起動する
- [ ] AC-2.3: 依存関係グラフが構築され、トポロジカルソートで実行順序が決定される
- [ ] AC-2.4: 並列実行可能な Spec が同時に起動される
- [ ] AC-2.5: 依存関係の循環検出が実装されている

#### REQ-3: モニターループ実装
- [ ] AC-3.1: `monitor_loop()` メソッドが非同期で実装されている
- [ ] AC-3.2: 1秒ごとに全セッションの状態をチェックする
- [ ] AC-3.3: 完了・失敗・タイムアウトを検出する
- [ ] AC-3.4: セッションの進捗率が計算される
- [ ] AC-3.5: ログが適切に記録される

#### REQ-4: エスカレーション処理実装
- [ ] AC-4.1: `escalate(session_id, reason)` メソッドが実装されている
- [ ] AC-4.2: エスカレーション情報が親セッションに通知される
- [ ] AC-4.3: `.aad/escalations/` ディレクトリにログが記録される
- [ ] AC-4.4: エスカレーションレベル（警告、エラー、クリティカル）が区別される
- [ ] AC-4.5: 人間への通知メカニズムが実装されている（ログ出力）

#### REQ-5: 完了・失敗ハンドリング実装
- [ ] AC-5.1: `on_session_completed(session_id)` メソッドが実装されている
- [ ] AC-5.2: `on_session_failed(session_id, error)` メソッドが実装されている
- [ ] AC-5.3: 正常完了時に次のセッションが起動される
- [ ] AC-5.4: 失敗時にリトライ可否が判定される
- [ ] AC-5.5: ロールバック処理が実装されている（必要に応じて）

#### REQ-6: orchestrate コマンド実装
- [ ] AC-6.1: `cli/src/commands/orchestrate.rs` が実装されている
- [ ] AC-6.2: `aad orchestrate --specs SPEC-001,SPEC-002` で複数 Spec が指定可能である
- [ ] AC-6.3: 実行ログがコンソールに表示される
- [ ] AC-6.4: 実行完了時に結果サマリーが表示される

#### REQ-7: --resume, --dry-run オプション実装
- [ ] AC-7.1: `aad orchestrate --resume` で前回の状態から再開できる
- [ ] AC-7.2: `aad orchestrate --dry-run --specs SPEC-001,SPEC-002` で実行計画が表示される
- [ ] AC-7.3: ドライランでは実際の Spec 実行は行われない
- [ ] AC-7.4: 依存関係グラフが視覚的に表示される

### 非機能テスト
- [ ] 複数 Spec の並列実行が可能である
- [ ] エスカレーションが正しく動作する
- [ ] 失敗時のリカバリーが機能する
- [ ] ドライランで実行計画が確認可能である
- [ ] `cargo test -p application` が全て pass する

### セキュリティ
- [ ] セッション間のデータ分離が保証されている
- [ ] エスカレーションログに機密情報が含まれないよう配慮されている

---

## 🚧 制約・前提条件

### 技術的制約
- 非同期処理には `tokio` を使用する
- 並列実行数の上限は設定可能とする（デフォルト: CPU コア数）
- エスカレーションログは JSON 形式で保存する
- タイムアウト時間は `config/aad.toml` で設定可能とする

### ビジネス制約
- 期間: 2週間

### 依存関係
- SPEC-001（Domain基盤）の完了が前提
- SPEC-002（設定管理 + ワークフロー）の完了が前提
- SPEC-003（CLI基本コマンド）の完了が前提

---

## 🔄 マイグレーション計画

### 既存データへの影響
N/A（新規機能）

### ロールバック計画
`.aad/escalations/` ディレクトリのログにより、失敗したセッションの状態を追跡可能。

---

## 📚 参考資料

- [tokio公式ドキュメント](https://tokio.rs/)
- [トポロジカルソート解説](https://en.wikipedia.org/wiki/Topological_sorting)
- [SPEC-001: Domain基盤](./SPEC-001.md)
- [SPEC-002: 設定管理 + ワークフロー](./SPEC-002.md)
- [SPEC-003: CLI基本コマンド](./SPEC-003.md)

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

**注意**: この仕様書は承認後にタスク分割（`/aad:tasks SPEC-004`）を実行してください。
