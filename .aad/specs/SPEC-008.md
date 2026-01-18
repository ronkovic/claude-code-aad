# SPEC-008: 品質ゲート + GitHub連携

**作成日**: 2026-01-18

**担当者**: Claude Code

**ステータス**: Approved

**関連Issue**: N/A

---

## 📋 概要

品質チェックと GitHub PR 連携を実装し、完全な自動化を実現する。フェーズごとの品質ゲート条件を検証し、GitHub CLI を使用した PR 作成・マージ・Issue 操作を提供する。振り返りログの作成により、開発プロセスの継続的改善を支援する。

---

## 🎯 目的

### ビジネス目標
品質ゲートにより、各フェーズでの品質基準を明確化し、不具合の早期発見を実現する。GitHub 連携により、開発フローの完全自動化を達成し、人間の介入を最小化する。振り返りによる学びの蓄積で、開発プロセスの継続的改善を図る。

### ユーザーストーリー
```
As a 開発者
I want to 品質ゲートで自動チェックを行い、GitHub連携で自動マージする
So that 品質を保ちながら、開発プロセス全体を効率化できる
```

---

## 🔍 要件定義（MoSCoW）

### Must Have（必須）
これらがないとリリースできない機能

- [ ] **REQ-1: QualityService 実装** - 品質チェックロジックを実装し、フェーズごとのゲート条件を検証する。SPEC、TASKS、TDD の各フェーズのゲート条件をチェックし、検証結果をレポート形式で返す。
- [ ] **REQ-2: gate コマンド実装** - `aad gate` で品質ゲートチェックを実行し、結果を色分けして表示する。`--approve` オプションで人間承認を記録する。
- [ ] **REQ-3: GhCliAdapter 実装** - GitHub CLI (`gh` コマンド) のラッパーを実装し、PR 作成・マージ・Issue 操作を提供する。`gh` コマンドの存在確認と認証状態確認を含む。
- [ ] **REQ-4: integrate コマンド実装** - `aad integrate` で Draft PR を作成し、CI ステータスをポーリングする。CI が green になったら通知を表示する。
- [ ] **REQ-6: CI/CD 設定実装** - GitHub Actions ワークフローを定義し、`cargo build`, `cargo test`, `cargo clippy` を自動実行する。テストカバレッジを計測し、CI 結果を PR にコメントする。

### Should Have（重要）
できるだけ含めるべき機能

- [ ] **REQ-5: retro コマンド実装** - `aad retro` で振り返りログを作成し、CLAUDE.md 更新提案を生成する。学びの蓄積セクションに追記される内容をプレビューする。

### Could Have（あれば良い）
リソースに余裕があれば追加する機能

- なし

### Won't Have（対象外）
今回は実装しないことを明示

- [ ] **自動デプロイ** - 理由: 本番デプロイは人間が判断
- [ ] **Slack通知** - 理由: GitHub通知で十分

---

## 🎨 UI/UX要件

### 画面構成
コマンドラインインターフェース

### 主要な操作フロー

#### 1. 品質ゲートチェック
```bash
$ aad gate
🔍 品質ゲートチェック (TDDフェーズ)

✅ テスト成功: 全てのテストがパスしました
✅ カバレッジ: 85% (目標: 80%以上)
✅ Lint通過: エラーなし
✅ ビルド成功

✓ 全ての品質基準を満たしています

$ aad gate --approve
✓ 人間承認を記録しました
```

#### 2. PR作成・統合
```bash
$ aad integrate
📝 Draft PR を作成中...
✓ PR #42 を作成しました: [SPEC-001] ユーザー認証機能の実装
🔗 https://github.com/user/repo/pull/42

⏳ CI ステータスを確認中...
  - Build: ✅ 成功
  - Test: ✅ 成功
  - Lint: ✅ 成功

✓ CI が全て green になりました
✓ PR を Ready for Review に変更しました
```

