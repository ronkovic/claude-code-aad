# 全自動オーケストレーション（3層アーキテクチャ）

## 🔴 重要: 必須手順（スキップ禁止）

以下の手順は**タスクの種類に関わらず**必ず実行すること。ドキュメント改善でも、コード変更でも、同じワークフローを使用します。

### 必須実行項目

1. **worktree作成**: 各タスクごとに独立した作業ディレクトリを作成
   ```bash
   git worktree add ../{{PROJECT_NAME}}-{{TASK_SHORT_ID}} -b feature/{{TASK_ID}}
   ```

2. **子Agent起動**: Taskツールで`run_in_background=true`を使用
   ```
   Task(
     description: "SPEC-XXX-TXXを実行",
     subagent_type: "general-purpose",
     run_in_background: true,
     prompt: WORKER-PROMPT.md の内容
   )
   ```

3. **進捗監視**: 監視UIを表示し、完了通知を出力

### なぜドキュメント改善でもworktreeが必要か

| 理由 | 説明 |
|------|------|
| **変更の追跡性** | PRごとに差分が明確になる |
| **並列実行** | Wave内タスクを同時実行可能 |
| **ワークフローの一貫性** | 全タスクで同じ手順を踏むことで学習効果が高まる |
| **自己検証** | 実装した機能（監視UI、エスカレーション等）を自分で使用して検証 |

### ❌ 禁止事項

- **直接編集の禁止**: 親Agentが直接 Edit/Write ツールでファイルを編集することは禁止
- **worktreeスキップの禁止**: 「ドキュメント改善だから不要」という判断は禁止
- **子Agent省略の禁止**: 「簡単なタスクだから」という理由での省略は禁止

### ⚠️ 違反時の対応

上記手順をスキップした場合、以下の問題が発生します：
- 並列実行の機会損失
- 進捗監視の欠如
- エスカレーション機能の未検証
- ワークフローの学習効果低下

**必ず3層アーキテクチャ（親→子→サブエージェント）を守ってください。**

---

## 🔴 重要: 出力指示

オーケストレーション完了後、**必ず以下の形式で「完了サマリー」を目立つように表示すること**:

### 必須出力フォーマット

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
完了：オーケストレーションが完了しました。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 サマリー:
  総所要時間: XX分
  完了SPEC: X
  完了タスク: X/X
  平均カバレッジ: XX%

結果：全ての処理が正常に終了しました。
通知：/aad:retro で振り返りの実行を推奨します。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 理由

- オーケストレーション完了時、全体の成果を一目で把握できるようにする
- 次のステップ（振り返り）への導線を明確にする
- 長時間実行後の重要な区切りとして視覚的に目立たせる

---

SPECからタスク分割、並列開発、統合まで全て自動実行します。

## アーキテクチャ概要

```
親 Claude Code (このセッション)
    │
    ├─→ 子 Claude Code (SPEC-001担当) ──→ サブエージェント (T01, T02, T03...)
    └─→ 子 Claude Code (SPEC-002担当) ──→ サブエージェント (T01, T02...)
```

- **親**: 複数SPECの管理、人間とのインターフェース、エスカレーション処理
- **子**: SPEC単位のブランチ管理、Wave計画、軽微な判断を自律実行
- **サブエージェント**: 個々のタスクの実装・テスト

## 実行内容

1. **SPEC確認**
   - `.aad/specs/SPEC-XXX.md` を読み込み
   - 承認済みか確認（人間承認必須）

2. **タスク分割**（必要な場合）
   - `/aad:tasks SPEC-XXX` を実行
   - GitHub Issues作成（`--no-issues`未指定時）

3. **進捗ディレクトリ初期化**
   - `.aad/progress/` ディレクトリ作成
   - `orchestrator.json` 初期化

4. **子 Claude Code 起動**
   - SPEC単位で子を起動（Task ツール使用）
   - `run_in_background=true` で並列実行
   - 各子は独立したコンテキストで動作

5. **進捗監視**
   - 3秒間隔でポーリング（TaskOutput使用）
   - エスカレーション発生時は人間に質問
   - 完了した子から順次確認

6. **エスカレーション処理**
   - 子からのエスカレーションを受信
   - `.aad/progress/SPEC-XXX/blocks/` から質問を読み取り
   - 人間に AskUserQuestion で質問
   - 回答を保存し、新しい子を resume モードで起動

