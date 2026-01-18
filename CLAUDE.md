# プロジェクト指示書

このファイルはClaude Codeへの指示書です。プロジェクトのルール、学び、制約を記載します。

---

## 📋 プロジェクト概要

**プロジェクト名**: claude-code-aad v2

**目的**: Rust + Ratatui による自律型AI駆動開発ツール

**開始日**: 2026-01-18

**現在のフェーズ**: SPEC

---

## ⚙️ プロジェクト設定

| 設定 | 値 |
|------|-----|
| デフォルトブランチ | `docs/add-implementation-phases` |

**注**: デフォルトブランチは `/aad:init` で自動検出されます。変更する場合はこの表を更新してください。

---

## 🛠️ 技術スタック

### 言語・フレームワーク
- Rust (Edition 2021)
- Ratatui 0.28
- tokio (非同期ランタイム)
- clap (CLI)
- git2 (Git操作)

### テストツール
- cargo test (標準テストフレームワーク)
- cargo-llvm-cov (カバレッジ計測)

### Linter/Formatter
- Clippy
- rustfmt

### その他
- GitHub Actions
- handlebars (テンプレート)
- serde + serde_json (シリアライズ)
- toml (設定ファイル)

---

## 📐 コーディングルール

### 命名規則
- **ファイル名**: snake_case（例: `user_service.rs`）
- **型名**: PascalCase（例: `UserService`）
- **関数名**: snake_case（例: `get_user_by_id`）
- **定数**: UPPER_SNAKE_CASE（例: `MAX_RETRY_COUNT`）

### コードスタイル
- インデント: 4スペース（rustfmt標準）
- 最大行長: 100文字
- rustfmtに準拠

### コメント
- 複雑なロジックには必ずコメントを追加
- TODOコメントには担当者とIssue番号を記載
  ```rust
  // TODO(@username #123): ユーザー認証ロジックを追加
  ```

---

## 💬 コミットメッセージ規約

**Conventional Commits形式を使用**

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type
- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメント変更
- `style`: コードスタイル変更（機能に影響なし）
- `refactor`: リファクタリング
- `test`: テスト追加・修正
- `chore`: ビルド・補助ツール変更

### 例
```
feat(auth): JWT認証機能を追加

- ログインエンドポイントを実装
- トークン検証ミドルウェアを追加

Closes #12
```

---

## 🎯 コンテキスト管理ルール（70%ルール）

| 使用率 | ステータス | アクション |
|--------|------------|------------|
| 0-50% | 🟢 快適 | 通常作業 |
| 50-70% | 🟡 通知：注意 | 大きなタスクは新セッション推奨 |
| 70-85% | 🟠 通知：警告 | `/aad:handoff` 実行推奨 |
| 85-95% | 🔴 通知：危機的 | 即座に `/aad:handoff` |
| 95%+ | ⛔ 通知：限界 | 自動圧縮（制御不能） |

**原則**:
- 70%に達したら作業を区切る
- 複雑なタスクは50%以下で開始する
- `/aad:context`コマンドで定期的に確認

---

## 🚨 エスカレーションルール

### 🔴 即時エスカレーション（作業停止）
- セキュリティ脆弱性の発見
- 本番環境への影響が予想される変更
- データ損失のリスクがある操作
- アーキテクチャ変更が必要な問題

**アクション**: 作業停止 → GitHub Issue作成 → 人間に通知

### 🟡 警告エスカレーション（作業継続）
- テストが3回連続で失敗
- カバレッジが70-79%で目標未達
- 外部APIの仕様変更
- 依存ライブラリの非推奨警告

**アクション**: 作業継続 → GitHub Issue作成 → 通知

### 🟢 情報エスカレーション（ログ記録のみ）
- 軽微な設計判断
- パフォーマンス改善の提案
- コードスタイルの統一提案

**アクション**: .aad/retrospectives/にログ記録

---

## 📊 品質ゲート

各フェーズの完了条件：

### SPEC（仕様）
- [ ] 受け入れ基準がテスト可能な形式で記述されている
- [ ] MoSCoWで優先度が設定されている
- [ ] **⚠️ 人間承認必須**

### TASKS（タスク分割）
- [ ] 全タスクにID（SPEC-XXX-TXX）が付与されている
- [ ] 依存関係が明記されている
- [ ] GitHub Issuesが作成されている
- [ ] **⚠️ 人間承認必須**

