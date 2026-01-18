# Phase 6: TUI ダッシュボード - 要件定義

## プロジェクト概要

**目標**: Ratatui によるリアルタイムダッシュボードを実装し、視覚的に進捗を把握できるようにする。

**期間**: 2週間

**依存関係**: Phase 1, 2, 3, 4, 5 完了（全基盤が必要）

## 要件

### REQ-1: tui クレート作成

**要件文 (EARS形式)**:
The system shall `crates/tui/` ディレクトリに tui クレートを作成し、TUI 基盤を提供すること。

**受入基準 (AC)**:
- AC-1.1: `crates/tui/Cargo.toml` が存在する
- AC-1.2: `ratatui` クレート（v0.28）が依存に含まれている
- AC-1.3: `crossterm` クレートが依存に含まれている
- AC-1.4: `cargo build -p tui` が成功する

**優先度**: Must

---

### REQ-2: App 構造体・状態管理実装

**要件文 (EARS形式)**:
The system shall TUI アプリケーション状態を管理する `App` 構造体を実装すること。

**受入基準 (AC)**:
- AC-2.1: `tui/src/app.rs` に `App` 構造体が定義されている
- AC-2.2: 現在の画面（`View`）を保持する状態フィールドがある
- AC-2.3: `update()` メソッドで状態更新が行われる
- AC-2.4: `render()` メソッドで画面描画が行われる
- AC-2.5: 画面遷移ロジックが実装されている

**優先度**: Must

---

### REQ-3: Widgets 実装

**要件文 (EARS形式)**:
The system shall 以下の Widget を実装すること:
- `SessionList`: セッション一覧
- `TaskProgress`: タスク進捗バー
- `SpecTree`: Spec ツリー表示
- `ContextBar`: コンテキスト使用率
- `PhaseIndicator`: 現在のフェーズ表示

**受入基準 (AC)**:
- AC-3.1: `tui/src/widgets/session_list.rs` が実装されている
- AC-3.2: `tui/src/widgets/task_progress.rs` が実装されている
- AC-3.3: `tui/src/widgets/spec_tree.rs` が実装されている
- AC-3.4: `tui/src/widgets/context_bar.rs` が実装されている（70%ルールの可視化）
- AC-3.5: `tui/src/widgets/phase_indicator.rs` が実装されている
- AC-3.6: 各 Widget が `ratatui::widgets::Widget` トレイトを実装している

**優先度**: Must

---

### REQ-4: Views 実装

**要件文 (EARS形式)**:
The system shall 以下の View を実装すること:
- `Dashboard`: メイン画面
- `Monitor`: 監視画面
- `Workflow`: ワークフロー画面
- `Detail`: 詳細画面

**受入基準 (AC)**:
- AC-4.1: `tui/src/views/dashboard.rs` が実装されている
- AC-4.2: `tui/src/views/monitor.rs` が実装されている
- AC-4.3: `tui/src/views/workflow.rs` が実装されている
- AC-4.4: `tui/src/views/detail.rs` が実装されている
- AC-4.5: 各 View が適切な Widget を組み合わせて表示している

**優先度**: Must

---

### REQ-5: キーボードイベント処理実装

**要件文 (EARS形式)**:
When ユーザーがキーボード操作を行う場合、the system shall 適切なアクションを実行すること。

**受入基準 (AC)**:
- AC-5.1: `tui/src/events.rs` にイベントハンドラが実装されている
- AC-5.2: `q` キーで終了できる
- AC-5.3: `Tab` キーで View が切り替わる
- AC-5.4: `↑` `↓` キーでリストが選択できる
- AC-5.5: `Enter` キーで詳細画面に遷移できる
- AC-5.6: `Esc` キーで前の画面に戻れる

**優先度**: Must

---

### REQ-6: monitor コマンド連携

**要件文 (EARS形式)**:
When ユーザーが `aad monitor` を実行した場合、the system shall TUI ダッシュボードを起動し、データを定期更新すること。

**受入基準 (AC)**:
- AC-6.1: `cli/src/commands/monitor.rs` が実装されている
- AC-6.2: `aad monitor` 実行で TUI が起動する
- AC-6.3: 1秒ごとにセッション状態が更新される
- AC-6.4: リアルタイムで進捗が反映される
- AC-6.5: `Ctrl+C` または `q` キーで正常終了できる

**優先度**: Must

---

## 完了条件

Phase 6 は以下の条件をすべて満たした場合に完了とする:

1. ✅ `aad monitor` でダッシュボードが起動する
2. ✅ リアルタイム更新が動作する
3. ✅ キーボード操作が快適である
4. ✅ UI が見やすく整理されている
5. ✅ `cargo test -p tui` が全て pass する
6. ✅ すべての要件（REQ-1 〜 REQ-6）の受入基準が満たされている

## 成果物

- `crates/tui/` クレート
  - `src/app.rs`
  - `src/events.rs`
  - `src/widgets/session_list.rs`
  - `src/widgets/task_progress.rs`
  - `src/widgets/spec_tree.rs`
  - `src/widgets/context_bar.rs`
  - `src/widgets/phase_indicator.rs`
  - `src/views/dashboard.rs`
  - `src/views/monitor.rs`
  - `src/views/workflow.rs`
  - `src/views/detail.rs`
- `crates/cli/src/commands/monitor.rs`

## 備考

- `ratatui` のバージョンは 0.28 を使用する
- ターミナルバックエンドには `crossterm` を使用する
- カラースキームは設定可能とする（デフォルト: ダークテーマ）
- 更新頻度は設定可能とする（デフォルト: 1秒）
- パフォーマンスを考慮し、差分更新を実装する

---

**最終更新**: 2026-01-18
