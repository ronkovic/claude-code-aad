Clone the later half of the current conversation, discarding earlier context to reduce token usage while preserving recent work.

## 🔴 重要: 出力指示

ハーフクローン完了後、**必ず以下の形式で出力すること**:

### キーワード凡例
- **解：** - 分析結果、答え
- **告：** - システム通知、実行確認、異常完了報告
- **確認：** - 検知、照合
- **成功しました：** - 正常完了報告
- **否：** - 否定

### 必須出力フォーマット

```
成功しました：会話の後半をクローンしました。
解：`claude -r` で [HALF-CLONE <timestamp>] を選択してください。
解：前半のコンテキストは破棄され、トークン使用量が削減されました。
```

---

Steps:
1. Get the current session ID and project path: `tail -1 ~/.claude/history.jsonl | jq -r '[.sessionId, .project] | @tsv'`
2. Find half-clone-conversation.sh with bash: `find ~/.claude -name "half-clone-conversation.sh" 2>/dev/null | sort -V | tail -1`
   - This finds the script whether installed via plugin or manual symlink
   - Uses version sort to prefer the latest version if multiple exist
3. Run: `<script-path> <session-id> <project-path>`
   - Always pass the project path from the history entry, not the current working directory
4. Tell the user they can access the half-cloned conversation with `claude -r` and look for the one marked `[HALF-CLONE <timestamp>]` (e.g., `[HALF-CLONE Jan 7 14:30]`)