### TDD（開発）
- [ ] 全テストがgreen
- [ ] カバレッジ80%以上
- [ ] Lint通過
- [ ] `gh pr create --draft`でPR作成完了

### REVIEW（レビュー）
- [ ] AI自己レビュー完了
- [ ] CI green
- [ ] **⚠️ 人間承認必須**

### RETRO（振り返り）
- [ ] .aad/retrospectives/にログ作成
- [ ] CLAUDE.md更新提案

### MERGE（統合）
- [ ] mainマージ完了
- [ ] Issue閉鎖
- [ ] worktree削除

---

## 🧠 学びの蓄積

このセクションはプロジェクトを通じて得た学びを記録します。`/aad:retro`コマンドで自動追記されます。

### [日付] - [タスクID]: [学びのタイトル]

**状況**: [何をしていたか]

**問題**: [何が起きたか]

**解決策**: [どう解決したか]

**学び**: [次回に活かすこと]

---

### 例: 2026-01-15 - SPEC-001-T03: テストデータのクリーンアップ漏れ

**状況**: ユーザー登録機能のテストを実行中

**問題**: テスト間でデータが残留し、次のテストが失敗

**解決策**: `afterEach`フックでデータベースをクリーンアップ

**学び**:
- テストの独立性を保つため、必ず`afterEach`でクリーンアップ
- `beforeEach`でのセットアップとセットで実装する

---

### 2026-01-18 - SPEC-003: 子Agentへの既存API確認指示

**状況**: オーケストレーションで子AgentがTokenMapを使用

**問題**: 子Agentが存在しないメソッド名（apply）を使用してビルドエラー

**解決策**: 親Agentが`replace_tokens`に修正

**学び**:
- 子Agentへの指示で「既存のメソッド名をコードから確認してから使用すること」を明示
- 特にドメイン層のAPIを使う場合は、先に該当ファイルを読むよう指示

---

### 2026-01-18 - SPEC-003: Clippy needless_borrows_for_generic_args

**状況**: Command::argsメソッドで`.args(&[...])`形式を使用

**問題**: Rust 1.92では不要な借用警告が発生

**解決策**: `.args([...])`に修正（借用なし）

**学び**:
- Command::argsは借用なしで配列を渡せる
- ジェネリック関数では不要な借用を避けるべき
- ビルド成功後は即座にClippyを実行して警告チェック

---

### 2026-01-18 - SPEC-001: 段階的実装の重要性

**状況**: SPEC-001〜003を1コミットで実装

**問題**: レビューが困難、問題の切り分けが難しい

**解決策**: 1タスク = 1PR の原則を徹底

**学び**:
- PRは200行以内を目標
- 小さな単位での統合
- 段階的なレビューで品質向上

---

### 2026-01-18 - SPEC-001: Rustdocの重要性

**状況**: Domain層実装時にドキュメントが不足

**問題**: コードの意図が不明確、使用例がない

**解決策**: コード記述前にRustdocコメントを書く

**学び**:
- doctestで使用例を提供
- `cargo doc --open`で確認
- ドキュメントは実装の一部

---

### 2026-01-18 - SPEC-001: Value Objectバリデーションの標準化

**状況**: Value Object作成時にバリデーションが不足

**問題**: 後からバリデーションを追加すると既存コードに影響

**解決策**: 作成時のチェックリストを用意

**学び**:
- 空文字列チェック
- 長さ制限
- フォーマット検証
- 不正文字チェックを最初から実装

---

### 2026-01-18 - SPEC-002: 設定ファイルのバージョニング

**状況**: Application層とInfrastructure層で設定管理を実装

**問題**: 設定ファイルの構造変更時に互換性問題が発生

**解決策**: スキーマバージョンフィールドを最初から含める

**学び**:
- `version = "1.0"`フィールドを追加
- マイグレーション機能を計画
- 後方互換性を考慮した設計

---

### 2026-01-18 - SPEC-002: テストデータ管理の改善

**状況**: Infrastructure層のテスト実装中

**問題**: テスト実行後に一時ファイルが残る

**解決策**: `tempfile`クレートを使った自動クリーンアップ

**学び**:
- `TempDir::new()`で自動削除されるディレクトリ作成
- Dropトレイトでクリーンアップを保証
- CI環境での信頼性向上

---

### 2026-01-18 - SPEC-002: エラーメッセージの国際化

**状況**: バリデーション機能実装中

**問題**: エラーメッセージが英語と日本語で混在

