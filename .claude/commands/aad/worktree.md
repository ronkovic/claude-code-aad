# worktree + ブランチ作成

Git worktreeを使用して、元のフォルダに影響を与えずに並列開発環境を構築します。

## 実行内容

1. **タスク情報の読み込み**
   - `.aad/tasks/SPEC-XXX/TXX-xxx.md` からタスク情報を取得

2. **ブランチ作成**
   - ブランチ名: `feature/SPEC-XXX-TXX`
   - デフォルトブランチから分岐（CLAUDE.mdで設定）

3. **worktree作成**
   - worktreeパス: `../[project-name]-TXX/`
   - 元のフォルダには影響なし

4. **セットアップ**
   - 依存関係インストール（package.jsonがある場合）
   - 環境変数コピー（.envがある場合）

5. **HANDOFF.md更新**
   - 「進行中のタスク」セクションにworktree情報を追記

## 使用方法

```
/aad:worktree SPEC-001-T01
```

## 出力例

```
worktreeを作成します: SPEC-001-T01

📖 タスク情報:
   データベーススキーマ
   複雑度: S
   Issue: #12

🌿 ブランチ作成: feature/SPEC-001-T01

📂 worktree作成: /Users/user/workspace/my-project-T01/

⚙️  セットアップ:
   ✅ npm install 完了
   ✅ .env コピー完了

📝 HANDOFF.md更新完了

✅ worktree準備完了！

次のステップ:
1. cd ../my-project-T01
2. claude --dangerously-skip-permissions
3. タスクの実装を開始

または:
- Docker環境で実行: docker run -v $(pwd)/../my-project-T01:/workspace ...
```

## worktreeアーキテクチャ

```
my-project/              # デフォルトブランチ - 調整役/統合用
├── HANDOFF.md           # 全体進捗管理
└── .aad/tasks/SPEC-001/      # タスク定義

my-project-T01/          # worktree - Worker 1
└── (独立した作業環境)

my-project-T02/          # worktree - Worker 2
└── (独立した作業環境)
```

## ワーカー完了条件

1. [ ] 全テストgreen
2. [ ] カバレッジ80%以上
3. [ ] Lint通過
4. [ ] `gh pr create --draft` でPR作成
5. [ ] GitHub Issue更新（進捗コメント追加）

## 関連コマンド

- `/aad:tasks` - タスク分割
- `/aad:status` - 進捗確認
- `/aad:integrate` - PRマージ + worktree削除
- `/aad:orchestrate` - 全自動オーケストレーション

## Docker環境での使用

### 動的Worker管理スクリプト（推奨）

worktree作成後、動的Workerスクリプトで自動的にコンテナを起動できます：

```bash
# 1. worktree作成
/aad:worktree SPEC-001-T01

# 2. Docker Worker起動
cd .aad/container/scripts
./start-worker.sh SPEC-001-T01 /path/to/project-T01

# 3. 起動後すぐに接続
./start-worker.sh SPEC-001-T01 /path/to/project-T01 --attach

# 4. Worker一覧確認
./list-workers.sh

# 5. Worker停止
./stop-worker.sh SPEC-001-T01
```

**メリット**:
- タスクIDに基づくコンテナ名（`aad-SPEC-001-T01`）
- 同一パスマウントで自動Git worktree互換
- 環境変数の自動ロード（`.env`から）
- 無制限の並列Worker実行

詳細は `.aad/container/README.md` の「動的Worker管理」セクションを参照してください。

### 手動実行（従来方式）

```bash
# ⚠️ 重要: Git worktree互換のため、同一パスでマウント
docker run -it \
  -v /path/to/project-T01:/path/to/project-T01 \
  -w /path/to/project-T01 \
  -e CLAUDE_CODE_OAUTH_TOKEN="xxx" \
  autonomous-dev

# コンテナ内で
claude --dangerously-skip-permissions
```

## 注意事項

- worktreeは `.aad/worktrees/` 配下ではなく、プロジェクトの親ディレクトリに作成されます
- 元のフォルダ（デフォルトブランチ）には一切影響を与えません
- 複数のworktreeを同時に作成して並列開発が可能です
- worktreeを削除する場合は `/aad:integrate` コマンドを使用してください
- 手動削除する場合: `git worktree remove ../my-project-T01`
