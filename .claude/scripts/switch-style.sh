#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
COMMANDS_DIR="$SCRIPT_DIR/../commands/aad"
CONTEXT_BAR="$SCRIPT_DIR/context-bar.sh"
CLAUDE_MD="$SCRIPT_DIR/../../CLAUDE.md"
STATE_FILE="$SCRIPT_DIR/../styles/.current-style"
BACKUP_DIR="$SCRIPT_DIR/../styles/backups"
MAX_BACKUPS=3

# セッションID生成
generate_session_id() {
  if command -v xxd &>/dev/null; then
    head -c 4 /dev/urandom | xxd -p
  else
    openssl rand -hex 4
  fi
}

# 現在のスタイル取得
get_current() {
  if [[ -f "$STATE_FILE" ]]; then
    cat "$STATE_FILE"
  else
    echo "standard"
  fi
}

# ディレクトリ初期化
init_dirs() {
  mkdir -p "$BACKUP_DIR"
  mkdir -p "$(dirname "$STATE_FILE")"
}

# 古いバックアップを削除
cleanup_old_backups() {
  local count
  count=$(ls -1d "$BACKUP_DIR"/*/ 2>/dev/null | wc -l | tr -d ' ')
  if (( count > MAX_BACKUPS )); then
    local to_remove=$((count - MAX_BACKUPS))
    ls -1d "$BACKUP_DIR"/*/ 2>/dev/null | head -n "$to_remove" | while read -r dir; do
      rm -rf "$dir"
      echo "Deleted old backup: $(basename "$dir")"
    done
  fi
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
  cp "$COMMANDS_DIR"/*.md "$backup_path/commands/aad/" 2>/dev/null || true
  cp "$CONTEXT_BAR" "$backup_path/" 2>/dev/null || true
  cp "$CLAUDE_MD" "$backup_path/" 2>/dev/null || true
  echo "Backup created: $backup_path"
  cleanup_old_backups
}

# バックアップ一覧表示
list_backups() {
  if [[ ! -d "$BACKUP_DIR" ]] || [[ -z "$(ls -A "$BACKUP_DIR" 2>/dev/null)" ]]; then
    echo "No backups available"
    return 1
  fi

  echo "=== Available Backups ==="
  local i=1
  for backup_path in "$BACKUP_DIR"/*/; do
    [[ ! -d "$backup_path" ]] && continue
    local backup
    backup=$(basename "$backup_path")
    local timestamp="${backup%_*}"
    local style="${backup##*_}"
    printf "%d. %s [%s]\n" "$i" "$backup" "$style"
    i=$((i + 1))
  done
}

# バックアップから復元
restore_backup() {
  local target="${1:-}"

  if [[ -z "$target" ]]; then
    target=$(ls -1r "$BACKUP_DIR" 2>/dev/null | head -1)
    if [[ -z "$target" ]]; then
      echo "Error: No backups available"
      return 1
    fi
    echo "Using latest backup: $target"
  fi

  local backup_path="$BACKUP_DIR/$target"

  if [[ ! -d "$backup_path" ]]; then
    echo "Error: Backup not found: $target"
    list_backups
    return 1
  fi

  backup

  cp "$backup_path/commands/aad/"*.md "$COMMANDS_DIR/" 2>/dev/null || true
  cp "$backup_path/context-bar.sh" "$CONTEXT_BAR" 2>/dev/null || true
  cp "$backup_path/CLAUDE.md" "$CLAUDE_MD" 2>/dev/null || true

  local style="${target##*_}"
  echo "$style" > "$STATE_FILE"

  echo "Restored from backup: $target"
  echo "Current style: $style"
}

# 単一ファイルの変換処理
convert_single_file() {
  local file="$1"
  local from_style="$2"
  local session_id="$3"
  local verbose="$4"

  [[ ! -f "$file" ]] && return 0
  [[ ! -w "$file" ]] && { echo "Warning: $file is not writable"; return 1; }

  local tmp_file
  tmp_file=$(mktemp)
  trap 'rm -f "$tmp_file"' RETURN
  cp "$file" "$tmp_file"

  local total_matches=0

  # Standard -> Sage tokens
  if [[ "$from_style" == "standard" ]]; then
    # Context bar tokens (with emoji)
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "🟡 通知：注意" "🟡 告：中程度")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "🟠 通知：警告" "🟠 告：警告レベル")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "🔴 通知：危機的" "🔴 告：危機的")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "⛔ 通知：限界" "⛔ 告：限界")))
    # Message prefix tokens
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "完了：" "成功しました：")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "結果：" "解：")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "通知：" "告：")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "エラー：" "否：")))
  else
    # Sage -> Standard tokens (reverse)
    # Context bar tokens
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "🟡 告：中程度" "🟡 通知：注意")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "🟠 告：警告レベル" "🟠 通知：警告")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "🔴 告：危機的" "🔴 通知：危機的")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "⛔ 告：限界" "⛔ 通知：限界")))
    # Message prefix tokens (longer first to avoid partial matches)
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "成功しました：" "完了：")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "解：" "結果：")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "告：" "通知：")))
    total_matches=$((total_matches + $(do_replace "$tmp_file" "$session_id" "$verbose" "否：" "エラー：")))
  fi

  if [[ "$total_matches" -gt 0 ]]; then
    mv "$tmp_file" "$file"
  else
    rm -f "$tmp_file"
  fi

  echo "$total_matches"
}

