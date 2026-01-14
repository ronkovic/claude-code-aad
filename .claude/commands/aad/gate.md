# 品質ゲートチェック

各フェーズの完了条件をチェックし、次フェーズへの移行可否を判定します。

## 実行内容

1. **指定フェーズのチェック**
   - SPEC / TASKS / TDD / REVIEW / RETRO / MERGE

2. **完了条件の検証**
   - 各フェーズの品質基準を確認
   - 人間承認が必要なフェーズは承認状態を確認

3. **レポート生成**
   - 合格/不合格の判定
   - 不合格の場合は改善項目を提示

4. **GitHub Issue更新**（GitHub連携使用時）
   - 品質ゲート通過状況をコメント

## 使用方法

```
/aad:gate SPEC      # SPEC品質ゲート
/aad:gate TASKS     # TASKS品質ゲート
/aad:gate TDD       # TDD品質ゲート
/aad:gate REVIEW    # REVIEW品質ゲート
/aad:gate RETRO     # RETRO品質ゲート
/aad:gate MERGE     # MERGE品質ゲート
```

### auto-commitオプション

TDD品質ゲート合格時に自動的にコミットを作成：

```
/aad:gate TDD --auto-commit
```

全フェーズをチェック：

```
/aad:gate --all
```

## 出力例

### SPECフェーズ

```
品質ゲートチェック: SPEC

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 SPEC-001: ユーザー認証機能
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ 受け入れ基準がテスト可能な形式で記述されている
✅ MoSCoWで優先度が設定されている
   Must: 3項目
   Should: 1項目
   Could: 1項目
   Won't: 2項目
✅ API仕様が明確に定義されている
✅ データモデルが定義されている
❌ 人間承認が完了していない

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
判定: ❌ 不合格
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

改善項目:
  ⚠️  人間承認を取得してください
     - SPEC-001.md の「✅ 承認」セクションを記入
     - 承認者と承認日を明記

承認後に再度チェックしてください:
  /aad:gate SPEC
```

### TDDフェーズ

```
品質ゲートチェック: TDD (SPEC-001-T01)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🧪 テスト結果
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ 全テストgreen
   Total: 25 passed, 0 failed
   Duration: 2.3s

✅ カバレッジ80%以上
   Statements: 85%
   Branches: 82%
   Functions: 90%
   Lines: 85%

✅ Lint通過
   0 errors, 0 warnings

✅ Type check通過
   0 errors

✅ PR作成完了
   PR: #18 (Draft)
   Branch: feature/SPEC-001-T01

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
判定: ✅ 合格
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

次のフェーズに進めます:
  /aad:gate REVIEW
```

### TDDフェーズ（auto-commit有効時）

```
品質ゲートチェック: TDD (SPEC-001-T01)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🧪 テスト結果
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ 全テストgreen
   Total: 25 passed, 0 failed
   Duration: 2.3s

✅ カバレッジ80%以上
   Statements: 85%
   Branches: 82%
   Functions: 90%
   Lines: 85%

✅ Lint通過
   0 errors, 0 warnings

✅ Type check通過
   0 errors

✅ PR作成完了
   PR: #18 (Draft)
   Branch: feature/SPEC-001-T01

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
判定: ✅ 合格
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔄 auto-commit実行中...

✅ コミット作成完了
   メッセージ: feat(SPEC-001-T01): implement user authentication

   - 品質ゲート: TDD合格
   - カバレッジ: 85%
   - テスト: 25 passed

   Refs #18

次のフェーズに進めます:
  /aad:gate REVIEW
```

## 各フェーズの完了条件

### SPEC（仕様）

- [ ] 受け入れ基準がテスト可能な形式で記述されている
- [ ] MoSCoWで優先度が設定されている
- [ ] API仕様が明確（該当する場合）
- [ ] データモデルが定義（該当する場合）
- [ ] **⚠️ 人間承認必須**

### TASKS（タスク分割）

- [ ] 全タスクにID（SPEC-XXX-TXX）が付与されている
- [ ] 依存関係が明記されている
- [ ] 複雑度（S/M/L）が設定されている
- [ ] GitHub Issuesが作成されている（GitHub連携使用時）
- [ ] HANDOFF.mdが更新されている
- [ ] **⚠️ 人間承認必須**

### TDD（開発）

- [ ] 全テストgreen
- [ ] カバレッジ80%以上
- [ ] Lint通過（0 errors）
- [ ] Type check通過
- [ ] `gh pr create --draft`でPR作成完了

### REVIEW（レビュー）

- [ ] AI自己レビュー完了
- [ ] CI green
- [ ] コンフリクトなし
- [ ] セキュリティスキャン通過
- [ ] **⚠️ 人間承認必須**

### RETRO（振り返り）

- [ ] .aad/retrospectives/にログが作成されている
- [ ] Keep/Problem/Tryが記載されている
- [ ] 技術的な学びが明記されている
- [ ] CLAUDE.md更新提案がある

### MERGE（統合）

- [ ] mainマージ完了
- [ ] Issue閉鎖（GitHub連携使用時）
- [ ] worktree削除
- [ ] デプロイ成功（該当する場合）

## 厳格モード

開発環境に応じて基準を調整：

```
/aad:gate TDD --strict         # より厳格な基準（カバレッジ90%等）
/aad:gate TDD --lenient        # 緩い基準（カバレッジ70%等）
```

## レポート出力

結果をファイルに保存：

```
/aad:gate --all --output=quality-report.md
```

## 関連コマンド

- `/aad:tasks` - タスク分割
- `/aad:worktree` - worktree作成
- `/aad:integrate` - PRマージ
- `/aad:retro` - 振り返り

## カスタマイズ

プロジェクト固有の基準を`CLAUDE.md`の「品質ゲート」セクションで定義できます：

```markdown
## 📊 品質ゲート

### プロジェクト固有の基準

TDDフェーズ:
- カバレッジ: 90%以上（厳格）
- パフォーマンステスト必須
- アクセシビリティスコア: 95以上
```

## auto-commit機能の詳細

### 概要

`--auto-commit` オプションを使用すると、TDD品質ゲート合格時に自動的にコミットを作成します。

### コミットメッセージ形式

Conventional Commits形式で自動生成：

```
<type>(<task-id>): <description>

- 品質ゲート: TDD合格
- カバレッジ: XX%
- テスト: XX passed

Refs #<issue-number>
```

### typeの自動判定

タスクファイル（`.aad/tasks/SPEC-XXX/TXX-xxx.md`）の内容から自動判定：

| タスク内容キーワード | type |
|---------------------|------|
| 新規、追加、実装 | feat |
| 修正、バグ、fix | fix |
| リファクタリング | refactor |
| テスト | test |
| ドキュメント、docs | docs |
| デフォルト | feat |

### 使用タイミング

- TDDサイクル完了時
- 全テストが通過し、品質基準を満たしている時
- PR作成後の最終確認時

### 注意事項

- コミットメッセージは自動生成されますが、後で修正可能です（`git commit --amend`）
- `--auto-commit` はデフォルトで無効です（明示的な指定が必要）
- 品質ゲートが不合格の場合はコミットされません

## 注意事項

- 品質ゲートは通過するまで次フェーズに進まないでください
- 人間承認が必要なフェーズは必ず承認を取得してください
- 不合格の場合は改善項目を確認し、修正後に再チェックしてください
- CI失敗時は原因を特定してから再実行してください
- 品質基準は「最低限」ではなく「望ましい水準」として設定されています
