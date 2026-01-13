# Docker開発環境

## 概要

Claude Codeを使用した自律的AI駆動開発のためのDocker環境です。複数のワーカーコンテナを並列実行し、効率的な開発を実現します。

## 前提条件

- Docker Desktop 4.0以上
- Claude Code認証（OAuth TokenまたはAPI Key）
- 8GB以上のメモリ推奨

## クイックスタート

### 1. 環境変数の設定

```bash
cd container
cp .env.example .env
# .envファイルを編集して認証情報を設定
```

必須の設定：
- `CLAUDE_CODE_OAUTH_TOKEN`: Claude Code OAuth Token（Max Planユーザー）
- または `ANTHROPIC_API_KEY`: Anthropic API Key（APIユーザー）

### 2. イメージビルド

```bash
docker-compose build
```

### 3. 起動

```bash
# Orchestratorのみ起動
docker-compose up orchestrator

# 全サービス起動（Orchestrator + Worker 1, 2）
docker-compose up
```

### 4. アクセス

```bash
# Orchestratorに接続（PROJECT_NAMEは.envのCOMPOSE_PROJECT_NAME）
docker exec -it PROJECT_NAME-orchestrator-1 bash

# Worker 1に接続
docker exec -it PROJECT_NAME-worker-1-1 bash

# 例: COMPOSE_PROJECT_NAME=my-project-aad の場合
docker exec -it my-project-aad-orchestrator-1 bash
```

**ヒント**: `docker ps`で実際のコンテナ名を確認できます。

---

## 複数プロジェクトでの同時実行

異なるプロジェクトで同時にAADコンテナを実行する場合、コンテナ名の衝突を避けるために
プロジェクトごとにユニークな`COMPOSE_PROJECT_NAME`を設定する必要があります。

### 設定方法

#### Option 1: .envファイルで設定（推奨）

```bash
# プロジェクトAの.env
COMPOSE_PROJECT_NAME=project-a-aad
CLAUDE_CODE_OAUTH_TOKEN=xxx

# プロジェクトBの.env
COMPOSE_PROJECT_NAME=project-b-aad
CLAUDE_CODE_OAUTH_TOKEN=xxx
```

それぞれのプロジェクトで起動:

```bash
# プロジェクトA
cd /path/to/project-a/aad/container
docker-compose up -d

# プロジェクトB
cd /path/to/project-b/aad/container
docker-compose up -d
```

#### Option 2: 起動時に指定

```bash
# プロジェクトA
cd /path/to/project-a/aad/container
docker-compose -p project-a-aad up -d

# プロジェクトB
cd /path/to/project-b/aad/container
docker-compose -p project-b-aad up -d
```

### コンテナ名の確認

```bash
docker ps
# project-a-aad-orchestrator-1
# project-a-aad-worker-1-1
# project-a-aad-worker-2-1
# project-b-aad-orchestrator-1
# project-b-aad-worker-1-1
# project-b-aad-worker-2-1
```

### プロジェクト別のコンテナ操作

```bash
# プロジェクトAのOrchestratorに接続
docker exec -it project-a-aad-orchestrator-1 bash

# プロジェクトBのWorker 1に接続
docker exec -it project-b-aad-worker-1-1 bash

# プロジェクトAのログ確認
docker logs -f project-a-aad-orchestrator-1

# プロジェクトBを停止
cd /path/to/project-b/aad/container
docker-compose -p project-b-aad down
```

### 注意事項

- `COMPOSE_PROJECT_NAME`を設定しない場合、ディレクトリ名（`container`）がプロジェクト名として使用されます
- 同じディレクトリ名の場合、コンテナ名が衝突するため必ず`COMPOSE_PROJECT_NAME`を設定してください
- プロジェクトごとに異なる`COMPOSE_PROJECT_NAME`を使用することを強く推奨します

---

## Git Worktreeとの連携

### 重要: パスマッピングについて

Git worktreeを使用する場合、**同一パスでマウント**する必要があります。

**理由**:
- worktreeの`.git`ファイルはホストの絶対パスを参照
- 例: `/Users/yourname/workspace/project/.git/worktrees/feature-branch`
- コンテナ内で異なるパスにマウントすると、このパスにアクセスできずgit操作が失敗

### 設定方法

#### Option 1: .envファイルで設定（推奨）

```bash
# .envファイルに追加
HOST_PROJECT_PATH=/Users/yourname/workspace/project

# docker-composeで起動
docker-compose up
```

#### Option 2: 環境変数で指定

```bash
HOST_PROJECT_PATH=/Users/yourname/workspace/project docker-compose up
```