7. **最終確認**
   - 全SPECの完了を確認
   - 人間に最終承認を依頼
   - worktree 削除

8. **振り返り**（オプション）
   - 全タスク完了後、`/aad:retro` 実行

## 使用方法

### 基本的な使用

```bash
# 単一SPEC実行
/aad:orchestrate SPEC-001

# 複数SPEC同時実行
/aad:orchestrate SPEC-001 SPEC-002
```

### オプション

```bash
# ドライラン（実行前確認）
/aad:orchestrate SPEC-001 --dry-run

# 中断からの再開
/aad:orchestrate --resume

# 実行状況確認
/aad:orchestrate --status

# GitHub Issues作成をスキップ
/aad:orchestrate SPEC-001 --no-issues

# 自律実行モード（人間介入を最小化）
/aad:orchestrate SPEC-001 --autonomous
```

### 自律実行モード（`--autonomous`）

人間介入を最小化し、可能な限り自律的にタスクを実行します。

**自律判断可能な項目**（人間介入なし）:
- 軽微な設計判断（既存パターンに従う場合）
- コーディング規約に関する判断
- テストのエッジケース追加
- 依存ライブラリのマイナーバージョンアップ
- ドキュメントの更新
- リファクタリング（既存機能を壊さない範囲）

**エスカレーション維持項目**（人間承認必須）:
- セキュリティ関連判断
- アーキテクチャ変更
- 仕様の重大な曖昧さ
- 外部API/サービスの選択
- 破壊的変更（Breaking Changes）
- 本番環境への影響がある変更

**動作**:
- 自律判断可能な項目は、子Agentが自動的に判断して記録
- エスカレーション維持項目は、通常通り人間に質問
- 判断内容は `.aad/progress/SPEC-XXX/autonomous-decisions.json` に記録

**例**:
```bash
/aad:orchestrate SPEC-001 --autonomous

# 自律判断の例:
# - テストでのエッジケース追加 → 自動判断（記録）
# - 既存パターンに従うUI実装 → 自動判断（記録）
# - 外部API選択 → エスカレーション（人間承認）
```

## 実装詳細

### Step 1: 事前確認

```
1. 指定されたSPECファイルの存在を確認
2. タスクファイルの存在を確認（なければ /aad:tasks 実行）
3. 進捗ディレクトリを初期化
```

### Step 2: 子 Claude Code 起動

```
各SPECに対して:
  1. CHILD-PROMPT.md テンプレートを読み込み
  2. {{SPEC_ID}} などを置換
  3. Task ツールで子を起動:
     - description: "SPEC-001を実行"
     - subagent_type: "general-purpose"
     - prompt: テンプレートから生成したプロンプト
     - run_in_background: true
     - max_turns: 200 (60分相当)
  4. 返された taskId を記録
```

### Step 3: 監視ループ

**重要: 監視中は視覚的フィードバックを提供してください**

待機中の視覚的フィードバック例:
```
🔄 監視中...
┌─────────────────────────────────┐
│ SPEC-009 [████████░░░░] 67%     │
│ ├─ T01 ✅ 完了 (3分前)          │
│ ├─ T02 ✅ 完了 (1分前)          │
│ ├─ T03 🔄 実行中 (5分経過)      │
│ └─ T04 ⏸️ 待機中                │
└─────────────────────────────────┘

⏱️  経過時間: 10分
📍 Worktree: /path/to/worktree-T03
```

