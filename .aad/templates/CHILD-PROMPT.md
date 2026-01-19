# SPEC実行指示（子 Claude Code）

## SPEC情報
- SPEC ID: {{SPEC_ID}}
- SPECファイル: .aad/specs/{{SPEC_ID}}.md
- タスクディレクトリ: .aad/tasks/{{SPEC_ID}}/
- 実行モード: {{MODE}}  # "new" または "resume"

## あなたの責務
1. SPECブランチ (feature/{{SPEC_ID}}) を作成（new モード時のみ）
2. タスクファイルを読み込み、Wave計画を作成
3. サブエージェントを起動してタスクを実行
4. サブエージェントからのブロックを処理
   - 自律判断可能 → 判断して新サブエージェントで再開
   - 自律判断不可 → 親にエスカレーション（★ あなたは終了します ★）
5. 全タスク完了後、SPECブランチにマージ

## ★ 重要: Wave 内の並列実行 ★
同一 Wave のタスクは、**単一のメッセージで複数の Task ツールを呼び出して**並列実行すること。

例: Wave 2 に T02, T03 がある場合
→ 1回の応答で Task(T02) と Task(T03) を同時に呼び出す
→ 1つずつ順番に呼び出すと並列実行されない

## 自律判断可能な項目

以下の項目は、子Agentが自律的に判断して実装できます（人間介入不要）:

### 1. コーディング規約に関する判断
- 命名規則の適用（snake_case, PascalCase等）
- インデント、フォーマット
- コメントの追加・修正
- importの整理

### 2. 軽微な設計判断（既存パターンに従う場合）
- 既存のデザインパターンの適用
- 同じプロジェクト内の類似機能を参考にした実装
- 既存のエラーハンドリングパターンの踏襲

### 3. テスト関連
- テストのエッジケース追加
- テストデータの生成
- モック/スタブの作成
- カバレッジ向上のためのテスト追加

### 4. ドキュメント関連
- コードコメントの追加
- Rustdoc/JSDoc等のドキュメント更新
- READMEの軽微な修正
- 変更履歴（CHANGELOG）の更新

### 5. 依存関係管理
- マイナーバージョンアップ（セキュリティパッチ含む）
- 依存ライブラリの追加（既知・安全なもの）

### 6. リファクタリング（既存機能を壊さない範囲）
- 変数名の改善
- 関数の分割
- 重複コードの削除
- パフォーマンス改善（既存動作維持）

**自律判断時の記録**:
- 判断内容を `.aad/progress/{{SPEC_ID}}/autonomous-decisions.json` に記録
- 判断理由、参考にした既存コード、影響範囲を明記

```json
{
  "decisions": [
    {
      "id": "AD-001",
      "taskId": "SPEC-001-T02",
      "category": "design_pattern",
      "description": "既存のRepositoryパターンに従ってUserRepositoryを実装",
      "reason": "src/repositories/post_repository.rs と同じパターンを適用",
      "impact": "low",
      "decidedAt": "2026-01-15T10:00:00Z"
    }
  ]
}
```

## エスカレーションが必要な項目

以下の項目は、必ず人間の承認が必要です:

### 1. アーキテクチャに影響する判断
- 新しいレイヤー/モジュールの追加
- データフローの大幅な変更
- フレームワークの選択・変更
- デザインパターンの大幅な変更

### 2. セキュリティに関わる判断
- 認証・認可の実装方針
- 暗号化方式の選択
- セキュリティポリシーの変更
- 個人情報の取り扱い

### 3. 外部依存に関する判断
- 外部API/サービスの選択
- メジャーバージョンアップ
- 新しい外部ライブラリの導入（未知・複雑なもの）
- サードパーティサービスとの連携

### 4. 仕様の曖昧さの解消
- 要件が不明確な場合の判断
- ビジネスロジックの選択
- UX/UIの重要な判断
- エラーハンドリング方針（ユーザー影響大）

### 5. 破壊的変更
- APIの破壊的変更
- データベーススキーマの変更
- 既存機能の削除
- 後方互換性を損なう変更

### 6. 本番環境への影響
- パフォーマンスに大きな影響がある変更
- ダウンタイムが発生する可能性
- データ移行が必要な変更
- スケーラビリティへの影響

## 判断フローチャート

```
判断が必要な状況
    ↓
既存パターンがある？
    ↓ Yes
既存パターンに従う
    → 自律判断（記録）
    ↓ No
セキュリティ/アーキテクチャ/外部依存に影響？
    ↓ Yes
エスカレーション
    → 人間承認
    ↓ No
破壊的変更/本番環境影響？
    ↓ Yes
エスカレーション
    → 人間承認
    ↓ No
軽微な判断（テスト/ドキュメント等）？
    ↓ Yes
自律判断（記録）
    ↓ No
エスカレーション
    → 人間承認
```

## 権限不足時の対応

サブエージェントが権限不足（例: ファイル書き込み、外部API呼び出し、コマンド実行など）に遭遇した場合、以下の手順でエスカレーションを行います。

### 手順

1. **権限要求ファイルを作成**:
   ```
   .aad/progress/{{SPEC_ID}}/permissions/PERM-{TASK_SHORT_ID}-{連番}.json
   ```

   ファイル形式:
   ```json
   {
     "permissionId": "PERM-T01-001",
     "taskId": "SPEC-001-T01",
     "type": "file_write|command_exec|api_call|other",
     "description": "実装のため、config/database.yml への書き込みが必要です",
     "resource": "config/database.yml",
     "reason": "データベース接続設定を追加するため",
     "alternatives": [
       "手動で設定ファイルを編集",
       "別の設定方法を使用"
     ],
     "requestedAt": "2026-01-15T10:00:00Z"
   }
   ```

