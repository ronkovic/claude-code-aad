#!/usr/bin/env bash
# start-worker.sh - Start a Docker worker for a specific task
#
# Usage:
#   start-worker.sh <task-id> <worktree-path> [--attach]
#
# Examples:
#   start-worker.sh SPEC-001-T01 /Users/user/workspace/myproject-T01
#   start-worker.sh SPEC-001-T01 /Users/user/workspace/myproject-T01 --attach

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/worker-common.sh"

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Usage
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

usage() {
    echo "Usage: $0 <task-id> <worktree-path> [--attach]"
    echo ""
    echo "Arguments:"
    echo "  task-id       Task ID (e.g., SPEC-001-T01)"
    echo "  worktree-path Absolute path to the worktree directory"
    echo "  --attach      Attach to container after starting (optional)"
    echo ""
    echo "Examples:"
    echo "  $0 SPEC-001-T01 /Users/user/workspace/myproject-T01"
    echo "  $0 SPEC-001-T01 /Users/user/workspace/myproject-T01 --attach"
    echo ""
    echo "Environment variables:"
    echo "  CLAUDE_CODE_OAUTH_TOKEN  OAuth token for Claude Code (required)"
    echo "  ANTHROPIC_API_KEY        Alternative API key authentication"
    echo "  GITHUB_TOKEN             GitHub Personal Access Token (optional)"
    echo "  GIT_USER_NAME            Git user name for commits"
    echo "  GIT_USER_EMAIL           Git user email for commits"
    exit 1
}

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Parse Arguments
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[[ $# -lt 2 ]] && usage

TASK_ID="$1"
WORKTREE_PATH="$2"
ATTACH_FLAG="${3:-}"

# Validate inputs
validate_task_id "$TASK_ID" || exit 1

if [[ ! -d "$WORKTREE_PATH" ]]; then
    echo -e "${RED}ERROR${NC}: Worktree path does not exist: $WORKTREE_PATH"
    exit 1
fi

# Get absolute path
WORKTREE_PATH=$(get_absolute_path "$WORKTREE_PATH")

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Preparation
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# Get container name
CONTAINER_NAME=$(get_container_name "$TASK_ID")
TASK_NUMBER=$(get_task_number "$TASK_ID")

# Check if already running
if container_running "$CONTAINER_NAME"; then
    echo -e "${YELLOW}WARNING${NC}: Container $CONTAINER_NAME is already running"
    echo ""
    if [[ "$ATTACH_FLAG" == "--attach" ]]; then
        echo -e "${BLUE}Attaching to existing container...${NC}"
        docker exec -it "$CONTAINER_NAME" bash
    else
        echo "To attach to this container:"
        echo "  docker exec -it $CONTAINER_NAME bash"
    fi
    exit 0
fi

# Remove stopped container if exists
if container_exists "$CONTAINER_NAME"; then
    echo -e "${BLUE}Removing stopped container: $CONTAINER_NAME${NC}"
    docker rm "$CONTAINER_NAME" >/dev/null
fi

# Load environment variables
load_env "$SCRIPT_DIR"

# Ensure network exists
ensure_network

# Verify authentication
if [[ -z "${CLAUDE_CODE_OAUTH_TOKEN:-}" ]] && [[ -z "${ANTHROPIC_API_KEY:-}" ]]; then
    echo -e "${RED}ERROR${NC}: No authentication configured"
    echo ""
    echo "Please set one of the following in container/.env:"
    echo "  CLAUDE_CODE_OAUTH_TOKEN  (Max Plan users - recommended)"
    echo "  ANTHROPIC_API_KEY        (API users)"
    echo ""
    echo "Or export them as environment variables:"
    echo "  export CLAUDE_CODE_OAUTH_TOKEN=\"sk-ant-oat01-...\""
    exit 1
fi

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Start Container
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Starting Docker Worker${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "  Task ID:    ${GREEN}${TASK_ID}${NC}"
echo -e "  Container:  ${CYAN}${CONTAINER_NAME}${NC}"
echo -e "  Worktree:   ${WORKTREE_PATH}"
echo -e "  Image:      ${IMAGE_NAME}"
echo -e "  Network:    ${NETWORK_NAME}"
echo ""

# Build docker run command
# Key: Same-path mounting for Git worktree compatibility
docker run -d \
    --name "$CONTAINER_NAME" \
    --network "$NETWORK_NAME" \
    -v "${WORKTREE_PATH}:${WORKTREE_PATH}" \
    -v "${SCRIPT_DIR}/../../.claude:/home/claude/.claude" \
    -w "${WORKTREE_PATH}" \
    -e "ROLE=worker" \
    -e "TASK_ID=${TASK_ID}" \
    -e "WORKER_ID=${TASK_NUMBER}" \
    -e "HOST_PROJECT_PATH=${WORKTREE_PATH}" \
    -e "CLAUDE_CODE_OAUTH_TOKEN=${CLAUDE_CODE_OAUTH_TOKEN:-}" \
    -e "ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY:-}" \
    -e "GITHUB_TOKEN=${GITHUB_TOKEN:-}" \
    -e "GIT_USER_NAME=${GIT_USER_NAME:-Claude AI}" \
    -e "GIT_USER_EMAIL=${GIT_USER_EMAIL:-claude@example.com}" \
    --stdin-open \
    --tty \
    "$IMAGE_NAME" >/dev/null

echo -e "${GREEN}✅ Worker started successfully!${NC}"
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Next Steps${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "1. Attach to worker:"
echo -e "   ${CYAN}docker exec -it $CONTAINER_NAME bash${NC}"
echo ""
echo "2. Inside the container, run Claude:"
echo -e "   ${CYAN}claude --dangerously-skip-permissions${NC}"
echo ""
echo "3. Or provide a specific prompt:"
echo -e "   ${CYAN}claude --dangerously-skip-permissions -p 'docs/aad/tasks/${TASK_ID}に従って実装'${NC}"
echo ""
echo "4. When done, stop the worker:"
echo -e "   ${CYAN}./stop-worker.sh $TASK_ID${NC}"
echo ""

# Attach if requested
if [[ "$ATTACH_FLAG" == "--attach" ]]; then
    echo -e "${BLUE}Attaching to container...${NC}"
    echo ""
    docker exec -it "$CONTAINER_NAME" bash
fi
