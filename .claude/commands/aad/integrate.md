# PRマージ + worktree削除

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

4. **Issue閉鎖**
   - 対応するGitHub Issueを閉鎖
   - クローズコメントに成果物を記載

5. **Docker Worker停止**（使用時のみ）
   - `.aad/container/scripts/stop-worker.sh` でコンテナ停止
   - 動的Workerを削除

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

🔒 Issue閉鎖:
   ✅ Issue #12 を閉鎖しました

🐳 Docker Worker停止:
   ✅ aad-SPEC-001-T01 を停止・削除しました

🗑️  worktree削除:
   ✅ ../my-project-T01 を削除しました
   ✅ ローカルブランチを削除しました

📝 HANDOFF.md更新:
   ✅ 完了タスクに追加しました

✅ 統合完了！

次のステップ:
1. /aad:status で全体進捗を確認
2. 次のタスクに着手: /aad:worktree SPEC-001-T02
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