#### 3. 振り返り
```bash
$ aad retro
📝 振り返りを作成します

> 何をしていましたか?
ユーザー認証機能の実装

> 何が問題でしたか?
テストデータのクリーンアップ漏れ

> どう解決しましたか?
afterEachフックでデータベースをクリーンアップ

✓ 振り返りログを保存しました: .aad/retrospectives/2026-01-18-SPEC-001.md
✓ CLAUDE.md 更新提案を生成しました
```

### レスポンシブ対応
N/A

---

## 🔧 技術要件

### フロントエンド
N/A

### バックエンド
- **言語**: Rust (Edition 2021)
- **GitHub CLI**: `gh` コマンド (v2.0以上)
- **CI/CD**: GitHub Actions
- **カバレッジツール**: cargo-llvm-cov

### データベース
N/A

### パフォーマンス要件
- 品質チェック実行時間: 10秒以内
- CI ステータス取得時間: 1秒以内

---

## 📊 データモデル

### 品質ゲート関連構造体

#### `QualityGate` 構造体

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| phase | Phase | フェーズ |
| checks | Vec<QualityCheck> | チェック項目 |

#### `QualityCheck` 構造体

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| name | String | チェック名 |
| status | CheckStatus | ステータス（Pass/Fail） |
| message | String | メッセージ |

#### `PullRequest` 構造体

| フィールド名 | 型 | 説明 |
|------------|-----|------|
| number | u32 | PR番号 |
| title | String | タイトル |
| url | String | URL |
| status | PrStatus | ステータス |

---

## 🔗 API仕様

### QualityService API

```rust
impl QualityService {
    /// フェーズゲートをチェック
    pub fn check_phase_gate(&self, phase: Phase) -> QualityGateResult;

    /// 承認を記録
    pub fn record_approval(&self, phase: Phase) -> Result<(), Error>;
}
```

### GhCliAdapter API

```rust
impl GhCliAdapter {
    /// PR を作成
    pub fn create_pr(&self, title: &str, body: &str) -> Result<PullRequest, Error>;

    /// PR をマージ
    pub fn merge_pr(&self, pr_number: u32) -> Result<(), Error>;

    /// Issue を作成
    pub fn create_issue(&self, title: &str, body: &str, labels: Vec<String>) -> Result<u32, Error>;

    /// CI ステータスを取得
    pub fn get_ci_status(&self, pr_number: u32) -> Result<CiStatus, Error>;
}
```

---

## ✅ 受け入れ基準

テスト可能な形式で記述すること

### 機能テスト

#### REQ-1: QualityService 実装
- [ ] AC-1.1: `application/src/quality/quality_service.rs` に `QualityService` が定義されている
- [ ] AC-1.2: `check_phase_gate(phase)` メソッドが実装されている
- [ ] AC-1.3: SPEC フェーズのゲート条件（受入基準がテスト可能か）が検証される
- [ ] AC-1.4: TASKS フェーズのゲート条件（全タスクに ID が付与されているか）が検証される
- [ ] AC-1.5: TDD フェーズのゲート条件（テスト成功、カバレッジ80%以上、Lint通過）が検証される
- [ ] AC-1.6: 検証結果がレポート形式で返される

#### REQ-2: gate コマンド実装
- [ ] AC-2.1: `cli/src/commands/gate.rs` が実装されている
- [ ] AC-2.2: `aad gate` で現在フェーズのゲートチェックが実行される
- [ ] AC-2.3: `aad gate --phase TDD` で特定フェーズのチェックが実行される
- [ ] AC-2.4: チェック結果が色分けされて表示される（✅ 成功、❌ 失敗）
- [ ] AC-2.5: 失敗時に修正アクションが提案される
- [ ] AC-2.6: `--approve` オプションで人間承認が記録される

