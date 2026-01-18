# SPEC-006: TUIダッシュボード

**作成日**: 2026-01-18

**担当者**: Claude Code

**ステータス**: Approved

**関連Issue**: N/A

---

## 📋 概要

Ratatui によるリアルタイムダッシュボードを実装し、視覚的に進捗を把握できるようにする。セッション一覧、タスク進捗バー、Spec ツリー、コンテキスト使用率、フェーズインディケーターなどのWidgetを提供し、キーボード操作による直感的なUIを実現する。

---

## 🎯 目的

### ビジネス目標
リアルタイムダッシュボードにより、開発プロセスの可視化を実現し、進捗状況を一目で把握できるようにする。70%ルールに基づくコンテキスト使用率の可視化により、適切なタイミングでのセッション管理を支援する。

### ユーザーストーリー
```
As a 開発者
I want to TUIダッシュボードで開発プロセスの進捗をリアルタイムに確認する
So that 複数のタスクやセッションの状態を視覚的に把握し、効率的に作業を進められる
```

---

## 🔍 要件定義（MoSCoW）

### Must Have（必須）
これらがないとリリースできない機能

- [ ] **REQ-1: tui クレート作成** - `crates/tui/` ディレクトリに tui クレートを作成し、TUI 基盤を提供する。`ratatui` (v0.28) と `crossterm` を依存に含める。
- [ ] **REQ-2: App 構造体・状態管理実装** - TUI アプリケーション状態を管理する `App` 構造体を実装する。現在の画面（`View`）を保持し、`update()` と `render()` メソッドで状態更新と画面描画を行う。
- [ ] **REQ-3: Widgets 実装** - `SessionList`, `TaskProgress`, `SpecTree`, `ContextBar`, `PhaseIndicator` の5つのWidgetを実装する。各Widgetは `ratatui::widgets::Widget` トレイトを実装する。
- [ ] **REQ-4: Views 実装** - `Dashboard`, `Monitor`, `Workflow`, `Detail` の4つのViewを実装する。各Viewは適切なWidgetを組み合わせて表示する。
- [ ] **REQ-5: キーボードイベント処理実装** - キーボード操作（`q`: 終了、`Tab`: View切り替え、`↑↓`: リスト選択、`Enter`: 詳細画面遷移、`Esc`: 戻る）を実装する。
- [ ] **REQ-6: monitor コマンド連携** - `aad monitor` コマンドでTUIダッシュボードを起動し、データを1秒ごとに定期更新する。`Ctrl+C` または `q` キーで正常終了できる。

### Should Have（重要）
できるだけ含めるべき機能

- なし

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- なし

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] **マウス操作** - 理由: キーボード操作に集中
- [ ] **カスタムテーマエディタ** - 理由: 設定ファイルで十分

---

## 🎨 UI/UX要件

### 画面構成

#### Dashboard View
```
┌─ AAD Dashboard ─────────────────────────────────┐
│ Context: ████████░░░░░░░░░░ 42%                 │
│ Phase: [SPEC] → TASKS → TDD → REVIEW → RETRO    │
├─────────────────────────────────────────────────┤
│ Active Sessions                                  │
│ ┌───────────────────────────────────────────┐  │
│ │ ✓ SPEC-001  [TDD]        Progress: 65%   │  │
│ │ ⏳ SPEC-002  [TASKS]      Progress: 30%   │  │
│ │ ❌ SPEC-003  [FAILED]     Error: tests    │  │
│ └───────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│ [Tab] Switch View  [q] Quit  [↑↓] Navigate     │
└─────────────────────────────────────────────────┘
```

#### Monitor View
```
┌─ Monitor ────────────────────────────────────────┐
│ SPEC-001: ユーザー認証機能                       │
│ ┌───────────────────────────────────────────┐  │
│ │ Task-01 [✓] ████████████████████ 100%     │  │
│ │ Task-02 [⏳] ██████████░░░░░░░░░  50%     │  │
│ │ Task-03 [ ] ░░░░░░░░░░░░░░░░░░░░   0%     │  │
│ └───────────────────────────────────────────┘  │
│                                                  │
│ Last Update: 2026-01-18 10:30:45                │
└─────────────────────────────────────────────────┘
```

