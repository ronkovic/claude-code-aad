#!/usr/bin/env bash
# Container Setup Script
# Runs on container start

set -euo pipefail

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Autonomous AI-Driven Development Container${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Detect role
ROLE=${ROLE:-standalone}

if [ "$ROLE" = "orchestrator" ]; then
    echo -e "${GREEN}Role: Orchestrator (調整役)${NC}"
    echo "  - タスク分割・進捗監視・統合を担当"
elif [ "$ROLE" = "worker" ]; then
    WORKER_ID=${WORKER_ID:-unknown}
    echo -e "${GREEN}Role: Worker ${WORKER_ID}${NC}"
    echo "  - タスク実装を担当"
else
    echo -e "${GREEN}Role: Standalone${NC}"
    echo "  - 単独実行モード"
fi

echo ""
echo "All files are pre-installed. Ready to use!"
echo ""

# Check authentication
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Authentication${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [ -n "${CLAUDE_CODE_OAUTH_TOKEN:-}" ]; then
    echo "  ✅ CLAUDE_CODE_OAUTH_TOKEN is set (Max Plan)"
elif [ -n "${ANTHROPIC_API_KEY:-}" ]; then
    echo "  ✅ ANTHROPIC_API_KEY is set (API)"
else
    echo -e "  ${YELLOW}⚠️  No authentication configured${NC}"
    echo "  You will need to authenticate manually:"
    echo "    - Run 'claude' and follow OAuth flow"
    echo "    - Or set CLAUDE_CODE_OAUTH_TOKEN / ANTHROPIC_API_KEY"
fi

echo ""

# Check Gemini (if installed)
if command -v gemini &> /dev/null; then
    echo "  ℹ️  Gemini CLI: Installed (optional)"
    if [ -n "${GEMINI_API_KEY:-}" ]; then
        echo "  ✅ GEMINI_API_KEY is set"
    else
        echo "  ⚠️  GEMINI_API_KEY not set (manual auth required)"
    fi
else
    echo "  ℹ️  Gemini CLI: Not installed (optional)"
fi

echo ""

# Configure Git
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Git Configuration${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [ -n "${GIT_USER_NAME:-}" ]; then
    git config --global user.name "$GIT_USER_NAME"
    echo "  ✅ Git user.name: $GIT_USER_NAME"
else
    echo "  ⚠️  GIT_USER_NAME not set (will use default)"
fi

if [ -n "${GIT_USER_EMAIL:-}" ]; then
    git config --global user.email "$GIT_USER_EMAIL"
    echo "  ✅ Git user.email: $GIT_USER_EMAIL"
else
    echo "  ⚠️  GIT_USER_EMAIL not set (will use default)"
fi

# Configure GitHub CLI if token is available
if [ -n "${GITHUB_TOKEN:-}" ]; then
    echo "$GITHUB_TOKEN" | gh auth login --with-token 2>/dev/null && \
        echo "  ✅ GitHub CLI authenticated" || \
        echo "  ⚠️  GitHub CLI auth failed (manual auth required)"
else
    echo "  ℹ️  GITHUB_TOKEN not set (manual gh auth required)"
fi

echo ""

# Display next steps
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Next Steps${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [ "$ROLE" = "orchestrator" ]; then
    echo "  1. Run 'claude' to start orchestrator"
    echo "  2. Use '/aad:orchestrate SPEC-001' for full automation"
    echo "  3. Or use '/aad:tasks SPEC-001' for manual workflow"
elif [ "$ROLE" = "worker" ]; then
    echo "  1. Wait for orchestrator to assign task"
    echo "  2. Run 'claude --dangerously-skip-permissions' to start"
    echo "  3. Worker will complete task autonomously"
else
    echo "  1. Run 'claude' to start Claude Code"
    echo "  2. Authenticate if needed"
    echo "  3. Run '/aad:init' to setup project"
    echo "  4. Create SPEC and start development!"
fi

echo ""
echo -e "${GREEN}Available commands:${NC}"
echo "  /aad:init       - Initialize project"
echo "  /aad:tasks      - Split SPEC into tasks"
echo "  /aad:worktree   - Create worktree for task"
echo "  /aad:status     - Check overall progress"
echo "  /aad:orchestrate - Full automation"
echo "  /aad:context    - Check context usage"
echo "  /aad:handoff    - Create handoff document"
echo ""

# 作業ディレクトリに移動（環境変数が設定されている場合）
if [ -n "${HOST_PROJECT_PATH:-}" ] && [ -d "$HOST_PROJECT_PATH" ]; then
    cd "$HOST_PROJECT_PATH" || echo "⚠️  Warning: Could not change to $HOST_PROJECT_PATH"
    echo -e "${GREEN}  📁 Working directory: $HOST_PROJECT_PATH${NC}"
elif [ -d "/home/claude/workspace" ]; then
    cd /home/claude/workspace || true
    echo -e "${GREEN}  📁 Working directory: /home/claude/workspace${NC}"
else
    echo -e "${YELLOW}  📁 Working directory: $(pwd)${NC}"
fi

# Start interactive shell
exec /bin/bash