#### REQ-3: GhCliAdapter 実装
- [ ] AC-3.1: `infrastructure/src/github/gh_cli_adapter.rs` が実装されている
- [ ] AC-3.2: `create_pr(title, body)` メソッドが実装されている
- [ ] AC-3.3: `merge_pr(pr_number)` メソッドが実装されている
- [ ] AC-3.4: `create_issue(title, body, labels)` メソッドが実装されている
- [ ] AC-3.5: `gh` コマンドが存在しない場合にエラーを返す
- [ ] AC-3.6: GitHub CLI の認証状態を確認する

#### REQ-4: integrate コマンド実装
- [ ] AC-4.1: `cli/src/commands/integrate.rs` が実装されている
- [ ] AC-4.2: `aad integrate` で Draft PR が作成される
- [ ] AC-4.3: PR タイトルが自動生成される（例: `[SPEC-001] ユーザー認証機能の実装`）
- [ ] AC-4.4: PR 本文に Spec サマリーが含まれる
- [ ] AC-4.5: CI ステータスがポーリングされる
- [ ] AC-4.6: CI が green になったら通知が表示される

#### REQ-5: retro コマンド実装
- [ ] AC-5.1: `cli/src/commands/retro.rs` が実装されている
- [ ] AC-5.2: `aad retro` で振り返りテンプレートが表示される
- [ ] AC-5.3: `.aad/retrospectives/<timestamp>-<spec-id>.md` にログが保存される
- [ ] AC-5.4: CLAUDE.md への更新提案が生成される
- [ ] AC-5.5: 学びの蓄積セクションに追記される内容がプレビューされる

#### REQ-6: CI/CD 設定実装
- [ ] AC-6.1: `.github/workflows/ci.yml` が存在する
- [ ] AC-6.2: `cargo build`, `cargo test`, `cargo clippy` が自動実行される
- [ ] AC-6.3: テストカバレッジが計測される
- [ ] AC-6.4: PR 作成時に CI が自動実行される
- [ ] AC-6.5: CI 結果が PR にコメントされる

### 非機能テスト
- [ ] 品質ゲートが全フェーズで動作する
- [ ] PR 作成・マージが自動化されている
- [ ] CI/CD が正しく動作する
- [ ] 振り返りログが記録される
- [ ] `cargo test --all` が全て pass する

### セキュリティ
- [ ] GitHub トークンが安全に管理されている
- [ ] CI ログに機密情報が含まれないよう配慮されている

---

## 🚧 制約・前提条件

### 技術的制約
- GitHub CLI のバージョンは 2.0 以上を要求する
- CI/CD には `actions-rs` アクションを使用する
- カバレッジ計測には `cargo-llvm-cov` を使用する
- 品質ゲート基準は `config/aad.toml` で設定可能とする
- PR テンプレートは `.github/pull_request_template.md` で定義する

### ビジネス制約
- 期間: 1週間

### 依存関係
- SPEC-001〜SPEC-007 の全てが前提

---

## 🔄 マイグレーション計画

### 既存データへの影響
N/A（新規機能）

### ロールバック計画
N/A（新規機能）

---

## 📚 参考資料

- [GitHub CLI公式ドキュメント](https://cli.github.com/manual/)
- [GitHub Actions公式ドキュメント](https://docs.github.com/actions)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [SPEC-001: Domain基盤](./SPEC-001.md)
- [SPEC-002: 設定管理 + ワークフロー](./SPEC-002.md)
- [SPEC-003: CLI基本コマンド](./SPEC-003.md)
- [SPEC-004: オーケストレーション](./SPEC-004.md)
- [SPEC-005: 永続化](./SPEC-005.md)
- [SPEC-006: TUIダッシュボード](./SPEC-006.md)
- [SPEC-007: タスクループ](./SPEC-007.md)

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

- [x] 技術レビュー完了（担当: Claude Code、日付: 2026-01-18）
- [x] ビジネスレビュー完了（担当: Claude Code、日付: 2026-01-18）
- [x] 最終承認（担当: User、日付: 2026-01-18）

---

**注意**: この仕様書は承認後にタスク分割（`/aad:tasks SPEC-008`）を実行してください。
