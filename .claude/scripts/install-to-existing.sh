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

# 1. CLAUDE.md
if [ "$CLAUDE_MD_EXISTS" = true ]; then
  cp "$TARGET_DIR/CLAUDE.md" "$BACKUP_DIR/"
  echo ""
  echo "CLAUDE.md にAADセクションを追記しますか？ (y/n)"
  read -r APPEND_CLAUDE
  if [ "$APPEND_CLAUDE" = "y" ]; then
    cat >> "$TARGET_DIR/CLAUDE.md" << 'CLAUDE_SECTION'

---

## ⚙️ プロジェクト設定

| 設定 | 値 |
|------|-----|
| デフォルトブランチ | `main` |

**注**: デフォルトブランチは `/aad:init` で自動検出されます。

---

## 🎯 コンテキスト管理ルール（70%ルール）

| 使用率 | ステータス | アクション |
|--------|------------|------------|
| 0-50% | 🟢 快適 | 通常作業 |
| 50-70% | 🟡 注意 | 大きなタスクは新セッション推奨 |
| 70-85% | 🟠 警告 | `/aad:handoff` 実行推奨 |
| 85-95% | 🔴 危険 | 即座に `/aad:handoff` |
| 95%+ | ⛔ 限界 | 自動圧縮（制御不能） |

CLAUDE_SECTION
    echo "  ✅ CLAUDE.md にAADセクションを追記しました"
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
.aad/container/.env
.aad/container-aad/.env
.aad/retrospectives/*.md
!.aad/retrospectives/RETRO-TEMPLATE.md
!.aad/retrospectives/TEMPLATE.md
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
  echo "  ✅ .claude/ にaadコマンドとスクリプトをマージしました"
else
  cp -r "$TEMPLATE_ROOT/.claude" "$TARGET_DIR/"
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
mkdir -p "$TARGET_DIR/.aad/specs" \
         "$TARGET_DIR/.aad/tasks" \
         "$TARGET_DIR/.aad/retrospectives" \
         "$TARGET_DIR/.aad/worktrees"

# 5. テンプレートファイル
[ ! -f "$TARGET_DIR/.aad/specs/SPEC-TEMPLATE.md" ] && \
  cp "$TEMPLATE_ROOT/.aad/specs/SPEC-TEMPLATE.md" "$TARGET_DIR/.aad/specs/"
[ ! -f "$TARGET_DIR/.aad/retrospectives/RETRO-TEMPLATE.md" ] && \
  cp "$TEMPLATE_ROOT/.aad/retrospectives/RETRO-TEMPLATE.md" "$TARGET_DIR/.aad/retrospectives/"
[ ! -f "$TARGET_DIR/.aad/retrospectives/TEMPLATE.md" ] && \
  cp "$TEMPLATE_ROOT/.aad/retrospectives/TEMPLATE.md" "$TARGET_DIR/.aad/retrospectives/" 2>/dev/null || true
[ ! -f "$TARGET_DIR/.aad/tasks/TASK-TEMPLATE.md" ] && \
  cp "$TEMPLATE_ROOT/.aad/tasks/TASK-TEMPLATE.md" "$TARGET_DIR/.aad/tasks/" 2>/dev/null || true

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

# 8. aad/配下（container, worktrees）
mkdir -p "$TARGET_DIR/.aad/worktrees"  # worktreesは常に作成

if [ "$AAD_EXISTS" = true ]; then
  # 既存のaad/がある場合は別名で配置
  if [ -d "$TARGET_DIR/.aad/container" ]; then
    echo "  ⚠️  .aad/container/ は既に存在します（スキップ）"
    echo "     テンプレートは .aad/container-aad として配置する予定でしたが、"
    echo "     container-aad が既に存在する場合は手動で更新してください"
  elif [ -d "$TEMPLATE_ROOT/.aad/container" ]; then
    cp -r "$TEMPLATE_ROOT/.aad/container" "$TARGET_DIR/.aad/container-aad"
    echo "  ✅ .aad/container-aad としてDocker環境を配置しました"
  fi
  [ -d "$TEMPLATE_ROOT/.aad/worktrees" ] && \
    cp -r "$TEMPLATE_ROOT/.aad/worktrees/"* "$TARGET_DIR/.aad/worktrees/" 2>/dev/null || true
  echo "  ✅ .aad/worktrees/ を作成しました"
else
  if [ -d "$TARGET_DIR/.aad/container" ]; then
    echo "  ⚠️  .aad/container/ は既に存在します"
    echo "     最新のテンプレートに更新する場合は以下を実行:"
    echo "     rm -rf $TARGET_DIR/.aad/container"
    echo "     cp -r $TEMPLATE_ROOT/.aad/container $TARGET_DIR/aad/"
  else
    if [ -d "$TEMPLATE_ROOT/.aad/container" ]; then
      cp -r "$TEMPLATE_ROOT/.aad/container" "$TARGET_DIR/aad/"
      echo "  ✅ .aad/container/ を作成しました"
    else
      echo "  ⚠️  .aad/container/ はテンプレートに存在しません（スキップ）"
    fi
  fi
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
