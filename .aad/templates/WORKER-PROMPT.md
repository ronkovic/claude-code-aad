# タスク実行指示（サブエージェント）

## タスク情報
- タスクID: {{TASK_ID}}
- タスクファイル: .aad/tasks/{{SPEC_ID}}/{{TASK_ID}}.md
- worktree: ../{{PROJECT_NAME}}-{{TASK_SHORT_ID}}/

## 実行手順

1. **状態確認**
   - .aad/progress/{{SPEC_ID}}/{{TASK_SHORT_ID}}-state.json が存在するか確認
   - 存在する場合は resume モード、なければ新規実行

2. **worktree 作成**（新規実行時のみ）
   - ブランチ: feature/{{TASK_ID}}
   - パス: ../{{PROJECT_NAME}}-{{TASK_SHORT_ID}}/

3. **実装**
   - タスクファイルの受け入れ基準に従って実装
   - TDD: テスト先行で実装

4. **ブロック発生時**
   - .aad/progress/{{SPEC_ID}}/{{TASK_SHORT_ID}}-state.json に現在状態を保存
   - .aad/progress/{{SPEC_ID}}/blocks/{{TASK_SHORT_ID}}-XXX.md に質問を記載
   - 結果: { status: "blocked", blockId: "{{TASK_SHORT_ID}}-XXX" }

5. **完了時**
   - 全テスト green 確認
   - カバレッジ 80% 以上確認
   - gh pr create --draft でPR作成
   - 結果: { status: "completed", pr: "#XX" }

## 禁止事項
- main ブランチへの直接 push
- 他のタスクの worktree への変更
- SPECブランチへの直接マージ（子 Claude Code が行う）
