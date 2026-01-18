# Phase 8: 品質ゲート + GitHub連携 - 要件定義

## プロジェクト概要

**目標**: 品質チェックと GitHub PR 連携を実装し、完全な自動化を実現する。

**期間**: 1週間

**依存関係**: Phase 1-7 完了（全機能が必要）

## 要件

### REQ-1: QualityService 実装

**要件文 (EARS形式)**:
The system shall 品質チェックロジックを実装し、フェーズごとのゲート条件を検証すること。

**受入基準 (AC)**:
- AC-1.1: `application/src/quality/quality_service.rs` に `QualityService` が定義されている
- AC-1.2: `check_phase_gate(phase)` メソッドが実装されている
- AC-1.3: SPEC フェーズのゲート条件（受入基準がテスト可能か）が検証される
- AC-1.4: TASKS フェーズのゲート条件（全タスクに ID が付与されているか）が検証される
- AC-1.5: TDD フェーズのゲート条件（テスト成功、カバレッジ80%以上、Lint通過）が検証される
- AC-1.6: 検証結果がレポート形式で返される

**優先度**: Must

---

### REQ-2: gate コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad gate` を実行した場合、the system shall 品質ゲートチェックを実行し、結果を表示すること。

**受入基準 (AC)**:
- AC-2.1: `cli/src/commands/gate.rs` が実装されている
- AC-2.2: `aad gate` で現在フェーズのゲートチェックが実行される
- AC-2.3: `aad gate --phase TDD` で特定フェーズのチェックが実行される
- AC-2.4: チェック結果が色分けされて表示される（✅ 成功、❌ 失敗）
- AC-2.5: 失敗時に修正アクションが提案される
- AC-2.6: `--approve` オプションで人間承認が記録される

**優先度**: Must

---

### REQ-3: GhCliAdapter 実装

**要件文 (EARS形式)**:
The system shall GitHub CLI (`gh` コマンド) のラッパーを実装し、PR 作成・マージ・Issue 操作を提供すること。

**受入基準 (AC)**:
- AC-3.1: `infrastructure/src/github/gh_cli_adapter.rs` が実装されている
- AC-3.2: `create_pr(title, body)` メソッドが実装されている
- AC-3.3: `merge_pr(pr_number)` メソッドが実装されている
- AC-3.4: `create_issue(title, body, labels)` メソッドが実装されている
- AC-3.5: `gh` コマンドが存在しない場合にエラーを返す
- AC-3.6: GitHub CLI の認証状態を確認する

**優先度**: Must

---

### REQ-4: integrate コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad integrate` を実行した場合、the system shall Draft PR を作成し、CI ステータスを確認すること。

**受入基準 (AC)**:
- AC-4.1: `cli/src/commands/integrate.rs` が実装されている
- AC-4.2: `aad integrate` で Draft PR が作成される
- AC-4.3: PR タイトルが自動生成される（例: `[SPEC-001] ユーザー認証機能の実装`）
- AC-4.4: PR 本文に Spec サマリーが含まれる
- AC-4.5: CI ステータスがポーリングされる
- AC-4.6: CI が green になったら通知が表示される

**優先度**: Must

---

### REQ-5: retro コマンド実装

**要件文 (EARS形式)**:
When ユーザーが `aad retro` を実行した場合、the system shall 振り返りログを作成し、CLAUDE.md 更新提案を生成すること。

**受入基準 (AC)**:
- AC-5.1: `cli/src/commands/retro.rs` が実装されている
- AC-5.2: `aad retro` で振り返りテンプレートが表示される
- AC-5.3: `.aad/retrospectives/<timestamp>-<spec-id>.md` にログが保存される
- AC-5.4: CLAUDE.md への更新提案が生成される
- AC-5.5: 学びの蓄積セクションに追記される内容がプレビューされる

**優先度**: Should

---

### REQ-6: CI/CD 設定実装

**要件文 (EARS形式)**:
The system shall GitHub Actions ワークフローを定義し、自動テストと品質チェックを実行すること。

**受入基準 (AC)**:
- AC-6.1: `.github/workflows/ci.yml` が存在する
- AC-6.2: `cargo build`, `cargo test`, `cargo clippy` が自動実行される
- AC-6.3: テストカバレッジが計測される
- AC-6.4: PR 作成時に CI が自動実行される
- AC-6.5: CI 結果が PR にコメントされる

**優先度**: Must

---

## 完了条件

Phase 8 は以下の条件をすべて満たした場合に完了とする:

1. ✅ 品質ゲートが全フェーズで動作する
2. ✅ PR 作成・マージが自動化されている
3. ✅ CI/CD が正しく動作する
4. ✅ 振り返りログが記録される
5. ✅ `cargo test --all` が全て pass する
6. ✅ すべての要件（REQ-1 〜 REQ-6）の受入基準が満たされている

## 成果物

- `crates/application/src/quality/` モジュール
  - `quality_service.rs`
  - `gate_checker.rs`
- `crates/infrastructure/src/github/` モジュール
  - `gh_cli_adapter.rs`
- `crates/cli/src/commands/`
  - `gate.rs`
  - `integrate.rs`
  - `retro.rs`
- `.github/workflows/ci.yml`
- `.aad/retrospectives/` ディレクトリ

## 備考

- GitHub CLI のバージョンは 2.0 以上を要求する
- CI/CD には `actions-rs` アクションを使用する
- カバレッジ計測には `cargo-llvm-cov` を使用する
- 品質ゲート基準は `config/aad.toml` で設定可能とする
- PR テンプレートは `.github/pull_request_template.md` で定義する

---

**最終更新**: 2026-01-18
