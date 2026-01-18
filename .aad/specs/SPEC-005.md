# SPEC-005: セッション管理 + 永続化

**作成日**: 2026-01-18

**担当者**: Claude Code

**ステータス**: Approved

**関連Issue**: N/A

---

## 📋 概要

セッション状態の永続化とスタイル管理を実装し、処理の中断・再開を可能にする。JSON ファイル I/O を使用したRepository の具象実装、スタイルファイルアダプター、トークン置換ロジック、バックアップ機能を提供する。

---

## 🎯 目的

### ビジネス目標
セッション状態の永続化により、長時間の開発タスクの中断・再開を可能にし、開発プロセスの柔軟性を向上させる。スタイル管理機能により、出力形式のカスタマイズを実現し、多様なプロジェクト要件に対応する。

### ユーザーストーリー
```
As a 開発者
I want to セッション状態を保存・復元し、スタイルをカスタマイズする
So that 長時間のタスクを安全に中断・再開でき、プロジェクトに適した形式で作業できる
```

---

## 🔍 要件定義（MoSCoW）

### Must Have（必須）
これらがないとリリースできない機能

- [ ] **REQ-1: persistence/ モジュール実装** - `infrastructure/persistence/` モジュールを作成し、JSON ファイル I/O の基盤を提供する。`FileStore` トレイトと共通のエラー型を定義する。
- [ ] **REQ-2: JSON Repository 実装** - `SpecRepository`, `TaskRepository`, `SessionRepository` の具象実装を JSON ファイルとして提供する。`.aad/data/` ディレクトリに保存する。
- [ ] **REQ-3: StyleFileAdapter 実装** - スタイルが適用される際、`CLAUDE.md` ファイルにスタイルセクションを書き込む。マーカー（`<!-- AAD_STYLE:BEGIN -->`, `<!-- AAD_STYLE:END -->`）を使用する。
- [ ] **REQ-4: TokenReplacer 実装** - トークン置換ロジックを実装し、`{{token}}` を実際の値に置換する。未定義トークンの場合はエラーを返す。
- [ ] **REQ-5: BackupAdapter 実装** - ファイルが更新される前に、更新前のファイルをバックアップする。バックアップは `.aad/backups/` に保存し、ISO 8601 形式のタイムスタンプを含める。

### Should Have（重要）
できるだけ含めるべき機能

- [ ] **REQ-6: persist コマンド実装** - `aad persist save` で全セッション状態を保存し、`aad persist restore <timestamp>` で指定時刻の状態に復元する。`aad persist list` でバックアップ一覧を表示する。

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- なし

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] **トランザクション機能** - 理由: 将来の拡張として検討
- [ ] **データベース対応** - 理由: JSON ファイルで十分

---

## 🎨 UI/UX要件

### 画面構成
コマンドラインインターフェース

### 主要な操作フロー

#### 1. セッション状態の保存
```bash
$ aad persist save
✓ セッション状態を保存しました (.aad/data/)
```

#### 2. セッション状態の復元
```bash
$ aad persist list
バックアップ一覧:
  1. 2026-01-18T10:30:00 (SPEC-001, SPEC-002)
  2. 2026-01-18T09:15:00 (SPEC-001)

$ aad persist restore 2026-01-18T10:30:00
⚠ 現在の状態は上書きされます。続行しますか? (y/N): y
✓ セッション状態を復元しました
```

#### 3. スタイル適用
```bash
$ aad style apply expert-mode
✓ スタイル "expert-mode" を CLAUDE.md に適用しました
✓ バックアップを作成しました: .aad/backups/CLAUDE.md.2026-01-18T10-30-45.bak
```

### レスポンシブ対応
N/A

---

## 🔧 技術要件

### フロントエンド
N/A

### バックエンド
- **言語**: Rust (Edition 2021)
- **シリアライズ**: serde_json
- **ファイル操作**: std::fs
- **バックアップ世代管理**: デフォルト10世代保持（設定可能）

### データベース
N/A（JSON ファイルベース）

### パフォーマンス要件
- JSON読み込み時間: 100ms以内
- JSON書き込み時間: 100ms以内
- バックアップ作成時間: 50ms以内

---

## 📊 データモデル

### ファイル配置

```
.aad/
├── data/
│   ├── specs/
│   │   └── SPEC-001.json
│   ├── tasks/
│   │   └── SPEC-001-tasks.json
│   └── sessions/
│       └── session-12345.json
└── backups/
    └── CLAUDE.md.2026-01-18T10-30-45.bak
```

### JSON構造例

#### Spec JSONspec
```json
{
  "id": "SPEC-001",
  "name": "ユーザー認証機能",
  "phase": "TDD",
  "status": "in_progress",
  "created_at": "2026-01-18T00:00:00Z"
}
```

#### Session JSON
```json
{
  "id": "session-12345",
  "spec_id": "SPEC-001",
  "phase": "TDD",
  "started_at": "2026-01-18T10:00:00Z"
}
```

---

## 🔗 API仕様

### Repository API

```rust
// SpecJsonRepo
impl SpecRepository for SpecJsonRepo {
    fn find_by_id(&self, id: &SpecId) -> Result<Option<Spec>, Error>;
    fn save(&self, spec: &Spec) -> Result<(), Error>;
    fn delete(&self, id: &SpecId) -> Result<(), Error>;
}

// TaskJsonRepo, SessionJsonRepo も同様
```

