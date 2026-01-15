#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
COMMANDS_DIR="$SCRIPT_DIR/../commands/aad"
CONTEXT_BAR="$SCRIPT_DIR/context-bar.sh"
STATE_FILE="$SCRIPT_DIR/../styles/.current-style"
BACKUP_DIR="$SCRIPT_DIR/../styles/backups"

# スタイル定義
SAGE_KEYWORDS=(
  "成功しました：:完了："
  "解：:結果："
  "告：:通知："
  "否：:エラー："
)
SAGE_CONTEXTBAR=(
  "告：限界:通知：限界"
  "告：危機的:通知：危機的"
  "告：警告レベル:通知：警告"
  "告：中程度:通知：注意"
)

# 現在のスタイル取得
get_current() {
  [[ -f "$STATE_FILE" ]] && cat "$STATE_FILE" || echo "sage"
}

# ディレクトリ初期化
init_dirs() {
  mkdir -p "$BACKUP_DIR"
  mkdir -p "$(dirname "$STATE_FILE")"
}

# バックアップ作成
backup() {
  init_dirs
  local timestamp
  timestamp=$(date +%Y%m%d_%H%M%S)
  local current
  current=$(get_current)
  local backup_path="$BACKUP_DIR/${timestamp}_${current}"
  mkdir -p "$backup_path/commands/aad"
  cp "$COMMANDS_DIR"/*.md "$backup_path/commands/aad/"
  cp "$CONTEXT_BAR" "$backup_path/"
  echo "バックアップ作成: $backup_path"
}

# sage → standard 変換
to_standard() {
  for pair in "${SAGE_KEYWORDS[@]}"; do
    local from="${pair%%:*}"
    local to="${pair#*:}"
    sed -i '' "s|$from|$to|g" "$COMMANDS_DIR"/*.md
  done
  for pair in "${SAGE_CONTEXTBAR[@]}"; do
    local from="${pair%%:*}"
    local to="${pair#*:}"
    sed -i '' "s|$from|$to|g" "$CONTEXT_BAR"
  done
  echo "standard" > "$STATE_FILE"
}

# standard → sage 変換（逆方向）
to_sage() {
  for pair in "${SAGE_KEYWORDS[@]}"; do
    local from="${pair#*:}"
    local to="${pair%%:*}"
    sed -i '' "s|$from|$to|g" "$COMMANDS_DIR"/*.md
  done
  for pair in "${SAGE_CONTEXTBAR[@]}"; do
    local from="${pair#*:}"
    local to="${pair%%:*}"
    sed -i '' "s|$from|$to|g" "$CONTEXT_BAR"
  done
  echo "sage" > "$STATE_FILE"
}

# ドライラン（プレビュー）
dry_run() {
  local target=$1
  echo "=== ドライラン: $target への変換 ==="
  echo "現在のスタイル: $(get_current)"
  echo "変換対象ファイル:"
  ls -1 "$COMMANDS_DIR"/*.md
  echo "$CONTEXT_BAR"
  echo "※ 実際の変換は行われません"
}

# バックアップ一覧表示
list_backups() {
  if [[ ! -d "$BACKUP_DIR" ]] || [[ -z "$(ls -A "$BACKUP_DIR" 2>/dev/null)" ]]; then
    echo "バックアップがありません"
    return 1
  fi

  echo "=== 利用可能なバックアップ ==="
  local i=1
  while IFS= read -r backup; do
    local timestamp="${backup%_*}"
    local style="${backup##*_}"
    local formatted
    formatted=$(echo "$timestamp" | sed 's/\([0-9]\{4\}\)\([0-9]\{2\}\)\([0-9]\{2\}\)_\([0-9]\{2\}\)\([0-9]\{2\}\)\([0-9]\{2\}\)/\1-\2-\3 \4:\5:\6/')
    printf "%d. %s (%s) [%s]\n" "$i" "$backup" "$formatted" "$style"
    ((i++))
  done < <(ls -1r "$BACKUP_DIR" 2>/dev/null)
}

# バックアップから復元
restore_backup() {
  local target="${1:-}"

  # 引数なしの場合は最新のバックアップを使用
  if [[ -z "$target" ]]; then
    target=$(ls -1r "$BACKUP_DIR" 2>/dev/null | head -1)
    if [[ -z "$target" ]]; then
      echo "エラー：バックアップがありません"
      return 1
    fi
    echo "最新のバックアップを使用: $target"
  fi

  local backup_path="$BACKUP_DIR/$target"

  # バックアップ存在確認
  if [[ ! -d "$backup_path" ]]; then
    echo "エラー：バックアップが見つかりません: $target"
    echo ""
    list_backups
    return 1
  fi

  # 復元前にバックアップ作成
  backup

  # ファイル復元
  cp "$backup_path/commands/aad/"*.md "$COMMANDS_DIR/"
  cp "$backup_path/context-bar.sh" "$CONTEXT_BAR"

  # スタイル状態を更新
  local style="${target##*_}"
  echo "$style" > "$STATE_FILE"

  echo "完了：バックアップから復元しました"
  echo "  復元元: $target"
  echo "  現在のスタイル: $style"
}

# メイン処理
case "${1:-}" in
  standard)
    [[ "$(get_current)" == "standard" ]] && { echo "既にstandardスタイルです"; exit 0; }
    backup && to_standard && echo "完了：標準スタイルに変換しました"
    ;;
  sage)
    [[ "$(get_current)" == "sage" ]] && { echo "既にsageスタイルです"; exit 0; }
    backup && to_sage && echo "成功しました：大賢者スタイルに変換しました"
    ;;
  --dry-run)
    [[ -z "${2:-}" ]] && { echo "Usage: $0 --dry-run {standard|sage}"; exit 1; }
    dry_run "$2"
    ;;
  --current) echo "現在のスタイル: $(get_current)" ;;
  --list)    echo "利用可能: sage, standard" ;;
  --list-backups)
    list_backups
    ;;
  --restore)
    restore_backup "${2:-}"
    ;;
  *)         echo "Usage: $0 {standard|sage|--current|--list|--list-backups|--restore [timestamp]|--dry-run <style>}" ;;
esac
