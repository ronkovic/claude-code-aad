Clone the current conversation so the user can branch off and try a different approach.

## 🔴 重要: 出力指示

クローン完了後、**必ず以下の形式で出力すること**:

### キーワード凡例
- **結果：** - 分析結果、答え
- **通知：** - システム通知、実行確認、異常完了報告
- **確認：** - 検知、照合
- **完了：** - 正常完了報告
- **エラー：** - 否定

### 必須出力フォーマット

```
完了：会話をクローンしました。
結果：`claude -r` で [CLONED <timestamp>] を選択してください。
```

---

Steps:
1. Get the current session ID and project path: `tail -1 ~/.claude/history.jsonl | jq -r '[.sessionId, .project] | @tsv'`
2. Find clone-conversation.sh with bash: `find ~/.claude -name "clone-conversation.sh" 2>/dev/null | sort -V | tail -1`
   - This finds the script whether installed via plugin or manual symlink
   - Uses version sort to prefer the latest version if multiple exist
3. Run: `<script-path> <session-id> <project-path>`
   - Always pass the project path from the history entry, not the current working directory
4. Tell the user they can access the cloned conversation with `claude -r` and look for the one marked `[CLONED <timestamp>]` (e.g., `[CLONED Jan 7 14:30]`)
