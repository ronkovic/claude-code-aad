# SPEC自律実行（Worker起動）

## 🔴 重要: 出力指示

worktree作成と自律実行プロンプト生成完了後、**必ず以下の形式で「次のステップ」を目立つように表示すること**:

### 必須出力フォーマット

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
完了：worktree作成と自律実行プロンプト生成が完了しました
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📋 SPEC: [SPEC-ID]
📂 worktree: ../[worktree-name]/
🌿 ブランチ: feature/[SPEC-ID]

タスク実行順序:
  1. [T01]: [タスク名]
  2. [T02]: [タスク名]
  3. [T03]: [タスク名]
  ...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
通知：別ターミナルで以下を実行してください
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

cd ../[worktree-name] && claude --dangerously-skip-permissions

子Claude Codeが自律的に全タスクを実行し、PR作成まで行います。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 理由
- 現在のセッションは統合・管理用
- worktreeは別プロセスで自律実行
- 子Claude Codeが全タスクを依存順に実行

---

SPEC全体を自律実行するためのworktree作成と自律実行プロンプトを生成します。

## 実行内容

1. **SPEC情報の読み込み**
   - `.aad/specs/SPEC-XXX.md` からSPEC情報を取得

2. **タスク一覧取得と依存関係解析**
   - `.aad/tasks/SPEC-XXX/` 配下の全タスクファイルを読み込み
   - 依存関係を解析して実行順序を決定

3. **ブランチ作成**
   - ブランチ名: `feature/SPEC-XXX`
   - デフォルトブランチから分岐（CLAUDE.mdで設定）

4. **worktree作成**
   - worktreeパス: `../[project-name]-SPEC-XXX/`
   - 元のフォルダには影響なし

5. **SPECとTASKのコピー**
   - 元のディレクトリから `.aad/specs/SPEC-XXX.md` をworktreeにコピー
   - 元のディレクトリから `.aad/tasks/SPEC-XXX/` ディレクトリ全体をworktreeにコピー
   - **重要**: `.aad` がgit管理されていない場合でも、子Claude Codeが必要なファイルにアクセスできるようにする

6. **自律実行プロンプト生成**
   - `AUTONOMOUS-PROMPT.md` を worktree 内に作成
   - タスク実行順序と実行手順を記述

7. **CLAUDE.md 自律実行モード追加**
   - worktreeのCLAUDE.mdに自律実行モードセクションを追加
   - 起動時に `AUTONOMOUS-PROMPT.md` を自動読み込み

8. **セットアップ**
   - 依存関係インストール（package.jsonがある場合）
   - 環境変数コピー（.envがある場合）

9. **HANDOFF.md更新**
   - 「進行中のタスク」セクションにworktree情報を追記

## 使用方法

```
/aad:worker SPEC-001
```

## 実行フロー

```
┌─────────────────────────────────────────┐
│ 親Claude Code（このセッション）           │
│                                         │
│ /aad:worker SPEC-001                    │
│   ↓                                     │
│ 1. SPEC配下のタスク一覧取得              │
│ 2. 依存関係解析 → 実行順序決定           │
│ 3. worktree作成（feature/SPEC-001）     │
│ 4. SPECとTASKをworktreeにコピー          │
│ 5. 自律実行プロンプト生成・配置          │
│ 6. 実行コマンドを表示                   │
└─────────────────────────────────────────┘
                    ↓
          ユーザーが手動で実行
                    ↓
┌─────────────────────────────────────────┐
│ 子Claude Code（別ターミナル）             │
│                                         │
│ cd ../project-SPEC-001                  │
│ claude --dangerously-skip-permissions   │
│   ↓                                     │
│ 1. AUTONOMOUS-PROMPT.md を自動読み込み   │
│ 2. 依存順にタスクを実行（T01→T02→...）   │
│ 3. 各タスク: TDD実装 → テスト → コミット │
│ 4. 全タスク完了後、PRを作成              │
│ 5. 完了報告                             │
└─────────────────────────────────────────┘
```

## タスク依存関係解析