#### Option 3: docker runで直接指定

```bash
docker run --rm -it \
  -v /Users/yourname/workspace:/Users/yourname/workspace \
  -w /Users/yourname/workspace/project \
  -e CLAUDE_CODE_OAUTH_TOKEN="xxx" \
  autonomous-dev
```

### worktree使用例

```bash
# 1. ホスト側でworktreeを作成
cd /Users/yourname/workspace/project
git worktree add ../project-feature -b feature/new-feature

# 2. .envにHOST_PROJECT_PATHを設定
echo 'HOST_PROJECT_PATH=/Users/yourname/workspace/project-feature' >> .env

# 3. Docker Worker起動
docker-compose up worker-1

# 4. Workerに接続して作業（PROJECT_NAMEは.envのCOMPOSE_PROJECT_NAME）
docker exec -it PROJECT_NAME-worker-1-1 bash
# コンテナ内で /Users/yourname/workspace/project-feature が作業ディレクトリになる
# git操作が正常に動作する
```

---

## 動的Worker管理

### 概要

タスクIDに基づいて動的にWorkerコンテナを起動・停止できます。`container/scripts/`配下のスクリプトを使用します。

**従来の方式との違い**:

| 項目 | 従来（docker-compose） | 動的Worker管理 |
|------|----------------------|---------------|
| コンテナ名 | `autonomous-dev-worker-1` | `aad-SPEC-001-T01` |
| Worker数 | 固定（yml編集必要） | 無制限（動的追加） |
| タスク対応 | 手動マッピング | 自動（タスクID＝コンテナ名） |

### Worker起動

```bash
cd container/scripts

# 基本的な起動
./start-worker.sh SPEC-001-T01 /path/to/worktree-T01

# 起動後、即座に接続
./start-worker.sh SPEC-001-T01 /path/to/worktree-T01 --attach
```

**引数**:
- `<task-id>`: タスクID（例: `SPEC-001-T01`）
- `<worktree-path>`: worktreeの絶対パス
- `--attach`: （オプション）起動後すぐにコンテナに接続

**動作**:
1. タスクIDからコンテナ名を自動生成（`aad-SPEC-001-T01`）
2. 同一パスマウントでworktreeをマウント
3. 環境変数を`.env`から自動ロード
4. `autonomous-net`ネットワークに接続

### Worker一覧表示

```bash
cd container/scripts

# 実行中のWorkerのみ表示
./list-workers.sh

# 停止中も含めて全て表示
./list-workers.sh --all
```

**出力例**:
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Docker Worker Status
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🚀 Running Workers:

  ● SPEC-001-T01
     Container: aad-SPEC-001-T01
     Status:    Up 5 minutes
     Workdir:   /Users/user/workspace/myproject-T01

     Actions:
       Attach:  docker exec -it aad-SPEC-001-T01 bash
       Logs:    docker logs -f aad-SPEC-001-T01
       Stop:    ./stop-worker.sh SPEC-001-T01

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running: 1  |  Stopped: 0  |  Total: 1
```

### Worker停止

```bash
cd container/scripts

# 単一Worker停止（確認あり）
./stop-worker.sh SPEC-001-T01

# 単一Worker停止（確認なし）
./stop-worker.sh SPEC-001-T01 --force

# 全Worker停止（確認あり）
./stop-worker.sh --all

# 全Worker停止（確認なし）
./stop-worker.sh --all --force
```

### 並列3Worker実行例

```bash
# 1. Worktreeを3つ作成（/aad:worktreeコマンド使用）
/aad:worktree SPEC-001-T01
/aad:worktree SPEC-001-T02
/aad:worktree SPEC-001-T03

# 2. 3つのWorkerを起動
cd container/scripts
./start-worker.sh SPEC-001-T01 /path/to/project-T01 &
./start-worker.sh SPEC-001-T02 /path/to/project-T02 &
./start-worker.sh SPEC-001-T03 /path/to/project-T03 &
wait

# 3. Worker状態確認
./list-workers.sh

# 4. 各Workerに接続して作業
docker exec -it aad-SPEC-001-T01 bash  # Terminal 1
docker exec -it aad-SPEC-001-T02 bash  # Terminal 2
docker exec -it aad-SPEC-001-T03 bash  # Terminal 3

# 5. コンテナ内でClaude実行
claude --dangerously-skip-permissions -p 'docs/aad/tasks/SPEC-001-T01.mdに従って実装'

