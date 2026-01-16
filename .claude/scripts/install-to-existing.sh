#!/bin/bash
# 既存プロジェクトへのAADテンプレート導入スクリプト

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEMPLATE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET_DIR="${1:-.}"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 AADテンプレート導入ウィザード"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "対象ディレクトリ: $TARGET_DIR"
echo ""

# === 差分確認フェーズ ===
echo "📋 既存ファイルをチェック中..."

check_exists() {
  if [ -e "$TARGET_DIR/$1" ]; then
    echo "  ⚠️  $1 が存在します"
    return 0
  else
    echo "  ✅ $1 は新規作成されます"
    return 1
  fi
}

# ファイル/フォルダの存在チェック
CLAUDE_MD_EXISTS=false
GITIGNORE_EXISTS=false
CLAUDE_DIR_EXISTS=false
DOCS_EXISTS=false
AAD_EXISTS=false

check_exists "CLAUDE.md" && CLAUDE_MD_EXISTS=true
check_exists ".gitignore" && GITIGNORE_EXISTS=true
check_exists ".claude" && CLAUDE_DIR_EXISTS=true
check_exists "docs" && DOCS_EXISTS=true
check_exists "aad" && AAD_EXISTS=true

echo ""
echo "続行しますか？ (y/n)"
read -r CONTINUE
[ "$CONTINUE" != "y" ] && exit 0

# === バックアップフェーズ ===
BACKUP_DIR="$TARGET_DIR/.aad-backup-$(date +%Y%m%d%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo ""
echo "📦 バックアップを作成: $BACKUP_DIR"

# === 導入フェーズ ===

# コンテキスト管理ルールセクションを更新する関数
update_context_section() {
  local file="$1"
  local tmp_file
  tmp_file=$(mktemp)
  local new_section_file
  new_section_file=$(mktemp)

  # 新しいセクションを一時ファイルに書き込む
  cat > "$new_section_file" << 'CONTEXT_EOF'
## 🎯 コンテキスト管理ルール（70%ルール）

| 使用率 | ステータス | アクション |
|--------|------------|------------|
| 0-50% | 🟢 快適 | 通常作業 |
| 50-70% | 🟡 通知：注意 | 大きなタスクは新セッション推奨 |
| 70-85% | 🟠 通知：警告 | `/aad:handoff` 実行推奨 |
| 85-95% | 🔴 通知：危機的 | 即座に `/aad:handoff` |
| 95%+ | ⛔ 通知：限界 | 自動圧縮（制御不能） |
CONTEXT_EOF

  # 既存のセクションを検出して置換
  local in_section=0
  local printed=0

  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$line" =~ ^"## 🎯 コンテキスト管理ルール" ]]; then
      in_section=1
      if [[ "$printed" -eq 0 ]]; then
        cat "$new_section_file"
        echo ""
        printed=1
      fi
      continue
    fi

    if [[ "$in_section" -eq 1 ]]; then
      if [[ "$line" =~ ^"## " ]] || [[ "$line" =~ ^"---" ]]; then
        in_section=0
        echo "$line"
      fi
      continue
    fi

    echo "$line"
  done < "$file" > "$tmp_file"

  mv "$tmp_file" "$file"
  rm -f "$new_section_file"
}

# 新しいコンテキスト管理ルールテーブル（standardスタイル）- 追記用
CONTEXT_TABLE='## 🎯 コンテキスト管理ルール（70%ルール）

| 使用率 | ステータス | アクション |
|--------|------------|------------|
| 0-50% | 🟢 快適 | 通常作業 |
| 50-70% | 🟡 通知：注意 | 大きなタスクは新セッション推奨 |
| 70-85% | 🟠 通知：警告 | `/aad:handoff` 実行推奨 |
| 85-95% | 🔴 通知：危機的 | 即座に `/aad:handoff` |
| 95%+ | ⛔ 通知：限界 | 自動圧縮（制御不能） |'

# 1. CLAUDE.md
if [ "$CLAUDE_MD_EXISTS" = true ]; then
  cp "$TARGET_DIR/CLAUDE.md" "$BACKUP_DIR/"

  # 既存のコンテキスト管理ルールセクションをチェック
  if grep -q "コンテキスト管理ルール" "$TARGET_DIR/CLAUDE.md"; then
    echo ""
    echo "📋 既存のコンテキスト管理ルールセクションを検出しました"
    echo "  新しいフォーマット（standardスタイル）で上書きしますか？ (y/n)"
    read -r UPDATE_CONTEXT
    if [ "$UPDATE_CONTEXT" = "y" ]; then
      update_context_section "$TARGET_DIR/CLAUDE.md"
      echo "  ✅ コンテキスト管理ルールセクションを更新しました"
    fi
  else
    echo ""
    echo "CLAUDE.md にAADセクションを追記しますか？ (y/n)"
    read -r APPEND_CLAUDE
    if [ "$APPEND_CLAUDE" = "y" ]; then
      cat >> "$TARGET_DIR/CLAUDE.md" << CLAUDE_SECTION

---

## ⚙️ プロジェクト設定