### StyleFileAdapter API

```rust
impl StyleFileAdapter {
    /// スタイルを CLAUDE.md に書き込む
    pub fn write_style(&self, style_name: &str, tokens: &TokenMap) -> Result<(), Error>;
}
```

### TokenReplacer API

```rust
impl TokenReplacer {
    /// トークンを置換する
    pub fn replace(&self, template: &str, token_map: &TokenMap) -> Result<String, Error>;
}
```

### BackupAdapter API

```rust
impl BackupAdapter {
    /// ファイルをバックアップする
    pub fn backup(&self, file_path: &Path) -> Result<PathBuf, Error>;

    /// 古いバックアップを削除する
    pub fn cleanup_old_backups(&self, keep_count: usize) -> Result<(), Error>;
}
```

---

## ✅ 受け入れ基準

テスト可能な形式で記述すること

### 機能テスト

#### REQ-1: persistence/ モジュール実装
- [ ] AC-1.1: `infrastructure/src/persistence/` ディレクトリが存在する
- [ ] AC-1.2: `serde_json` クレートが依存に含まれている
- [ ] AC-1.3: `FileStore` トレイトが定義されている
- [ ] AC-1.4: 共通のエラー型が定義されている

#### REQ-2: JSON Repository 実装
- [ ] AC-2.1: `SpecJsonRepo` が `SpecRepository` トレイトを実装している
- [ ] AC-2.2: `TaskJsonRepo` が `TaskRepository` トレイトを実装している
- [ ] AC-2.3: `SessionJsonRepo` が `SessionRepository` トレイトを実装している
- [ ] AC-2.4: `find_by_id()`, `save()`, `delete()` メソッドが動作する
- [ ] AC-2.5: JSON ファイルが `.aad/data/` ディレクトリに保存される

#### REQ-3: StyleFileAdapter 実装
- [ ] AC-3.1: `infrastructure/src/persistence/style_file_adapter.rs` が実装されている
- [ ] AC-3.2: `write_style(style_name, tokens)` メソッドが実装されている
- [ ] AC-3.3: マーカー `<!-- AAD_STYLE:BEGIN -->` と `<!-- AAD_STYLE:END -->` が使用される
- [ ] AC-3.4: 既存のスタイルセクションが正しく置換される
- [ ] AC-3.5: マーカーが存在しない場合は末尾に追加される

#### REQ-4: TokenReplacer 実装
- [ ] AC-4.1: `infrastructure/src/persistence/token_replacer.rs` が実装されている
- [ ] AC-4.2: `replace(template, token_map)` メソッドが実装されている
- [ ] AC-4.3: `{{role}}` → `賢者` のような置換が動作する
- [ ] AC-4.4: 未定義トークンの場合はエラーを返す
- [ ] AC-4.5: ネストされたトークンは非対応（エラー）

#### REQ-5: BackupAdapter 実装
- [ ] AC-5.1: `infrastructure/src/persistence/backup_adapter.rs` が実装されている
- [ ] AC-5.2: `backup(file_path)` メソッドが実装されている
- [ ] AC-5.3: バックアップファイルが `.aad/backups/` に保存される
- [ ] AC-5.4: ファイル名に ISO 8601 形式のタイムスタンプが含まれる（例: `CLAUDE.md.2026-01-18T12-30-45.bak`）
- [ ] AC-5.5: 古いバックアップの自動削除機能がある（デフォルト: 10世代保持）

#### REQ-6: persist コマンド実装
- [ ] AC-6.1: `cli/src/commands/persist.rs` が実装されている
- [ ] AC-6.2: `aad persist save` で全セッション状態が保存される
- [ ] AC-6.3: `aad persist restore <timestamp>` で指定時刻の状態に復元される
- [ ] AC-6.4: `aad persist list` でバックアップ一覧が表示される
- [ ] AC-6.5: 復元前に確認メッセージが表示される

### 非機能テスト
- [ ] セッション状態が保存・復元可能である
- [ ] スタイル切替が動作する
- [ ] バックアップが正しく作成される
- [ ] データ整合性が保たれる
- [ ] `cargo test -p infrastructure` が全て pass する

### セキュリティ
- [ ] ファイルパストラバーサル対策が実装されている
- [ ] JSONデシリアライズ時のDoS攻撃対策が実装されている

---

## 🚧 制約・前提条件

### 技術的制約
- JSON シリアライズには `serde_json` を使用する
- ファイル操作には `std::fs` を使用する
- トランザクション機能は Phase 5 では実装しない（将来の拡張として検討）
- バックアップ世代数は `config/aad.toml` で設定可能とする

### ビジネス制約
- 期間: 1週間

### 依存関係
- SPEC-001（Domain基盤）の完了が前提
- SPEC-002（設定管理 + ワークフロー）の完了が前提

---

## 🔄 マイグレーション計画

### 既存データへの影響
N/A（新規機能）

### ロールバック計画
バックアップ機能により、任意の時点の状態に復元可能。

---

## 📚 参考資料

- [serde_json公式ドキュメント](https://docs.rs/serde_json/)
- [Rustファイル操作](https://doc.rust-lang.org/std/fs/)
- [SPEC-001: Domain基盤](./SPEC-001.md)
- [SPEC-002: 設定管理 + ワークフロー](./SPEC-002.md)

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

**注意**: この仕様書は承認後にタスク分割（`/aad:tasks SPEC-005`）を実行してください。
