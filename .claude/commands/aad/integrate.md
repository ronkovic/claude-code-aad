# PRマージ + worktree削除

## 🔴 重要: 出力指示

統合完了後、**必ず以下の形式で「次のステップ」を目立つように表示すること**:

### 必須出力フォーマット

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
完了：統合が完了しました。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

次のステップ:

1️⃣ 全体進捗を確認
/aad:status
結果：プロジェクト全体の状態を表示します。

2️⃣ 次のタスクに着手
/aad:worktree SPEC-XXX-TXX
通知：次のタスクへの着手を推奨します。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 理由

- 統合完了後、次のタスクへの移行を促す
- 進捗確認の習慣を定着させる
- ファイル後半の「出力例」だけでは見逃される可能性がある

---

タスク完了後、PRをデフォルトブランチにマージしてworktreeを削除します。

## 実行内容

1. **品質チェック**
   - `/aad:gate TDD` で品質ゲートをチェック
   - テストgreen、カバレッジ80%以上、Lint通過を確認

2. **PRステータス確認**
   - GitHub PRの状態を確認
   - CI greenを確認
   - レビュー承認を確認

3. **マージ実行**
   - `gh pr merge` でPRをマージ
   - Squash mergeをデフォルトで使用
   - ブランチを自動削除

4. **PRボディにIssue自動クローズを追加**（GitHub連携使用時）
   - PRボディに `Closes #XX` を自動追加
   - タスクファイルから対応するGitHub Issue番号を取得
   - PRマージ時に自動的にIssueがクローズされる

5. **Issue閉鎖確認**（GitHub連携使用時）
   - PRマージ後、対応するIssueが自動クローズされたことを確認

6. **worktree削除**
   - `git worktree remove` でworktreeを削除
   - ローカルブランチも削除

7. **HANDOFF.md更新**
   - 「進行中のタスク」から削除
   - 「完了したタスク」に追加

## 使用方法

```
/aad:integrate SPEC-001-T01
```

オプション指定：

```
/aad:integrate SPEC-001-T01 --merge-strategy=merge  # Merge commit使用
/aad:integrate SPEC-001-T01 --no-delete-branch      # ブランチを残す
```

## 出力例

```
統合を開始します: SPEC-001-T01

🔍 品質チェック:
   ✅ テスト: green (25/25 passed)
   ✅ カバレッジ: 85%
   ✅ Lint: 0 errors

🔍 PRステータス確認:
   PR: #18
   ✅ CI: Pass
   ✅ レビュー: Approved by @reviewer
   ブランチ: feature/SPEC-001-T01

⚠️  デフォルトブランチにマージします。よろしいですか？ (y/n)

> y

🔀 マージ実行:
   ✅ PR #18 をマージしました
   ✅ ブランチ feature/SPEC-001-T01 を削除しました

🔒 Issue閉鎖:（GitHub連携使用時）
   ✅ Issue #12 を閉鎖しました

🗑️  worktree削除:
   ✅ ../my-project-T01 を削除しました
   ✅ ローカルブランチを削除しました

📝 HANDOFF.md更新:
   ✅ 完了タスクに追加しました

✅ 統合完了！

完了：統合が完了しました。

次のステップ:

1️⃣ 全体進捗を確認
/aad:status
結果：プロジェクト全体の状態を表示します。

2️⃣ 次のタスクに着手
/aad:worktree SPEC-001-T02
通知：次のタスクへの着手を推奨します。
```

## マージ戦略

### Squash Merge（デフォルト）
- コミット履歴を1つにまとめる
- デフォルトブランチがクリーンに保たれる
- 推奨：小〜中規模のタスク

### Merge Commit
- コミット履歴を保持
- タスクの作業過程を追跡可能
- 推奨：大規模なタスク、複数サブタスク

### Rebase Merge
- リニアな履歴を維持
- コンフリクト解決が必要な場合がある
- 推奨：履歴の美しさを重視する場合

---

## マージ方法の詳細