| 設定 | 値 |
|------|-----|
| デフォルトブランチ | \`main\` |

**注**: デフォルトブランチは \`/aad:init\` で自動検出されます。

---

$CONTEXT_TABLE

CLAUDE_SECTION
      echo "  ✅ CLAUDE.md にAADセクションを追記しました"
    fi
  fi
else
  cp "$TEMPLATE_ROOT/CLAUDE.md" "$TARGET_DIR/"
  echo "  ✅ CLAUDE.md を作成しました"
fi

# 2. .gitignore
if [ "$GITIGNORE_EXISTS" = true ]; then
  cp "$TARGET_DIR/.gitignore" "$BACKUP_DIR/"
  # 重複チェック
  if ! grep -q ".aad/worktrees/" "$TARGET_DIR/.gitignore"; then
    cat >> "$TARGET_DIR/.gitignore" << 'GITIGNORE_SECTION'

# === AAD Template ===
.aad/worktrees/
.aad/worktrees-aad/
.aad/retrospectives/*.md
!.aad/retrospectives/.gitkeep
GITIGNORE_SECTION
    echo "  ✅ .gitignore にAADエントリを追記しました"
  else
    echo "  ℹ️  .gitignore には既にAADエントリがあります"
  fi
else
  cp "$TEMPLATE_ROOT/.gitignore" "$TARGET_DIR/"
  echo "  ✅ .gitignore を作成しました"
fi

# 3. .claude/
if [ "$CLAUDE_DIR_EXISTS" = true ]; then
  cp -r "$TARGET_DIR/.claude" "$BACKUP_DIR/"
  mkdir -p "$TARGET_DIR/.claude/commands"
  mkdir -p "$TARGET_DIR/.claude/scripts"
  cp -r "$TEMPLATE_ROOT/.claude/commands/aad" "$TARGET_DIR/.claude/commands/"
  cp "$TEMPLATE_ROOT/.claude/scripts/"* "$TARGET_DIR/.claude/scripts/" 2>/dev/null || true
  chmod +x "$TARGET_DIR/.claude/scripts/"*.sh 2>/dev/null || true
  echo "  ✅ スクリプトに実行権限を付与しました"
  echo "  ✅ .claude/ にaadコマンドとスクリプトをマージしました"
else
  cp -r "$TEMPLATE_ROOT/.claude" "$TARGET_DIR/"
  chmod +x "$TARGET_DIR/.claude/scripts/"*.sh 2>/dev/null || true
  echo "  ✅ スクリプトに実行権限を付与しました"
  echo "  ✅ .claude/ を作成しました"
fi

# .claude/settings.json を作成（存在しない場合のみ）
if [ ! -f "$TARGET_DIR/.claude/settings.json" ]; then
  cat > "$TARGET_DIR/.claude/settings.json" << 'EOF'
{
  "statusLine": {
    "type": "command",
    "command": "./.claude/scripts/context-bar.sh"
  }
}
EOF
  echo "  ✅ .claude/settings.json を作成しました"
fi

# 4. ディレクトリ作成
mkdir -p "$TARGET_DIR/.aad/templates" \
         "$TARGET_DIR/.aad/specs" \
         "$TARGET_DIR/.aad/tasks" \
         "$TARGET_DIR/.aad/retrospectives" \
         "$TARGET_DIR/.aad/worktrees"

# 5. テンプレートファイル
[ ! -f "$TARGET_DIR/.aad/templates/SPEC-TEMPLATE.md" ] && \
  cp "$TEMPLATE_ROOT/.aad/templates/SPEC-TEMPLATE.md" "$TARGET_DIR/.aad/templates/"
[ ! -f "$TARGET_DIR/.aad/templates/TASK-TEMPLATE.md" ] && \
  cp "$TEMPLATE_ROOT/.aad/templates/TASK-TEMPLATE.md" "$TARGET_DIR/.aad/templates/"
[ ! -f "$TARGET_DIR/.aad/templates/RETRO-TEMPLATE.md" ] && \
  cp "$TEMPLATE_ROOT/.aad/templates/RETRO-TEMPLATE.md" "$TARGET_DIR/.aad/templates/"
[ ! -f "$TARGET_DIR/.aad/templates/TEMPLATE.md" ] && \
  cp "$TEMPLATE_ROOT/.aad/templates/TEMPLATE.md" "$TARGET_DIR/.aad/templates/"

# 6. HANDOFF.md
[ ! -f "$TARGET_DIR/HANDOFF.md" ] && cp "$TEMPLATE_ROOT/HANDOFF.md" "$TARGET_DIR/"

# 7. docs/（サブフォルダとして）
if [ "$DOCS_EXISTS" = true ]; then
  mkdir -p "$TARGET_DIR/.aad"
  cp -r "$TEMPLATE_ROOT/.aad/"* "$TARGET_DIR/.aad/"
  echo "  ✅ .aad/ にドキュメントを配置しました"
else
  cp -r "$TEMPLATE_ROOT/docs" "$TARGET_DIR/"
  echo "  ✅ docs/ を作成しました"
fi

# 8. aad/配下（worktrees）
mkdir -p "$TARGET_DIR/.aad/worktrees"  # worktreesは常に作成

if [ "$AAD_EXISTS" = true ]; then
  [ -d "$TEMPLATE_ROOT/.aad/worktrees" ] && \
    cp -r "$TEMPLATE_ROOT/.aad/worktrees/"* "$TARGET_DIR/.aad/worktrees/" 2>/dev/null || true
  echo "  ✅ .aad/worktrees/ を作成しました"
else
  echo "  ✅ .aad/worktrees/ を作成しました"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 導入完了！"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "次のステップ:"
echo "  1. cd $TARGET_DIR"
echo "  2. claude"
echo "  3. /aad:init"
echo ""
echo "バックアップ: $BACKUP_DIR"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⚠️  重要: 次のステップ"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "AADテンプレートファイルをコミットすることを推奨します："
echo ""
echo "  cd $TARGET_DIR"
echo "  git add ."
echo "  git commit -m \"chore: AADテンプレートを導入\""
echo ""
echo "※ コミット前にworktreeを作成すると、.gitignoreで"
echo "  コンフリクトが発生する可能性があります。"
echo ""
