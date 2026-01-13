# 全タスク/ワーカー進捗表示

現在のプロジェクト全体の進捗状況を一覧表示します。

## 実行内容

1. **SPEC一覧取得**
   - `.aad/specs/` 配下のSPECファイルを一覧表示
   - 各SPECのステータス（Draft/Approved/In Progress/Done）

2. **タスク進捗集計**
   - `.aad/tasks/` 配下のタスクを集計
   - 完了/進行中/未着手の数

3. **worktree一覧**
   - `git worktree list` で現在のworktreeを表示
   - 各worktreeのブランチとタスクID

4. **GitHub Issues同期**
   - `gh issue list` で各タスクのIssue状態を取得
   - Open/In Progress/Closedの数

5. **品質メトリクス**
   - PRのステータス（Draft/Open/Merged）
   - CIの状態（Pass/Fail）

## 使用方法

```
/aad:status
```

または特定のSPECのみ表示：

```
/aad:status SPEC-001
```

## 出力例

```
📊 プロジェクト進捗状況

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 SPEC一覧
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SPEC-001: ユーザー認証機能 [In Progress]
  📊 タスク: 3/5 完了 (60%)
  ├─ ✅ T01: データベーススキーマ (#12) - Merged
  ├─ ✅ T02: 認証API実装 (#13) - Merged
  ├─ 🚧 T03: フロントエンドUI (#14) - In Progress
  ├─ ⏸️  T04: パスワードリセット (#15) - Open
  └─ ⏸️  T05: ソーシャルログイン (#16) - Open

SPEC-002: ダッシュボード機能 [Draft]
  📊 タスク: 0/3 完了 (0%)
  (タスク分割未実施)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌿 アクティブなworktree
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. ../my-project-T03 [feature/SPEC-001-T03]
   └─ SPEC-001-T03: フロントエンドUI
      状態: 開発中
      PR: #18 (Draft)
      CI: ✅ Pass

2. ../my-project-T04 [feature/SPEC-001-T04]
   └─ SPEC-001-T04: パスワードリセット
      状態: 開発中
      PR: なし
      CI: N/A

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📈 全体サマリー
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SPEC: 2件
  ├─ Draft: 1件
  ├─ In Progress: 1件
  └─ Done: 0件

タスク: 8件
  ├─ ✅ 完了: 2件 (25%)
  ├─ 🚧 進行中: 2件 (25%)
  └─ ⏸️  未着手: 4件 (50%)

worktree: 2個 アクティブ

Issues:
  ├─ Open: 4件
  ├─ In Progress: 2件
  └─ Closed: 2件

Pull Requests:
  ├─ Draft: 1件
  ├─ Open: 0件
  └─ Merged: 2件

品質:
  ├─ CI Pass: 1/1 (100%)
  └─ 平均カバレッジ: 85%
```

## フィルタオプション

```
/aad:status SPEC-001           # 特定SPECのみ
/aad:status --active           # 進行中のタスクのみ
/aad:status --worktrees        # worktree一覧のみ
/aad:status --quality          # 品質メトリクスのみ
```

## 関連コマンド

- `/aad:tasks` - タスク分割
- `/aad:worktree` - worktree作成
- `/aad:integrate` - PRマージ + worktree削除
- `/aad:gate` - 品質ゲートチェック

## 注意事項

- GitHub CLIが設定されていることを確認してください（`gh auth status`）
- worktreeの状態はローカルの情報を基に表示されます
- CI状態の取得にはGitHub Actionsの設定が必要です
- 定期的に実行して進捗を把握してください
