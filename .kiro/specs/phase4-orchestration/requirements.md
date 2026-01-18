# Phase 4: オーケストレーション - 要件定義

## プロジェクト概要

**目標**: 3層オーケストレーターの実装により、複数の Spec を並列実行し、自動エスカレーションを実現する。

**期間**: 2週間

**依存関係**: Phase 1, 2, 3 完了（Domain + Config + CLI が必要）

## 要件

### REQ-1: Orchestrator 構造体実装

**要件文 (EARS形式)**:
The system shall `Orchestrator` 構造体を実装し、複数のセッションを管理する機能を提供すること。

**受入基準 (AC)**:
- AC-1.1: `application/src/orchestration/orchestrator.rs` に `Orchestrator` 構造体が定義されている
- AC-1.2: セッション管理用のデータ構造（`HashMap<SessionId, Session>`）が含まれている
- AC-1.3: `new()` メソッドで初期化が可能である
- AC-1.4: 並列実行のための非同期ランタイム（tokio）が統合されている

**優先度**: Must

---

### REQ-2: セッション登録・起動ロジック実装

**要件文 (EARS形式)**:
When ユーザーが複数の Spec を登録した場合、the system shall それらを依存関係に基づいて適切な順序で起動すること。

**受入基準 (AC)**:
- AC-2.1: `register_spec(spec_id)` メソッドが実装されている
- AC-2.2: `start_session(session_id)` メソッドが Child Session を起動する
- AC-2.3: 依存関係グラフが構築され、トポロジカルソートで実行順序が決定される
- AC-2.4: 並列実行可能な Spec が同時に起動される
- AC-2.5: 依存関係の循環検出が実装されている

**優先度**: Must

---

### REQ-3: モニターループ実装

**要件文 (EARS形式)**:
While オーケストレーターが実行中の場合、the system shall 定期的にセッション状態をチェックし、進捗を監視すること。

**受入基準 (AC)**:
- AC-3.1: `monitor_loop()` メソッドが非同期で実装されている
- AC-3.2: 1秒ごとに全セッションの状態をチェックする
- AC-3.3: 完了・失敗・タイムアウトを検出する
- AC-3.4: セッションの進捗率が計算される
- AC-3.5: ログが適切に記録される

**優先度**: Must

---

### REQ-4: エスカレーション処理実装

**要件文 (EARS形式)**:
When Child Session が失敗した場合、the system shall 親セッションにエスカレーションし、適切なハンドリングを行うこと。

**受入基準 (AC)**:
- AC-4.1: `escalate(session_id, reason)` メソッドが実装されている
- AC-4.2: エスカレーション情報が親セッションに通知される
- AC-4.3: `.aad/escalations/` ディレクトリにログが記録される
- AC-4.4: エスカレーションレベル（警告、エラー、クリティカル）が区別される
- AC-4.5: 人間への通知メカニズムが実装されている（ログ出力）

**優先度**: Must

---

### REQ-5: 完了・失敗ハンドリング実装

**要件文 (EARS形式)**:
When セッションが完了または失敗した場合、the system shall 適切な後処理を実行すること。

**受入基準 (AC)**:
- AC-5.1: `on_session_completed(session_id)` メソッドが実装されている
- AC-5.2: `on_session_failed(session_id, error)` メソッドが実装されている
- AC-5.3: 正常完了時に次のセッションが起動される
- AC-5.4: 失敗時にリトライ可否が判定される
- AC-5.5: ロールバック処理が実装されている（必要に応じて）

**優先度**: Must

---

### REQ-6: orchestrate コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad orchestrate` を実行した場合、the system shall オーケストレーターを起動し、登録された Spec を実行すること。

**受入基準 (AC)**:
- AC-6.1: `cli/src/commands/orchestrate.rs` が実装されている
- AC-6.2: `aad orchestrate --specs SPEC-001,SPEC-002` で複数 Spec が指定可能である
- AC-6.3: 実行ログがコンソールに表示される
- AC-6.4: 実行完了時に結果サマリーが表示される

**優先度**: Must

---

### REQ-7: --resume, --dry-run オプション実装

**要件文 (EARS形式)**:
The system shall `--resume` オプションで中断したオーケストレーションを再開し、`--dry-run` オプションで実行計画を表示すること。

**受入基準 (AC)**:
- AC-7.1: `aad orchestrate --resume` で前回の状態から再開できる
- AC-7.2: `aad orchestrate --dry-run --specs SPEC-001,SPEC-002` で実行計画が表示される
- AC-7.3: ドライランでは実際の Spec 実行は行われない
- AC-7.4: 依存関係グラフが視覚的に表示される

**優先度**: Should

---

## 完了条件

Phase 4 は以下の条件をすべて満たした場合に完了とする:

1. ✅ 複数 Spec の並列実行が可能である
2. ✅ エスカレーションが正しく動作する
3. ✅ 失敗時のリカバリーが機能する
4. ✅ ドライランで実行計画が確認可能である
5. ✅ `cargo test -p application` が全て pass する
6. ✅ すべての要件（REQ-1 〜 REQ-7）の受入基準が満たされている

## 成果物

- `crates/application/src/orchestration/` モジュール
  - `orchestrator.rs`
  - `session_manager.rs`
  - `dependency_graph.rs`
  - `escalation.rs`
- `crates/cli/src/commands/orchestrate.rs`
- `.aad/escalations/` ディレクトリ

## 備考

- 非同期処理には `tokio` を使用する
- 並列実行数の上限は設定可能とする（デフォルト: CPU コア数）
- エスカレーションログは JSON 形式で保存する
- タイムアウト時間は `config/aad.toml` で設定可能とする

---

**最終更新**: 2026-01-18