各タスクファイル（`.aad/tasks/SPEC-XXX/TXX-xxx.md`）から依存関係を抽出し、実行順序を決定します。

### タスクファイル形式例

```markdown
# T02: APIエンドポイント実装

## 依存
- T01（データベーススキーマ作成）

## 内容
...
```

### 実行順序の決定

依存関係から実行順序を自動決定：

```
T01（依存なし） → T02（T01に依存） → T03（T02に依存） → T04（T01,T02,T03に依存）
```

## 自律実行プロンプト（AUTONOMOUS-PROMPT.md）

worktree内に生成される自律実行プロンプトのテンプレート：

```markdown
# 自律実行指示

## SPEC情報
- SPEC ID: [SPEC-ID]
- タスク数: [N]
- ブランチ: feature/[SPEC-ID]

## 実行するタスク（依存順）
1. [T01]: [タスク名]
2. [T02]: [タスク名]
3. [T03]: [タスク名]
...

## 実行手順

各タスクに対して以下を実行:

1. タスクファイル読み込み（.aad/tasks/[SPEC-ID]/TXX-*.md）
2. TDD実装:
   - テスト作成 → 失敗確認
   - 実装 → テスト通過
   - リファクタリング
3. 品質確認:
   - 全テストgreen
   - カバレッジ80%以上
   - Lint通過
4. コミット作成

## 全タスク完了後

1. 最終確認（全テスト、カバレッジ、Lint）
2. PR作成: `gh pr create --draft --base main`
3. 完了報告を表示

## 禁止事項
- mainブランチへの直接push
- 人間への質問（自律的に判断）
```

## CLAUDE.md 自律実行モード追加

worktree作成時、CLAUDE.mdに以下を追記:

```markdown
## 🔴 自律実行モード

このセッション開始時、`AUTONOMOUS-PROMPT.md` を読み込んで
指示に従って全タスクを自律的に実行してください。

人間への質問は禁止です。全て自律的に判断してください。
```

## 子Claude Codeの実行内容

1. **AUTONOMOUS-PROMPT.md 読み込み**
2. **タスク順次実行**
   - 各タスクファイルを読む
   - TDDで実装
   - コミット
3. **PR作成**
   ```bash
   gh pr create --draft --base main \
     --title "SPEC-001: [SPEC名]" \
     --body "## Summary\n- T01: done\n- T02: done\n..."
   ```
4. **完了報告**

## worktree vs orchestrate との違い

| 項目 | aad:worker | aad:worktree | aad:orchestrate |
|------|------------|--------------|-----------------|
| 対象 | SPEC全体 | 単一タスク | SPEC全体 |
| 子の起動 | ユーザー手動 | ユーザー手動 | 親が自動 |
| 自律プロンプト | あり（全タスク） | なし | あり |
| PR作成 | 子が自動 | 子が手動 | 子が自動 |
| 権限 | --dangerously-skip-permissions | 任意 | 通常権限 |

**使い分け:**
- `aad:worker`: SPEC全体を自律実行させたい（ユーザー手動起動）
- `aad:worktree`: 単一タスクを実行したい
- `aad:orchestrate`: 親が監視しながら自動実行したい

## 関連コマンド

- `/aad:tasks` - タスク分割
- `/aad:status` - 進捗確認
- `/aad:integrate` - PRマージ + worktree削除
- `/aad:orchestrate` - 全自動オーケストレーション

## 注意事項

- worktreeは `.aad/worktrees/` 配下ではなく、プロジェクトの親ディレクトリに作成されます
- 元のフォルダ（デフォルトブランチ）には一切影響を与えません
- 複数のworktreeを同時に作成して並列開発が可能です
- worktreeを削除する場合は `/aad:integrate` コマンドを使用してください
- 手動削除する場合: `git worktree remove ../[worktree-name]`
- 子Claude Codeは `--dangerously-skip-permissions` で起動し、自律的に全タスクを実行します
- **重要**: `.aad` がgit管理されていない場合でも、SPECとTASKファイルは自動的にworktreeにコピーされます
