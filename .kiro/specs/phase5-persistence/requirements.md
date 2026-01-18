# Phase 5: セッション管理 + 永続化 - 要件定義

## プロジェクト概要

**目標**: セッション状態の永続化とスタイル管理を実装し、処理の中断・再開を可能にする。

**期間**: 1週間

**依存関係**: Phase 1, 2 完了（Domain + Config 層が必要）

## 要件

### REQ-1: persistence/ モジュール実装

**要件文 (EARS形式)**:
The system shall `infrastructure/persistence/` モジュールを作成し、JSON ファイル I/O の基盤を提供すること。

**受入基準 (AC)**:
- AC-1.1: `infrastructure/src/persistence/` ディレクトリが存在する
- AC-1.2: `serde_json` クレートが依存に含まれている
- AC-1.3: `FileStore` トレイトが定義されている
- AC-1.4: 共通のエラー型が定義されている

**優先度**: Must

---

### REQ-2: JSON Repository 実装

**要件文 (EARS形式)**:
The system shall `SpecRepository`, `TaskRepository`, `SessionRepository` の具象実装を JSON ファイルとして提供すること。

**受入基準 (AC)**:
- AC-2.1: `SpecJsonRepo` が `SpecRepository` トレイトを実装している
- AC-2.2: `TaskJsonRepo` が `TaskRepository` トレイトを実装している
- AC-2.3: `SessionJsonRepo` が `SessionRepository` トレイトを実装している
- AC-2.4: `find_by_id()`, `save()`, `delete()` メソッドが動作する
- AC-2.5: JSON ファイルが `.aad/data/` ディレクトリに保存される

**優先度**: Must

---

### REQ-3: StyleFileAdapter 実装

**要件文 (EARS形式)**:
When スタイルが適用される場合、the system shall `CLAUDE.md` ファイルにスタイルセクションを書き込むこと。

**受入基準 (AC)**:
- AC-3.1: `infrastructure/src/persistence/style_file_adapter.rs` が実装されている
- AC-3.2: `write_style(style_name, tokens)` メソッドが実装されている
- AC-3.3: マーカー `<!-- AAD_STYLE:BEGIN -->` と `<!-- AAD_STYLE:END -->` が使用される
- AC-3.4: 既存のスタイルセクションが正しく置換される
- AC-3.5: マーカーが存在しない場合は末尾に追加される

**優先度**: Must

---

### REQ-4: TokenReplacer 実装

**要件文 (EARS形式)**:
The system shall トークン置換ロジックを実装し、`{{token}}` を実際の値に置換すること。

**受入基準 (AC)**:
- AC-4.1: `infrastructure/src/persistence/token_replacer.rs` が実装されている
- AC-4.2: `replace(template, token_map)` メソッドが実装されている
- AC-4.3: `{{role}}` → `賢者` のような置換が動作する
- AC-4.4: 未定義トークンの場合はエラーを返す
- AC-4.5: ネストされたトークンは非対応（エラー）

**優先度**: Must

---

### REQ-5: BackupAdapter 実装

**要件文 (EARS形式)**:
When ファイルが更新される場合、the system shall 更新前のファイルをバックアップすること。

**受入基準 (AC)**:
- AC-5.1: `infrastructure/src/persistence/backup_adapter.rs` が実装されている
- AC-5.2: `backup(file_path)` メソッドが実装されている
- AC-5.3: バックアップファイルが `.aad/backups/` に保存される
- AC-5.4: ファイル名に ISO 8601 形式のタイムスタンプが含まれる（例: `CLAUDE.md.2026-01-18T12-30-45.bak`）
- AC-5.5: 古いバックアップの自動削除機能がある（デフォルト: 10世代保持）

**優先度**: Must

---

### REQ-6: persist コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad persist save` または `aad persist restore` を実行した場合、the system shall 現在状態の保存または過去状態の復元を行うこと。

**受入基準 (AC)**:
- AC-6.1: `cli/src/commands/persist.rs` が実装されている
- AC-6.2: `aad persist save` で全セッション状態が保存される
- AC-6.3: `aad persist restore <timestamp>` で指定時刻の状態に復元される
- AC-6.4: `aad persist list` でバックアップ一覧が表示される
- AC-6.5: 復元前に確認メッセージが表示される

**優先度**: Should

---

## 完了条件

Phase 5 は以下の条件をすべて満たした場合に完了とする:

1. ✅ セッション状態が保存・復元可能である
2. ✅ スタイル切替が動作する
3. ✅ バックアップが正しく作成される
4. ✅ データ整合性が保たれる
5. ✅ `cargo test -p infrastructure` が全て pass する
6. ✅ すべての要件（REQ-1 〜 REQ-6）の受入基準が満たされている

## 成果物

- `crates/infrastructure/src/persistence/` モジュール
  - `spec_json_repo.rs`
  - `task_json_repo.rs`
  - `session_json_repo.rs`
  - `style_file_adapter.rs`
  - `token_replacer.rs`
  - `backup_adapter.rs`
- `crates/cli/src/commands/persist.rs`
- `.aad/data/` ディレクトリ
- `.aad/backups/` ディレクトリ

## 備考

- JSON シリアライズには `serde_json` を使用する
- ファイル操作には `std::fs` を使用する
- トランザクション機能は Phase 5 では実装しない（将来の拡張として検討）
- バックアップ世代数は `config/aad.toml` で設定可能とする

---

**最終更新**: 2026-01-18
