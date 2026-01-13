#!/bin/bash
# デフォルトブランチを検出するスクリプト

# 方法1: リモートのHEADから取得
if git symbolic-ref refs/remotes/origin/HEAD &>/dev/null; then
  git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@'
  exit 0
fi

# 方法2: 現在のブランチを取得（初回コミット前や初期化直後）
CURRENT=$(git branch --show-current 2>/dev/null)
if [ -n "$CURRENT" ]; then
  echo "$CURRENT"
  exit 0
fi

# フォールバック: デフォルトで main
echo "main"
