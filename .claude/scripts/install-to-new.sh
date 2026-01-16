#!/bin/bash
# 新規プロジェクトへのAADテンプレート導入スクリプト

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEMPLATE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET_DIR="${1:-}"
PROJECT_NAME="${2:-}"

# === ヘルプ表示 ===
show_help() {
  echo "使用方法: $0 <target-directory> [project-name]"
  echo ""
  echo "引数:"
  echo "  target-directory  新規プロジェクトを作成するディレクトリパス"
  echo "  project-name      プロジェクト名（省略時はディレクトリ名を使用）"
  echo ""
  echo "例:"
  echo "  $0 ~/workspace/my-new-project"
  echo "  $0 ~/workspace/my-new-project \"My Awesome Project\""
  echo "  $0 .  # 現在のディレクトリに導入"
}

# 引数チェック
if [ -z "$TARGET_DIR" ]; then
  show_help
  exit 1
fi

if [ "$TARGET_DIR" = "--help" ] || [ "$TARGET_DIR" = "-h" ]; then
  show_help
  exit 0
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 AADテンプレート新規プロジェクト作成"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "対象ディレクトリ: $TARGET_DIR"
echo ""

# === ディレクトリ準備フェーズ ===
echo "📁 ディレクトリを準備中..."

# ディレクトリが存在しない場合は作成
if [ ! -d "$TARGET_DIR" ]; then
  mkdir -p "$TARGET_DIR"
  echo "  ✅ ディレクトリを作成しました: $TARGET_DIR"
else
  # ディレクトリが空でない場合は警告
  if [ "$(ls -A "$TARGET_DIR" 2>/dev/null)" ]; then
    echo "  ⚠️  ディレクトリが空ではありません"
    echo ""
    echo "既存ファイルがある場合は install-to-existing.sh を使用してください:"
    echo "  $SCRIPT_DIR/install-to-existing.sh $TARGET_DIR"
    echo ""
    echo "強制的に続行しますか？ (y/n)"
    read -r CONTINUE
    [ "$CONTINUE" != "y" ] && exit 0
  else
    echo "  ✅ 空のディレクトリを確認しました"
  fi
fi

# 絶対パスに変換
TARGET_DIR="$(cd "$TARGET_DIR" && pwd)"

# プロジェクト名をディレクトリ名から取得
if [ -z "$PROJECT_NAME" ]; then
  PROJECT_NAME="$(basename "$TARGET_DIR")"
fi

echo ""
echo "プロジェクト名: $PROJECT_NAME"
echo ""
echo "続行しますか？ (y/n)"
read -r CONTINUE
[ "$CONTINUE" != "y" ] && exit 0

# === ファイルコピーフェーズ ===
echo ""
echo "📦 テンプレートファイルをコピー中..."

# コピー除外リスト
EXCLUDE_ITEMS=(".git" "node_modules" ".aad-backup-*" ".aad/worktrees")

# rsyncが使えるかチェック
if command -v rsync &> /dev/null; then
  # rsyncでコピー（除外指定付き）
  EXCLUDE_ARGS=""
  for item in "${EXCLUDE_ITEMS[@]}"; do
    EXCLUDE_ARGS="$EXCLUDE_ARGS --exclude=$item"
  done
  rsync -a $EXCLUDE_ARGS "$TEMPLATE_ROOT/" "$TARGET_DIR/"
  echo "  ✅ rsyncでコピーしました"
else
  # cpでコピー（除外はコピー後に削除）
  cp -r "$TEMPLATE_ROOT/"* "$TARGET_DIR/"
  cp -r "$TEMPLATE_ROOT/".* "$TARGET_DIR/" 2>/dev/null || true

  # 除外アイテムを削除
  for item in "${EXCLUDE_ITEMS[@]}"; do
    rm -rf "${TARGET_DIR:?}/$item" 2>/dev/null || true
  done
  echo "  ✅ cpでコピーしました"
fi

# .aad/worktreesディレクトリを作成（空で）
mkdir -p "$TARGET_DIR/.aad/worktrees"
touch "$TARGET_DIR/.aad/worktrees/.gitkeep"
echo "  ✅ .aad/worktrees/ を作成しました"