# 単一置換処理（マーカー方式）
do_replace() {
  local file="$1"
  local session_id="$2"
  local verbose="$3"
  local from_val="$4"
  local to_val="$5"

  local cnt
  cnt=$(grep -oF "$from_val" "$file" 2>/dev/null | wc -l | tr -d ' ') || true
  cnt=${cnt:-0}

  if [[ "$cnt" -gt 0 ]]; then
    # Step 1: Insert markers
    sed -i '' "s|${from_val}|<<${session_id}>>${from_val}<<${session_id}>>|g" "$file"
    # Step 2: Replace with markers
    sed -i '' "s|<<${session_id}>>${from_val}<<${session_id}>>|${to_val}|g" "$file"

    # Verbose output to stderr (not to interfere with return value)
    [[ "$verbose" == "true" ]] && echo "  \"$from_val\" -> \"$to_val\" ($cnt)" >&2
  fi

  echo "$cnt"
}

# ドライラン表示
show_dry_run() {
  local from_style="$1"
  local to_style="$2"

  echo "=== Dry-run: Convert to $to_style ==="
  echo "Current style: $from_style"
  echo ""
  echo "Files to convert:"

  local files=()
  for f in "$COMMANDS_DIR"/*.md; do [[ -f "$f" ]] && files+=("$f"); done
  [[ -f "$CONTEXT_BAR" ]] && files+=("$CONTEXT_BAR")
  [[ -f "$CLAUDE_MD" ]] && files+=("$CLAUDE_MD")

  local total_count=0
  local total_files=0

  for file in "${files[@]}"; do
    local file_count=0

    if [[ "$from_style" == "standard" ]]; then
      file_count=$((file_count + $(grep -oF "🟡 通知：注意" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "🟠 通知：警告" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "🔴 通知：危機的" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "⛔ 通知：限界" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "完了：" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "結果：" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "通知：" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "エラー：" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
    else
      file_count=$((file_count + $(grep -oF "🟡 告：中程度" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "🟠 告：警告レベル" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "🔴 告：危機的" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "⛔ 告：限界" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "成功しました：" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "解：" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "告：" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
      file_count=$((file_count + $(grep -oF "否：" "$file" 2>/dev/null | wc -l | tr -d ' ') )) || true
    fi

    if [[ "$file_count" -gt 0 ]]; then
      echo "  $file ($file_count matches)"
      total_count=$((total_count + file_count))
      total_files=$((total_files + 1))
    fi
  done

  echo ""
  echo "Total: $total_files files, $total_count matches"
  echo "(No changes made)"
}

# メイン変換処理
switch_style() {
  local to_style="$1"
  local dry_run="$2"
  local verbose="$3"

  local from_style
  from_style=$(get_current)

  if [[ "$from_style" == "$to_style" ]]; then
    echo "Already in $to_style style"
    return 0
  fi

  if [[ "$dry_run" == "true" ]]; then
    show_dry_run "$from_style" "$to_style"
    return 0
  fi

  local session_id
  session_id=$(generate_session_id)

  backup

  local files=()
  for f in "$COMMANDS_DIR"/*.md; do [[ -f "$f" ]] && files+=("$f"); done
  [[ -f "$CONTEXT_BAR" ]] && files+=("$CONTEXT_BAR")
  [[ -f "$CLAUDE_MD" ]] && files+=("$CLAUDE_MD")

  local total_files=0
  local total_count=0

  for file in "${files[@]}"; do
    [[ "$verbose" == "true" ]] && echo "Converting: $file"
    local cnt
    cnt=$(convert_single_file "$file" "$from_style" "$session_id" "$verbose")
    if [[ "$cnt" -gt 0 ]]; then
      total_files=$((total_files + 1))
      total_count=$((total_count + cnt))
    fi
  done

  echo "$to_style" > "$STATE_FILE"

  # Verify no markers left
  for file in "${files[@]}"; do
    if grep -qF "<<${session_id}>>" "$file" 2>/dev/null; then
      echo "Warning: Markers remain in $file"
    fi
  done

  echo ""
  echo "Converted to $to_style style"
  echo "  Files: $total_files"
  echo "  Replacements: $total_count"
}

# メイン処理
case "${1:-}" in
  standard|sage)
    verbose=false
    dry_run=false
    for arg in "${@:2}"; do
      [[ "$arg" == "--verbose" ]] && verbose=true
      [[ "$arg" == "--dry-run" ]] && dry_run=true
    done
    switch_style "$1" "$dry_run" "$verbose"
    ;;
  --dry-run)
    [[ -z "${2:-}" ]] && { echo "Usage: $0 --dry-run {standard|sage}"; exit 1; }
    switch_style "$2" "true" "false"
    ;;
  --current)
    echo "Current style: $(get_current)"
    ;;
  --list)
    echo "Available: standard, sage"
    ;;
  --list-backups)
    list_backups
    ;;
  --restore)
    restore_backup "${2:-}"
    ;;
  --cleanup)
    cleanup_old_backups
    echo "Cleanup complete"
    ;;
  *)
    echo "Usage: $0 {standard|sage} [--dry-run] [--verbose]"
    echo "       $0 {--current|--list|--list-backups|--restore [name]|--cleanup}"
    echo "       $0 --dry-run {standard|sage}"
    ;;
esac
