Write or update a handoff document (in Japanese) so the next agent with fresh context can continue this work.

## 🔴 重要: 出力指示

ハンドオフドキュメント作成後、**必ず以下の形式で出力すること**:

### キーワード凡例
- **解：** - 分析結果、答え
- **告：** - システム通知、実行確認、異常完了報告
- **確認：** - 検知、照合
- **成功しました：** - 正常完了報告
- **否：** - 否定

### 必須出力フォーマット

```
成功しました：ハンドオフドキュメントを作成しました。
解：パス: HANDOFF.md
告：新しいセッションで上記ファイルを読み込んでください。
```

---

Steps:
1. Check if HANDOFF.md already exists in the project
2. If it exists, read it first to understand prior context before updating
3. Create or update the document **in Japanese** with:
   - **目標 (Goal)**: What we're trying to accomplish
   - **現在の進捗 (Current Progress)**: What's been done so far
   - **うまくいったこと (What Worked)**: Approaches that succeeded
   - **うまくいかなかったこと (What Didn't Work)**: Approaches that failed (so they're not repeated)
   - **次のステップ (Next Steps)**: Clear action items for continuing

4. Save as HANDOFF.md in the project root
5. Tell the user the file path so they can start a fresh conversation with just that path

Important: The entire HANDOFF.md document must be written in Japanese, following this format:

```markdown
# ハンドオフドキュメント

## 目標
[達成しようとしていることを日本語で記述]

## 現在の進捗
### 完了したこと
- [完了したタスク1]
- [完了したタスク2]

## うまくいったこと
- [成功したアプローチ1とその詳細]
- [成功したアプローチ2とその詳細]

## うまくいかなかったこと
- [失敗したアプローチ1] - 理由: [なぜ失敗したか]
- [失敗したアプローチ2] - 理由: [なぜ失敗したか]

## 次のステップ
1. [次に実行すべきアクション1]
2. [次に実行すべきアクション2]
3. [次に実行すべきアクション3]
```