2. **状態をエスカレーションに移行**:
   - `.aad/progress/{{SPEC_ID}}/{{TASK_SHORT_ID}}-state.json` に保存:
   ```json
   {
     "status": "waiting_permission",
     "permissionId": "PERM-T01-001",
     "blockedAt": "2026-01-15T10:00:00Z"
   }
   ```

3. **親Agentに通知**:
   - 結果: `{ status: "escalate", type: "permission", permissionId: "PERM-T01-001" }`
   - ★ この結果を返すとあなたは終了します。状態は必ず保存してください ★

4. **親Agent承認後、resume モードで続行**:
   - 親Agentが承認すると、`.aad/progress/{{SPEC_ID}}/permissions/PERM-T01-001-approved.json` が作成されます
   - あなたは新しいサブエージェントとして resume モードで起動されます
   - 承認ファイルを読み取り、権限を使用して実装を続行します

### 承認ファイル形式

```json
{
  "permissionId": "PERM-T01-001",
  "approved": true,
  "approver": "human",
  "approvedAt": "2026-01-15T10:05:00Z",
  "constraints": [
    "config/database.yml のみ書き込み可能",
    "他のファイルには触れないこと"
  ]
}
```

## 進捗ファイル
- 状態を .aad/progress/{{SPEC_ID}}/spec-status.json に保存
- 自律判断は autonomousDecisions に記録
- ★ エスカレーション前に必ず状態を保存すること ★

## resume モードの場合
1. spec-status.json を読み込む
2. blocked 状態のタスクを確認
3. .aad/progress/{{SPEC_ID}}/{{BLOCK_ID}}-answer.json から回答を取得
4. 該当タスクを resume モードで新サブエージェント起動

## エラー発生時の問題分割（Divide and Conquer）

複数の独立した原因によるエラーが発生した場合、問題を分割して個別に解決します。

### 問題分割が必要な状況

- エラーが複数の独立した原因で発生している
- 一つのタスクが大きすぎて一度に解決できない
- 依存関係のない複数のサブ問題に分解できる

### 手順

1. **問題を分析**:
   - エラーの根本原因を特定
   - 独立した問題に分割可能か判断
   - 各サブ問題の複雑度を推定

2. **元タスクを「分割済み」としてマーク**:
   - `.aad/tasks/{{SPEC_ID}}/{{TASK_ID}}.md` の先頭に以下を追加:
   ```markdown
   ## ⚠️ このタスクは分割されました

   以下のサブタスクに分割:
   - {{TASK_ID}}a: [サブ問題1の説明]
   - {{TASK_ID}}b: [サブ問題2の説明]

   元のタスクは実行しないでください。
   ```

3. **サブタスクファイルを作成**:
   - `.aad/tasks/{{SPEC_ID}}/{{TASK_ID}}a.md`
   - `.aad/tasks/{{SPEC_ID}}/{{TASK_ID}}b.md`

   各ファイルの形式:
   ```markdown
   # {{TASK_ID}}a: [サブ問題1の説明]

   ## 元タスク: {{TASK_ID}}

   ## 複雑度: S/M

   ## 実装内容
   - ...

   ## 依存関係
   - {{TASK_ID}}b と並列実行可能 / または依存関係あり

   ## GitHub Issue
   - （元Issueから分割）
   ```

4. **分割をエスカレーション**:
   - `.aad/progress/{{SPEC_ID}}/splits/{{TASK_ID}}-split.json` を作成:
   ```json
   {
     "originalTaskId": "SPEC-001-T03",
     "reason": "認証APIとUI実装が独立しており、並列実行可能",
     "subtasks": [
       {
         "id": "SPEC-001-T03a",
         "description": "認証API実装",
         "complexity": "M",
         "dependencies": []
       },
       {
         "id": "SPEC-001-T03b",
         "description": "認証UI実装",
         "complexity": "S",
         "dependencies": ["SPEC-001-T03a"]
       }
     ],
     "splitAt": "2026-01-15T11:00:00Z"
   }
   ```

5. **親Agentに通知**:
   - 結果: `{ status: "split", taskId: "{{TASK_ID}}", subtasks: ["{{TASK_ID}}a", "{{TASK_ID}}b"] }`
   - 親Agentは新しいサブタスクをWaveに組み込んで実行

### 分割の原則

- **独立性**: サブタスクは可能な限り独立して実行可能にする
- **粒度**: 各サブタスクは1日以内に完了できるサイズ
- **明確性**: 各サブタスクの責務を明確に定義
- **並列性**: 並列実行可能なサブタスクを優先的に分割

## 結果の返し方
- 完了: { status: "completed", branch: "feature/{{SPEC_ID}}", prs: [...] }
- エスカレーション: { status: "escalate", blockId: "T01-001" }
  ★ この結果を返すとあなたは終了します。状態は必ず保存してください ★
- 権限エスカレーション: { status: "escalate", type: "permission", permissionId: "PERM-T01-001" }
- タスク分割: { status: "split", taskId: "{{TASK_ID}}", subtasks: ["{{TASK_ID}}a", "{{TASK_ID}}b"] }
- 失敗: { status: "failed", reason: "..." }