# 6. 完了後、全Worker停止
./stop-worker.sh --all
```

### /aad:worktreeコマンドとの連携

`/aad:worktree`コマンドでworktreeを作成した後、自動的にWorkerを起動することも可能です。

詳細は`.claude/commands/aad/worktree.md`を参照してください。

---

## アーキテクチャ

### サービス構成

| サービス | 役割 | コンテナ名 |
|---------|------|-----------|
| orchestrator | タスク分割、進捗監視、統合 | `${COMPOSE_PROJECT_NAME}-orchestrator-1` |
| worker-1 | 並列ワーカー1 | `${COMPOSE_PROJECT_NAME}-worker-1-1` |
| worker-2 | 並列ワーカー2 | `${COMPOSE_PROJECT_NAME}-worker-2-1` |

**注**: コンテナ名は`COMPOSE_PROJECT_NAME`に基づいて動的に生成されます。
例: `COMPOSE_PROJECT_NAME=my-project-aad`の場合 → `my-project-aad-orchestrator-1`

### ネットワーク

すべてのサービスは`autonomous-net`ブリッジネットワークで接続されています。

---

## 環境変数

詳細は`.env.example`を参照してください。

### 認証関連

| 変数名 | 説明 | 必須 |
|--------|------|------|
| `CLAUDE_CODE_OAUTH_TOKEN` | Claude Code OAuth Token | ○（Max Plan） |
| `ANTHROPIC_API_KEY` | Anthropic API Key | ○（API） |
| `GITHUB_TOKEN` | GitHub Personal Access Token | △ |

### Git設定

| 変数名 | 説明 | デフォルト |
|--------|------|-----------|
| `GIT_USER_NAME` | Gitコミット時のユーザー名 | Claude AI |
| `GIT_USER_EMAIL` | Gitコミット時のメールアドレス | claude@example.com |

### Docker Worktree設定

| 変数名 | 説明 | 必須 |
|--------|------|------|
| `HOST_PROJECT_PATH` | ホストのプロジェクトパス | △（worktree使用時） |

### 品質ゲート

| 変数名 | 説明 | デフォルト |
|--------|------|-----------|
| `MIN_COVERAGE` | 最小テストカバレッジ（%） | 80 |
| `LINT_MODE` | Lintモード（strict/normal/lenient） | normal |

---

## トラブルシューティング

### git操作が失敗する

**症状**: `fatal: not in a git directory` または `fatal: unable to access '.git'`

**原因**: worktreeの`.git`ファイルが参照するホストパスにアクセスできない

**解決策**:
1. `.env`ファイルで`HOST_PROJECT_PATH`を設定
2. worktreeを使用している場合、同一パスマウントになっているか確認
3. パスが正しく設定されているか確認: `echo $HOST_PROJECT_PATH`

### 権限エラー

**症状**: `Permission denied` エラー

**原因**: コンテナ内のユーザー(claude)とホストのファイル所有者が異なる

**解決策**:
```bash
# ホスト側でファイル所有者を確認
ls -la /path/to/project

# 必要に応じて--userオプションを追加
docker run --user $(id -u):$(id -g) ...
```

### コンテナが起動しない

**症状**: `docker-compose up`でエラー

**原因**: 認証情報が未設定

**解決策**:
1. `.env`ファイルを確認
2. `CLAUDE_CODE_OAUTH_TOKEN`または`ANTHROPIC_API_KEY`が設定されているか確認

### ボリュームマウントが空

**症状**: コンテナ内で`/home/claude/workspace`が空

**原因**: `HOST_PROJECT_PATH`の設定ミス

**解決策**:
1. `HOST_PROJECT_PATH`を未設定にする（デフォルト動作）
2. または正しい絶対パスを設定する

---

## 高度な使用方法

### カスタムイメージビルド

特定のバージョンのClaude Codeを使用する場合：

```dockerfile
# Dockerfileを編集
RUN npm install -g @anthropic-ai/claude-code@2.1.1
```

### 追加のWorkerを起動

`docker-compose.yml`のworker-3をコメント解除：

```yaml
worker-3:
  # ... (設定はworker-1と同じ)
```

### デバッグモード

```bash
# .envファイル
DEBUG=true
LOG_LEVEL=debug

# 起動
docker-compose up
```

---

## 関連ドキュメント

- [メインREADME](../../README.md) - プロジェクト全体の概要
- [セットアップガイド](../SETUP.md) - 詳細なセットアップ手順
- [コマンドリファレンス](../COMMANDS.md) - 利用可能なコマンド一覧
- [ワークフロー](../WORKFLOW.md) - 開発フロー

---

## ライセンス

このプロジェクトは親プロジェクトのライセンスに従います。
