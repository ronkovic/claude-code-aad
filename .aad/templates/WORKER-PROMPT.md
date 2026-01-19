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

   **⚠️ 必須: worktree配下での実行確認**

   PR作成前に、必ずworktree配下（`../{{PROJECT_NAME}}-{{TASK_SHORT_ID}}/`）で以下を実行してください:

   ```bash
   cd ../{{PROJECT_NAME}}-{{TASK_SHORT_ID}}/
   pwd  # worktree配下であることを確認
   ```

   **包括的PR作成前チェックリスト**:

   - [ ] **全テストgreen**
     ```bash
     cargo test --all
     # または
     npm test
     ```

   - [ ] **カバレッジ80%以上**
     ```bash
     cargo llvm-cov --html
     # カバレッジレポートを確認
     ```

   - [ ] **Lint通過**
     ```bash
     cargo clippy --all -- -D warnings
     # または
     npm run lint
     ```

   - [ ] **Type check通過**（該当する場合）
     ```bash
     cargo check
     # または
     npm run type-check
     ```

   - [ ] **コンフリクトチェック実施**
     ```bash
     # mainブランチとのマージをシミュレーション
     git fetch origin main
     git merge --no-commit --no-ff origin/main

     # コンフリクトがなければ
     git merge --abort
     ```

   - [ ] **CI dry-run実施**
     ```bash
     # ローカルでCIと同じチェックを実行
     cargo test --all
     cargo clippy --all -- -D warnings
     cargo fmt --all -- --check
     ```

   **全チェック通過後、PR作成**:
   ```bash
   gh pr create --draft --title "feat({{TASK_ID}}): [タスク概要]" \
                 --body "## 概要

   {{タスクの説明}}

   ## 変更内容
   - ...

   ## テスト結果
   - ✅ 全テストgreen
   - ✅ カバレッジ: XX%
   - ✅ Lint通過
   - ✅ コンフリクトなし

   Closes #XX"
   ```

   - 結果: { status: "completed", pr: "#XX" }

## 禁止事項
- main ブランチへの直接 push
- 他のタスクの worktree への変更
- SPECブランチへの直接マージ（子 Claude Code が行う）
