#!/usr/bin/env bash
# list-workers.sh - List all running and stopped aad workers
#
# Usage:
#   list-workers.sh [--running|--all]
#
# Examples:
#   list-workers.sh
#   list-workers.sh --running
#   list-workers.sh --all

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/worker-common.sh"

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Parse Arguments
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SHOW_ALL="false"

for arg in "$@"; do
    case "$arg" in
        --all) SHOW_ALL="true" ;;
        --running) SHOW_ALL="false" ;;
        -h|--help)
            echo "Usage: $0 [--running|--all]"
            echo ""
            echo "Options:"
            echo "  --running  Show only running workers (default)"
            echo "  --all      Show all workers (running + stopped)"
            echo ""
            echo "Examples:"
            echo "  $0"
            echo "  $0 --running"
            echo "  $0 --all"
            exit 0
            ;;
        *) echo -e "${RED}Unknown argument: $arg${NC}"; exit 1 ;;
    esac
done

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Main Display
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Docker Worker Status${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Running Workers
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

echo -e "${CYAN}🚀 Running Workers:${NC}"
echo ""

running=$(docker ps --format '{{.Names}}\t{{.Status}}' | grep "^${CONTAINER_PREFIX}-SPEC-" || true)

if [[ -z "$running" ]]; then
    echo "  (none)"
    echo ""
else
    echo "$running" | while IFS=$'\t' read -r name status; do
        task_id="${name#"${CONTAINER_PREFIX}"-}"
        workdir=$(docker inspect --format '{{.Config.WorkingDir}}' "$name" 2>/dev/null || echo "unknown")

        echo -e "  ${GREEN}●${NC} ${GREEN}${task_id}${NC}"
        echo -e "     Container: ${CYAN}$name${NC}"
        echo -e "     Status:    $status"
        echo -e "     Workdir:   $workdir"
        echo ""
        echo -e "     ${BLUE}Actions:${NC}"
        echo -e "       Attach:  ${CYAN}docker exec -it $name bash${NC}"
        echo -e "       Logs:    ${CYAN}docker logs -f $name${NC}"
        echo -e "       Stop:    ${CYAN}./stop-worker.sh $task_id${NC}"
        echo ""
    done
fi

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Stopped Workers (if --all)
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if [[ "$SHOW_ALL" == "true" ]]; then
    echo -e "${CYAN}⏸  Stopped Workers:${NC}"
    echo ""

    stopped=$(docker ps -a --format '{{.Names}}\t{{.Status}}' | grep "^${CONTAINER_PREFIX}-SPEC-" | grep -v "Up " || true)

    if [[ -z "$stopped" ]]; then
        echo "  (none)"
        echo ""
    else
        echo "$stopped" | while IFS=$'\t' read -r name status; do
            task_id="${name#"${CONTAINER_PREFIX}"-}"

            echo -e "  ${YELLOW}○${NC} ${YELLOW}${task_id}${NC}"
            echo -e "     Container: ${CYAN}$name${NC}"
            echo -e "     Status:    $status"
            echo ""
            echo -e "     ${BLUE}Actions:${NC}"
            echo -e "       Remove:  ${CYAN}docker rm $name${NC}"
            echo -e "       Or:      ${CYAN}./stop-worker.sh $task_id --force${NC}"
            echo ""
        done
    fi
fi

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Summary
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

running_count=$(docker ps --format '{{.Names}}' | grep -c "^${CONTAINER_PREFIX}-SPEC-" || echo "0")
total_count=$(docker ps -a --format '{{.Names}}' | grep -c "^${CONTAINER_PREFIX}-SPEC-" || echo "0")
stopped_count=$((total_count - running_count))

echo -e "${GREEN}Running:${NC} $running_count  |  ${YELLOW}Stopped:${NC} $stopped_count  |  ${CYAN}Total:${NC} $total_count"

echo ""
echo -e "${BLUE}Common Commands:${NC}"
echo -e "  Start worker:   ${CYAN}./start-worker.sh <task-id> <worktree-path>${NC}"
echo -e "  Stop worker:    ${CYAN}./stop-worker.sh <task-id>${NC}"
echo -e "  Stop all:       ${CYAN}./stop-worker.sh --all${NC}"
echo -e "  Show all:       ${CYAN}./list-workers.sh --all${NC}"
echo ""