### 主要な操作フロー
1. `aad monitor` でダッシュボードを起動
2. `Tab` キーでView切り替え（Dashboard ⇄ Monitor ⇄ Workflow ⇄ Detail）
3. `↑` `↓` キーでリスト選択
4. `Enter` キーで詳細画面に遷移
5. `Esc` キーで前の画面に戻る
6. `q` キーで終了

### レスポンシブ対応
- 最小ターミナルサイズ: 80x24
- ウィンドウサイズに応じてレイアウト調整

---

## 🔧 技術要件

### フロントエンド
N/A

### バックエンド
- **言語**: Rust (Edition 2021)
- **TUIフレームワーク**: ratatui 0.28
- **ターミナルバックエンド**: crossterm
- **更新頻度**: 1秒（設定可能）
- **カラースキーム**: ダークテーマ（設定可能）

### データベース
N/A

### パフォーマンス要件
- 画面描画時間: 16ms以内（60fps）
- イベント処理時間: 10ms以内
- 差分更新による最適化

---

## 📊 データモデル

### TUI構造

#### `App` 構造体

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| current_view | View | 現在の画面 |
| session_data | Vec<SessionInfo> | セッション情報 |
| selected_index | usize | 選択中のインデックス |
| should_quit | bool | 終了フラグ |

#### `View` enum
画面種別
- `Dashboard`: メイン画面
- `Monitor`: 監視画面
- `Workflow`: ワークフロー画面
- `Detail`: 詳細画面

#### `SessionInfo` 構造体

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| spec_id | SpecId | 仕様ID |
| phase | Phase | 現在のフェーズ |
| progress | f32 | 進捗率（0.0-1.0） |
| status | Status | ステータス |

---

## 🔗 API仕様

### App API

```rust
impl App {
    /// 新しいAppを作成
    pub fn new() -> Self;

    /// 状態を更新
    pub fn update(&mut self);

    /// 画面を描画
    pub fn render(&mut self, frame: &mut Frame);

    /// イベントを処理
    pub fn handle_event(&mut self, event: Event) -> Result<(), Error>;

    /// 終了すべきか
    pub fn should_quit(&self) -> bool;
}
```

### Widget API

```rust
// SessionList Widget
impl Widget for SessionList {
    fn render(self, area: Rect, buf: &mut Buffer);
}

// TaskProgress Widget
impl Widget for TaskProgress {
    fn render(self, area: Rect, buf: &mut Buffer);
}

// 他のWidgetも同様
```

---

## ✅ 受け入れ基準

テスト可能な形式で記述すること

### 機能テスト

#### REQ-1: tui クレート作成
- [ ] AC-1.1: `crates/tui/Cargo.toml` が存在する
- [ ] AC-1.2: `ratatui` クレート（v0.28）が依存に含まれている
- [ ] AC-1.3: `crossterm` クレートが依存に含まれている
- [ ] AC-1.4: `cargo build -p tui` が成功する

#### REQ-2: App 構造体・状態管理実装
- [ ] AC-2.1: `tui/src/app.rs` に `App` 構造体が定義されている
- [ ] AC-2.2: 現在の画面（`View`）を保持する状態フィールドがある
- [ ] AC-2.3: `update()` メソッドで状態更新が行われる
- [ ] AC-2.4: `render()` メソッドで画面描画が行われる
- [ ] AC-2.5: 画面遷移ロジックが実装されている