# スクリプトに実行権限を付与
chmod +x "$TARGET_DIR/.claude/scripts/"*.sh 2>/dev/null || true
echo "  ✅ スクリプトに実行権限を付与しました"

# .aad/retrospectivesのファイルを削除（テンプレートは別ディレクトリ）
find "$TARGET_DIR/.aad/retrospectives" -name "*.md" -delete 2>/dev/null || true

# .aad/tasksを空に
rm -rf "$TARGET_DIR/.aad/tasks/"*
mkdir -p "$TARGET_DIR/.aad/tasks"
touch "$TARGET_DIR/.aad/tasks/.gitkeep"

# .aad/specsのファイルを削除（テンプレートは別ディレクトリ）
find "$TARGET_DIR/.aad/specs" -name "*.md" -delete 2>/dev/null || true

# .claude/settings.jsonが存在しない場合のみ作成
if [ ! -f "$TARGET_DIR/.claude/settings.json" ]; then
  mkdir -p "$TARGET_DIR/.claude"
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

echo ""
echo "コピー完了！ディレクトリ構造:"
echo ""
cd "$TARGET_DIR"
find . -maxdepth 2 -type d | grep -v ".git" | sort | head -20

# === Git初期化フェーズ ===
echo ""
echo "🔀 Git初期化..."

cd "$TARGET_DIR"

if [ -d ".git" ]; then
  echo "  ℹ️  既にGitリポジトリです"
else
  git init
  echo "  ✅ Gitリポジトリを初期化しました"
fi

# === デフォルトブランチ検出 ===
echo ""
echo "🌿 デフォルトブランチを検出..."

# Gitの設定からデフォルトブランチ名を取得
DEFAULT_BRANCH=$(git config --get init.defaultBranch 2>/dev/null || echo "main")
CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "")

if [ -z "$CURRENT_BRANCH" ]; then
  # まだブランチがない場合（初回コミット前）
  DETECTED_BRANCH="$DEFAULT_BRANCH"
else
  DETECTED_BRANCH="$CURRENT_BRANCH"
fi

echo "  デフォルトブランチ: $DETECTED_BRANCH"

# CLAUDE.mdのデフォルトブランチを更新
if [ -f "CLAUDE.md" ]; then
  if [ "$DETECTED_BRANCH" != "main" ]; then
    sed -i '' "s/| デフォルトブランチ | \`main\` |/| デフォルトブランチ | \`$DETECTED_BRANCH\` |/g" CLAUDE.md 2>/dev/null || \
    sed -i "s/| デフォルトブランチ | \`main\` |/| デフォルトブランチ | \`$DETECTED_BRANCH\` |/g" CLAUDE.md 2>/dev/null || true
    echo "  ✅ CLAUDE.mdを更新しました"
  fi
fi

# === 初回コミットフェーズ ===
echo ""
echo "📝 初回コミットを作成しますか？ (y/n)"
read -r DO_COMMIT

if [ "$DO_COMMIT" = "y" ]; then
  git add .
  git commit -m "$(cat <<EOF
chore: AADテンプレートで初期化

自律型AI駆動開発テンプレートを導入:
- 6フェーズワークフロー (SPEC/TASKS/TDD/REVIEW/RETRO/MERGE)
- 12個のスラッシュコマンド (/aad:*)
- 品質ゲート (カバレッジ80%以上)
- コンテキスト管理 (70%ルール)
EOF
)"
  echo "  ✅ 初回コミットを作成しました"
else
  echo "  ℹ️  コミットをスキップしました"
fi

# === 完了メッセージ ===
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 新規プロジェクト作成完了！"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "次のステップ:"
echo ""
echo "  1. cd $TARGET_DIR"
echo "  2. claude"
echo "  3. /aad:init"
echo ""
echo "初期化ウィザードでプロジェクト情報を設定してください。"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📚 クイックリファレンス:"
echo ""
echo "  /aad:tasks SPEC-001    # SPECからタスク分割"
echo "  /aad:worktree TASK-ID  # worktree作成"
echo "  /aad:status            # 進捗確認"
echo "  /aad:context           # コンテキスト確認"
echo "  /aad:handoff           # 引き継ぎ文書作成"
echo ""
echo "詳細: $TARGET_DIR/.aad/COMMANDS.md"