### 方法1: GitHub Web UI（推奨）

**メリット**:
- 視覚的で分かりやすい
- レビューコメントと一緒に確認できる
- CIステータスが一目で分かる

**手順**:
1. PR画面を開く
2. "Squash and merge" ボタンをクリック
3. コミットメッセージを確認・編集
4. "Confirm squash and merge" をクリック
5. "Delete branch" をクリック（オプション）

### マージ先ブランチの自動検出

PRマージ時、worktreeのベースブランチを自動検出してマージ先とします:

```bash
# worktreeのベースブランチ取得
BASE=$(git config --get branch.$(git branch --show-current).merge | sed 's|refs/heads/||')

# PRをベースブランチに向けて作成（既にPRが存在する場合は不要）
gh pr create --base "$BASE" --draft --title "feat(SPEC-XXX): タスク概要"

# マージもベースブランチに向ける
gh pr merge <PR番号> --squash --delete-branch --base "$BASE"
```

**注意**:
- worktreeで作成されたブランチは、元のブランチ（通常は`main`またはSPECブランチ）を自動的に追跡します
- `--base`を明示することで、意図しないブランチへのマージを防ぎます
- デフォルトブランチが`main`以外の場合も正しく動作します

### 方法2: GitHub CLI

**メリット**:
- コマンドラインで完結
- スクリプト化・自動化が容易
- 高速な操作が可能

**基本的なマージ（Squash）**:
```bash
# ベースブランチを自動検出してマージ
BASE=$(git config --get branch.$(git branch --show-current).merge | sed 's|refs/heads/||')
gh pr merge <PR番号> --squash --delete-branch

# または明示的にベース指定
gh pr merge <PR番号> --squash --delete-branch --base "$BASE"
```

**マージ戦略を指定**:
```bash
# Merge commit
gh pr merge <PR番号> --merge --delete-branch

# Rebase merge
gh pr merge <PR番号> --rebase --delete-branch
```

**承認待ちの場合、自動マージ設定**:
```bash
gh pr merge <PR番号> --squash --delete-branch --auto
```

**コミットメッセージを指定**:
```bash
gh pr merge <PR番号> --squash --delete-branch \
  --subject "feat(SPEC-001): ユーザー認証機能の実装" \
  --body "詳細な説明..."
```

### 方法3: Git CLI（上級者向け）

```bash
# mainブランチに切り替え
git checkout main

# 最新を取得
git pull origin main

# マージ（squash）
git merge --squash feature/SPEC-001-T01

# コミット
git commit -m "feat(SPEC-001): タスクT01の実装"

# プッシュ
git push origin main

# リモートブランチ削除
git push origin --delete feature/SPEC-001-T01

# ローカルブランチ削除
git branch -d feature/SPEC-001-T01
```

### マージ前の確認事項

- [ ] 全テストがgreen
- [ ] CI/CDが成功
- [ ] レビュー承認済み
- [ ] コンフリクトなし
- [ ] PRボディに `Closes #XX` が含まれている

## 品質ゲート完了条件

- [ ] 全テストgreen
- [ ] カバレッジ80%以上
- [ ] Lint通過
- [ ] CI green
- [ ] PRレビュー承認済み
- [ ] **⚠️ 人間承認必須**（マージ前）

## 関連コマンド

- `/aad:worktree` - worktree作成
- `/aad:status` - 進捗確認
- `/aad:gate` - 品質ゲートチェック
- `/aad:retro` - 振り返り実行

## ロールバック

マージ後に問題が発見された場合：

```bash
# デフォルトブランチでrevert
git revert <commit-hash>

# または直接修正
/aad:worktree SPEC-001-T01-hotfix
```

## 注意事項

- 必ず品質ゲートを通過してからマージしてください
- デフォルトブランチへのマージは慎重に行ってください
- worktree削除前に、未コミットの変更がないか確認してください
- マージ後は `/aad:retro` で振り返りを実行することを推奨します
- 本番環境に影響がある場合は、必ず人間の承認を得てください