#### REQ-3: Widgets 実装
- [ ] AC-3.1: `tui/src/widgets/session_list.rs` が実装されている
- [ ] AC-3.2: `tui/src/widgets/task_progress.rs` が実装されている
- [ ] AC-3.3: `tui/src/widgets/spec_tree.rs` が実装されている
- [ ] AC-3.4: `tui/src/widgets/context_bar.rs` が実装されている（70%ルールの可視化）
- [ ] AC-3.5: `tui/src/widgets/phase_indicator.rs` が実装されている
- [ ] AC-3.6: 各 Widget が `ratatui::widgets::Widget` トレイトを実装している

#### REQ-4: Views 実装
- [ ] AC-4.1: `tui/src/views/dashboard.rs` が実装されている
- [ ] AC-4.2: `tui/src/views/monitor.rs` が実装されている
- [ ] AC-4.3: `tui/src/views/workflow.rs` が実装されている
- [ ] AC-4.4: `tui/src/views/detail.rs` が実装されている
- [ ] AC-4.5: 各 View が適切な Widget を組み合わせて表示している

#### REQ-5: キーボードイベント処理実装
- [ ] AC-5.1: `tui/src/events.rs` にイベントハンドラが実装されている
- [ ] AC-5.2: `q` キーで終了できる
- [ ] AC-5.3: `Tab` キーで View が切り替わる
- [ ] AC-5.4: `↑` `↓` キーでリストが選択できる
- [ ] AC-5.5: `Enter` キーで詳細画面に遷移できる
- [ ] AC-5.6: `Esc` キーで前の画面に戻れる

#### REQ-6: monitor コマンド連携
- [ ] AC-6.1: `cli/src/commands/monitor.rs` が実装されている
- [ ] AC-6.2: `aad monitor` 実行で TUI が起動する
- [ ] AC-6.3: 1秒ごとにセッション状態が更新される
- [ ] AC-6.4: リアルタイムで進捗が反映される
- [ ] AC-6.5: `Ctrl+C` または `q` キーで正常終了できる

### 非機能テスト
- [ ] `aad monitor` でダッシュボードが起動する
- [ ] リアルタイム更新が動作する
- [ ] キーボード操作が快適である
- [ ] UI が見やすく整理されている
- [ ] `cargo test -p tui` が全て pass する

### セキュリティ
N/A（ローカルツール）

---

## 🚧 制約・前提条件

### 技術的制約
- `ratatui` のバージョンは 0.28 を使用する
- ターミナルバックエンドには `crossterm` を使用する
- カラースキームは設定可能とする（デフォルト: ダークテーマ）
- 更新頻度は設定可能とする（デフォルト: 1秒）
- パフォーマンスを考慮し、差分更新を実装する

### ビジネス制約
- 期間: 2週間

### 依存関係
- SPEC-001（Domain基盤）の完了が前提
- SPEC-002（設定管理 + ワークフロー）の完了が前提
- SPEC-003（CLI基本コマンド）の完了が前提
- SPEC-004（オーケストレーション）の完了が前提
- SPEC-005（永続化）の完了が前提

---

## 🔄 マイグレーション計画

### 既存データへの影響
N/A（新規機能）

### ロールバック計画
N/A（新規機能）

---

## 📚 参考資料

- [ratatui公式ドキュメント](https://docs.rs/ratatui/)
- [crossterm公式ドキュメント](https://docs.rs/crossterm/)
- [SPEC-001: Domain基盤](./SPEC-001.md)
- [SPEC-002: 設定管理 + ワークフロー](./SPEC-002.md)
- [SPEC-003: CLI基本コマンド](./SPEC-003.md)
- [SPEC-004: オーケストレーション](./SPEC-004.md)
- [SPEC-005: 永続化](./SPEC-005.md)

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

- [ ] 技術レビュー完了（担当: 未定、日付: 未定）
- [ ] ビジネスレビュー完了（担当: 未定、日付: 未定）
- [ ] 最終承認（担当: 未定、日付: 未定）

---

**注意**: この仕様書は承認後にタスク分割（`/aad:tasks SPEC-006`）を実行してください。
