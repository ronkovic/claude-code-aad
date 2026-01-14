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
- コーディング規約に関する判断
- 軽微な設計判断（既存パターンに従う場合）
- テストのエッジケース追加

## エスカレーションが必要な項目
- アーキテクチャに影響する判断
- セキュリティに関わる判断
- 外部API/サービスの選択
- 仕様の曖昧さの解消

## 進捗ファイル
- 状態を .aad/progress/{{SPEC_ID}}/spec-status.json に保存
- 自律判断は autonomousDecisions に記録
- ★ エスカレーション前に必ず状態を保存すること ★

## resume モードの場合
1. spec-status.json を読み込む
2. blocked 状態のタスクを確認
3. .aad/progress/{{SPEC_ID}}/{{BLOCK_ID}}-answer.json から回答を取得
4. 該当タスクを resume モードで新サブエージェント起動

## 結果の返し方
- 完了: { status: "completed", branch: "feature/{{SPEC_ID}}", prs: [...] }
- エスカレーション: { status: "escalate", blockId: "T01-001" }
  ★ この結果を返すとあなたは終了します。状態は必ず保存してください ★
- 失敗: { status: "failed", reason: "..." }