**解決策**: メッセージカタログの導入を検討

**学び**:
- 言語設定に応じたメッセージ切り替え
- 一貫性のあるエラーメッセージ
- 将来の国際化対応を見据えた設計

---

### 2026-01-18 - SPEC-004: Domain層の型設計先行アプローチ

**状況**: オーケストレーション機能を段階的に実装中

**問題**: T03でSessionIdをStringからnewtypeパターンに変更し、テストコードの修正が発生

**解決策**: Domain層のエンティティとValue Objectを最初に完全定義してから実装開始

**学び**:
- newtypeパターンで型安全性を最大化
- 後からの型変更は手戻りが大きい
- Domain層の設計が全体の品質を左右する

---

### 2026-01-18 - SPEC-004: Clippy警告への即時対応

**状況**: T06でビルド成功後、Clippy警告が発生

**問題**: `needless_borrows_for_generic_args`など、ビルドは通るが最適化の余地がある

**解決策**: `cargo build`成功後、即座に`cargo clippy`を実行

**学び**:
- Clippy警告は品質の指標
- 早期発見・早期修正が効率的
- CI/CDパイプラインでの必須化を推奨

---

### 2026-01-18 - SPEC-004: テストでのConfig初期化の改善

**状況**: OrchestratorConfigに新フィールド追加時、全テストが壊れた

**問題**: テストコード内で構造体を直接初期化していたため、フィールド追加の影響が大きい

**解決策**: Builder パターンまたはDefaultトレイトを活用

**学び**:
- テストではデフォルト値を使用し、変更箇所のみ指定
- `..Default::default()`パターンが有効
- 将来の拡張性を考慮したテスト設計が重要

---

## 🔧 プロジェクト固有の設定

### 環境変数
```bash
# .env.example を参照
ANTHROPIC_API_KEY=
```

### ビルド・実行
```bash
cargo build
cargo run
```

### テスト実行
```bash
cargo test
cargo llvm-cov --html  # カバレッジ計測
```

### Lint実行
```bash
cargo clippy
cargo fmt
```

---

## 📝 備考

このファイルはプロジェクトの進行に合わせて更新してください。特に「学びの蓄積」セクションは積極的に記録し、チーム全体の知見として共有します。

---

**最終更新**: 2026-01-18
**更新者**: Claude Code


# AI-DLC and Spec-Driven Development

Kiro-style Spec Driven Development implementation on AI-DLC (AI Development Life Cycle)

## Project Context

### Paths
- Steering: `.kiro/steering/`
- Specs: `.kiro/specs/`

### Steering vs Specification

**Steering** (`.kiro/steering/`) - Guide AI with project-wide rules and context
**Specs** (`.kiro/specs/`) - Formalize development process for individual features

### Active Specifications
- Check `.kiro/specs/` for active specifications
- Use `/kiro:spec-status [feature-name]` to check progress

## Development Guidelines
- Think in English, generate responses in Japanese. All Markdown content written to project files (e.g., requirements.md, design.md, tasks.md, research.md, validation reports) MUST be written in the target language configured for this specification (see spec.json.language).

## Minimal Workflow
- Phase 0 (optional): `/kiro:steering`, `/kiro:steering-custom`
- Phase 1 (Specification):
  - `/kiro:spec-init "description"`
  - `/kiro:spec-requirements {feature}`
  - `/kiro:validate-gap {feature}` (optional: for existing codebase)
  - `/kiro:spec-design {feature} [-y]`
  - `/kiro:validate-design {feature}` (optional: design review)
  - `/kiro:spec-tasks {feature} [-y]`
- Phase 2 (Implementation): `/kiro:spec-impl {feature} [tasks]`
  - `/kiro:validate-impl {feature}` (optional: after implementation)
- Progress check: `/kiro:spec-status {feature}` (use anytime)

## Development Rules
- 3-phase approval workflow: Requirements → Design → Tasks → Implementation
- Human review required each phase; use `-y` only for intentional fast-track
- Keep steering current and verify alignment with `/kiro:spec-status`
- Follow the user's instructions precisely, and within that scope act autonomously: gather the necessary context and complete the requested work end-to-end in this run, asking questions only when essential information is missing or the instructions are critically ambiguous.

## Steering Configuration
- Load entire `.kiro/steering/` as project memory
- Default files: `product.md`, `tech.md`, `structure.md`
- Custom files are supported (managed via `/kiro:steering-custom`)
