# Phase 7: タスクループ + 完了検出 - 要件定義

## プロジェクト概要

**目標**: ralph-tui 相当のタスクループ機能を実装し、タスクの自動進行を実現する。

**期間**: 1週間

**依存関係**: Phase 1, 2, 3, 4, 5 完了（セッション管理が必要）

## 要件

### REQ-1: LoopEngine 実装

**要件文 (EARS形式)**:
The system shall `LoopEngine` を実装し、タスクループのメインロジックとタスクキュー管理を提供すること。

**受入基準 (AC)**:
- AC-1.1: `application/src/loop_engine/mod.rs` に `LoopEngine` 構造体が定義されている
- AC-1.2: `run_loop(spec_id)` メソッドが実装されている
- AC-1.3: タスクキュー（`VecDeque<TaskId>`）が管理されている
- AC-1.4: 現在実行中のタスクが追跡される
- AC-1.5: ループの中断・再開が可能である

**優先度**: Must

---

### REQ-2: 完了パターン検出実装

**要件文 (EARS形式)**:
When タスクが完了した場合、the system shall `completion-patterns.json` に基づいて完了を検出すること。

**受入基準 (AC)**:
- AC-2.1: `config/completion-patterns.json` ファイルが存在する
- AC-2.2: `application/src/loop_engine/completion_detector.rs` が実装されている
- AC-2.3: 正規表現パターンマッチングが動作する
- AC-2.4: 完了メッセージ（例: `"Task completed successfully"`）が検出される
- AC-2.5: 複数パターンの OR 条件が設定可能である
- AC-2.6: パターンにマッチしない場合は未完了と判定される

**優先度**: Must

---

### REQ-3: 自動次タスク進行実装

**要件文 (EARS形式)**:
When タスクが完了検出された場合、the system shall 依存関係を考慮して次のタスクを自動的に開始すること。

**受入基準 (AC)**:
- AC-3.1: `next_task()` メソッドが実装されている
- AC-3.2: 依存タスクが完了している場合のみ次タスクが開始される
- AC-3.3: 依存タスクが未完了の場合はスキップされる
- AC-3.4: 全タスク完了時にループが終了する
- AC-3.5: リトライ機能が実装されている（最大3回）

**優先度**: Must

---

### REQ-4: loop コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad loop <spec-id>` を実行した場合、the system shall タスクループを開始すること。

**受入基準 (AC)**:
- AC-4.1: `cli/src/commands/loop.rs` が実装されている
- AC-4.2: `aad loop SPEC-001` でループが開始される
- AC-4.3: `aad loop SPEC-001 --resume` で中断したループを再開できる
- AC-4.4: `Ctrl+C` で中断できる（状態は保存される）
- AC-4.5: ループ進捗がコンソールに表示される

**優先度**: Must

---

### REQ-5: ループ状態の可視化

**要件文 (EARS形式)**:
When ループが実行中の場合、the system shall TUI ダッシュボードに進捗を表示すること。

**受入基準 (AC)**:
- AC-5.1: `aad monitor` でループ状態が確認できる
- AC-5.2: 現在実行中のタスクがハイライト表示される
- AC-5.3: タスク進捗率が表示される
- AC-5.4: 完了・失敗・スキップが色分けされる（緑・赤・黄）
- AC-5.5: リアルタイムで状態が更新される

**優先度**: Should

---

## 完了条件

Phase 7 は以下の条件をすべて満たした場合に完了とする:

1. ✅ タスクループが自動進行する
2. ✅ 完了検出が正しく動作する
3. ✅ 依存関係が考慮される
4. ✅ エラー時の中断が適切である
5. ✅ `cargo test -p application` が全て pass する
6. ✅ すべての要件（REQ-1 〜 REQ-5）の受入基準が満たされている

## 成果物

- `crates/application/src/loop_engine/` モジュール
  - `mod.rs` (`LoopEngine`)
  - `completion_detector.rs`
  - `task_queue.rs`
- `crates/cli/src/commands/loop.rs`
- `config/completion-patterns.json` (サンプル)

## 備考

- 完了パターンには正規表現ライブラリ `regex` を使用する
- タスクループの状態は `.aad/loop-state.json` に保存する
- リトライ回数とタイムアウト時間は `config/aad.toml` で設定可能とする
- 完了パターンはタスクごとにカスタマイズ可能とする

---

**最終更新**: 2026-01-18