```python
while 未完了の子がある:
    for each taskId in 実行中タスク:
        result = TaskOutput(taskId, block=false, timeout=1000)

        if result.status == "completed":
            # 完了処理
            - spec-status.json を確認
            - 完了したタスクのPRを確認
            - orchestrator.json を更新
            - 完了通知を表示:
              "✅ SPEC-XXX-TXX 完了！ (PR: #XX, 所要時間: XX分)"

        elif result.status == "escalate":
            # エスカレーション処理
            - result.type を確認（"question" または "permission" または "split"）

            # 通常のエスカレーション（質問）
            if result.type == "question":
                - blocks/*.md を読み取り
                - 人間に AskUserQuestion で質問
                - 回答を *-answer.json に保存
                - 新しい子を resume モードで起動

            # 権限エスカレーション
            elif result.type == "permission":
                - permissions/{{PERM_ID}}.json を読み取り
                - 権限要求の詳細を確認:
                  * type: file_write, command_exec, api_call, etc.
                  * resource: 対象リソース
                  * reason: 必要な理由
                  * alternatives: 代替案
                - 人間に承認を依頼:
                  ```
                  ⚠️  権限要求: {{PERM_ID}}

                  タスク: {{TASK_ID}}
                  種類: {{type}}
                  リソース: {{resource}}

                  理由:
                  {{reason}}

                  代替案:
                  {{alternatives}}

                  この権限を承認しますか？ (y/n)
                  制約を追加する場合は、制約内容を入力してください:
                  ```
                - 承認された場合:
                  * permissions/{{PERM_ID}}-approved.json を作成
                  * constraints を記録
                  * 新しい子を resume モードで起動
                - 拒否された場合:
                  * permissions/{{PERM_ID}}-rejected.json を作成
                  * reason を記録
                  * タスクを failed としてマーク

            # タスク分割エスカレーション
            elif result.type == "split":
                - splits/{{TASK_ID}}-split.json を読み取り
                - 分割内容を確認:
                  * originalTaskId
                  * subtasks (id, description, complexity, dependencies)
                  * reason
                - 人間に確認:
                  ```
                  📊 タスク分割提案: {{TASK_ID}}

                  理由: {{reason}}

                  サブタスク:
                  {{#each subtasks}}
                    - {{id}}: {{description}} ({{complexity}})
                      依存関係: {{dependencies}}
                  {{/each}}

                  この分割を承認しますか？ (y/n)
                  ```
                - 承認された場合:
                  * 元タスクを「分割済み」としてマーク
                  * サブタスクをWaveに追加
                  * サブタスクを実行
                - 拒否された場合:
                  * 元タスクを failed としてマーク
                  * 理由を記録

        elif result.status == "failed":
            # エラー処理
            - エラー内容を表示
            - 人間に対応を確認

        else:
            # 継続中 - 監視UIを更新
            - 進捗状況を表示（上記の視覚的フィードバック形式）
            - 現在のタスク状態を表示
            - Worktree配下で実行されていることを確認:
              "📍 Worktree確認: タスクは {worktree_path} で実行中"

    wait(3秒)
```

### Step 4: エスカレーション処理の詳細

```
1. エスカレーションを受信:
   - taskId から SPEC ID を特定
   - result.blockId から blocks/*.md のパスを特定

2. 質問内容を読み取り:
   - blocks/T01-001.md を読む
   - 状況、質問、選択肢、推奨を抽出

3. 人間に質問:
   - AskUserQuestion を使用
   - 選択肢を提示
   - 回答を取得

4. 回答を保存:
   - .aad/progress/SPEC-001/T01-001-answer.json に保存
   ```json
   {
     "blockId": "T01-001",
     "question": "セッション管理方式をどちらにしますか？",
     "answer": "JWT",
     "answeredAt": "2026-01-14T12:00:00Z"
   }
   ```

5. 新しい子を起動:
   - MODE: "resume"
   - prompt に "T01-001-answer.json に回答あり" を含める
```

### Step 5: 完了確認

```
全ての子が completed になったら:
  1. 各SPECのブランチを確認
  2. 人間に最終承認を依頼:
     "以下のSPECが完了しました。mainにマージしますか？
      - SPEC-001: feature/SPEC-001 (PR: #18, #19, #20)
      - SPEC-002: feature/SPEC-002 (PR: #21, #22)"

  3. 承認後:
     - 各SPECブランチをmainにマージ
     - worktree を削除
     - orchestrator.json を更新
```

## 出力例

```
全自動オーケストレーションを開始します

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Phase 1: SPEC確認
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ SPEC-001: ユーザー認証機能
   ステータス: Approved
   タスク数: 5

✅ SPEC-002: 管理画面機能
   ステータス: Approved
   タスク数: 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Phase 2: 子 Claude Code 起動
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🚀 子を起動:
   - SPEC-001 (taskId: abc123)
   - SPEC-002 (taskId: def456)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Phase 3: 進捗監視
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⏳ 監視中...

[3分後]
📊 SPEC-001:
   Wave 1 完了: T01 ✅
   Wave 2 実行中: T02 ⏳

📊 SPEC-002:
   Wave 1 実行中: T01 ⏳

[10分後]
🟡 SPEC-001 からエスカレーション:
   タスク: T03
   理由: セッション管理方式の選択が必要

質問: セッション管理方式をどちらにしますか？
選択肢:
  1. JWT - ステートレス、スケーラブル（推奨）
  2. Server Session - シンプル、即座に無効化可能

回答: JWT

✅ エスカレーション解決
   新しい子を起動（resume モード）

[30分後]
✅ SPEC-001 完了
   - 完了タスク: 5/5
   - PR: #18, #19, #20, #21, #22

✅ SPEC-002 完了
   - 完了タスク: 3/3
   - PR: #23, #24, #25

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Phase 4: 最終確認
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

以下のSPECが完了しました。mainにマージしますか？
  - SPEC-001: feature/SPEC-001 (PR: 5個)
  - SPEC-002: feature/SPEC-002 (PR: 3個)

→ はい

✅ マージ完了
✅ worktree 削除完了

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
完了：オーケストレーションが完了しました。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 サマリー:
  総所要時間: 35分
  完了SPEC: 2
  完了タスク: 8/8
  平均カバレッジ: 87%

結果：全ての処理が正常に終了しました。
通知：/aad:retro で振り返りの実行を推奨します。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## ドライランモード

実行前に計画を確認

```
/aad:orchestrate SPEC-001 SPEC-002 --dry-run

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
実行計画
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SPEC-001: ユーザー認証機能 (5タスク)
  Wave 1: T01
  Wave 2: T02
  Wave 3: T03, T04, T05 (並列)

SPEC-002: 管理画面機能 (3タスク)
  Wave 1: T01, T02 (並列)
  Wave 2: T03

予想所要時間: 30-60分
最大並列度: SPEC 2個 × Task 2個 = 4並列

この計画で実行しますか？ (y/n)
```

## 状態確認

実行中の状態を確認

```
/aad:orchestrate --status

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
実行状況
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SPEC-001: in_progress (経過時間: 15分)
  ✅ T01: completed (PR: #18)
  ⏳ T02: in_progress
  ⏸️  T03: blocked (escalation待ち)
  ⬜ T04: pending
  ⬜ T05: pending

SPEC-002: in_progress (経過時間: 15分)
  ⏳ T01: in_progress
  ⬜ T02: pending
  ⬜ T03: pending

エスカレーション待ち: 1件
  - SPEC-001-T03: セッション管理方式の選択
```

## resume モード

中断したオーケストレーションを再開：

```
/aad:orchestrate --resume

.aad/progress/orchestrator.json を読み込み中...

以下のSPECを再開しますか？
  - SPEC-001: in_progress (T03がblocked)
  - SPEC-002: in_progress (T02実行中)

→ はい

SPEC-001のエスカレーションを処理中...
```

## 進捗ファイル構造

```
.aad/progress/
├── orchestrator.json              # 親の状態
├── SPEC-001/
│   ├── spec-status.json           # 子の状態
│   ├── T01-state.json             # サブエージェントの状態
│   ├── T03-state.json
│   ├── T03-001-answer.json        # 回答
│   └── blocks/
│       └── T03-001.md             # ブロック報告
└── SPEC-002/
    ├── spec-status.json
    └── ...
```

## エスカレーションレベル

### 子が自律判断可能（人間への質問不要）
- コーディング規約に関する判断
- 軽微な設計判断（既存パターンに従う）
- テストのエッジケース追加

→ 子が autonomousDecisions に記録

### 親にエスカレーション（人間への質問必要）
- アーキテクチャに影響する判断
- セキュリティに関わる判断
- 外部API/サービスの選択
- 仕様の曖昧さの解消

→ 親が人間に AskUserQuestion

## 関連コマンド

- `/aad:tasks` - タスク分割のみ
- `/aad:status` - 進捗確認
- `/aad:retro` - 振り返り

## 注意事項

- 必ずSPECが承認済みであることを確認してください
- 長時間実行されるため、安定した環境が必要です
- エスカレーション発生時は速やかに対応してください
- ホストマシンに必要なツール（go, node, pythonなど）がインストールされていることを確認してください
- 子の最大実行時間は60分です
- SPEC並列度は最大2、Task並列度は最大2です（合計4並列）
