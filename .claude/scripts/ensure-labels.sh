#!/bin/bash
# ensure-labels.sh
# GitHub Issueラベルの存在確認・作成スクリプト

set -e

# 色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 必要なラベルの定義（ラベル名:色:説明）
declare -A LABELS
LABELS=(
    ["priority:must"]="ff0000:Must Have - 最高優先度"
    ["priority:should"]="ff9900:Should Have - 高優先度"
    ["priority:could"]="ffcc00:Could Have - 中優先度"
    ["priority:wont"]="cccccc:Won't Have - 対象外"
    ["size:S"]="c2e0c6:Small - 1-4時間"
    ["size:M"]="fef2c0:Medium - 4-8時間"
    ["size:L"]="f9d0c4:Large - 8時間以上"
    ["status:todo"]="ededed:To Do - 未着手"
    ["status:in-progress"]="0052cc:In Progress - 進行中"
    ["status:review"]="5319e7:Review - レビュー中"
    ["status:done"]="0e8a16:Done - 完了"
    ["status:blocked"]="d73a4a:Blocked - ブロック中"
    ["type:feature"]="0075ca:Feature - 新機能"
    ["type:bug"]="d73a4a:Bug - バグ修正"
    ["type:refactor"]="fbca04:Refactor - リファクタリング"
    ["type:docs"]="0075ca:Documentation - ドキュメント"
    ["type:test"]="d4c5f9:Test - テスト"
    ["type:chore"]="fef2c0:Chore - 雑務"
)

# SPEC-XXX形式のラベル用のプレフィックス
SPEC_PREFIX="spec:"

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}GitHub Issueラベル確認・作成${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# GitHub CLIの確認
if ! command -v gh &> /dev/null; then
    echo -e "${RED}エラー: GitHub CLI (gh) がインストールされていません${NC}"
    echo "インストール方法: https://cli.github.com/"
    exit 1
fi

# 認証確認
if ! gh auth status &> /dev/null; then
    echo -e "${RED}エラー: GitHub CLIが認証されていません${NC}"
    echo "認証方法: gh auth login"
    exit 1
fi

# 既存ラベルの取得
echo -e "${YELLOW}既存ラベルを取得中...${NC}"
existing_labels=$(gh label list --json name --jq '.[].name')

# 作成カウンター
created_count=0
skipped_count=0

# 各ラベルの確認・作成
for label in "${!LABELS[@]}"; do
    IFS=':' read -r color description <<< "${LABELS[$label]}"

    if echo "$existing_labels" | grep -q "^${label}$"; then
        echo -e "  ✅ ${label} - 既に存在"
        ((skipped_count++))
    else
        echo -e "  🆕 ${label} - 作成中..."
        gh label create "$label" \
            --color "$color" \
            --description "$description" \
            --force
        ((created_count++))
    fi
done

# SPEC-XXXラベルの確認（オプション引数）
if [ -n "$1" ]; then
    spec_label="${SPEC_PREFIX}$1"
    if echo "$existing_labels" | grep -q "^${spec_label}$"; then
        echo -e "  ✅ ${spec_label} - 既に存在"
    else
        echo -e "  🆕 ${spec_label} - 作成中..."
        gh label create "$spec_label" \
            --color "1d76db" \
            --description "SPEC $1 related issues" \
            --force
        ((created_count++))
    fi
fi

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}完了${NC}"
echo -e "  作成: ${created_count}個"
echo -e "  スキップ: ${skipped_count}個"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
