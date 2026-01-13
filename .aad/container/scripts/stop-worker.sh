#!/usr/bin/env bash
# stop-worker.sh - Stop and remove a Docker worker
#
# Usage:
#   stop-worker.sh <task-id> [--force]
#   stop-worker.sh --all [--force]
#
# Examples:
#   stop-worker.sh SPEC-001-T01
#   stop-worker.sh SPEC-001-T01 --force
#   stop-worker.sh --all
#   stop-worker.sh --all --force

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/worker-common.sh"

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Usage
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

usage() {
    echo "Usage: $0 <task-id> [--force]"
    echo "       $0 --all [--force]"
    echo ""
    echo "Arguments:"
    echo "  task-id    Task ID (e.g., SPEC-001-T01)"
    echo "  --all      Stop all aad-* workers"
    echo "  --force    Force stop without confirmation"
    echo ""
    echo "Examples:"
    echo "  $0 SPEC-001-T01"
    echo "  $0 SPEC-001-T01 --force"
    echo "  $0 --all"
    echo "  $0 --all --force"
    exit 1
}

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Functions
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

stop_container() {
    local container_name="$1"
    local force="$2"

    if ! container_exists "$container_name"; then
        echo -e "${YELLOW}Container not found: $container_name${NC}"
        return 0
    fi

    if container_running "$container_name"; then
        if [[ "$force" != "true" ]]; then
            echo -n "Stop running container $container_name? [y/N]: "
            read -r confirm
            if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
                echo "Skipped."
                return 0
            fi
        fi

        echo -e "${BLUE}Stopping container: $container_name${NC}"
        docker stop "$container_name" >/dev/null
    fi

    echo -e "${BLUE}Removing container: $container_name${NC}"
    docker rm "$container_name" >/dev/null
    echo -e "${GREEN}✅ Container removed: $container_name${NC}"
}

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Parse Arguments
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[[ $# -lt 1 ]] && usage

FORCE="false"
STOP_ALL="false"
TASK_ID=""

for arg in "$@"; do
    case "$arg" in
        --force) FORCE="true" ;;
        --all) STOP_ALL="true" ;;
        SPEC-*) TASK_ID="$arg" ;;
        -h|--help) usage ;;
        *) echo -e "${RED}Unknown argument: $arg${NC}"; usage ;;
    esac
done

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Main Logic
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if [[ "$STOP_ALL" == "true" ]]; then
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Stopping All Workers${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""

    # Find all aad-* containers
    containers=$(docker ps -a --format '{{.Names}}' | grep "^${CONTAINER_PREFIX}-SPEC-" || true)

    if [[ -z "$containers" ]]; then
        echo "No aad workers found."
        exit 0
    fi

    echo "Found workers:"
    echo "$containers" | while read -r container; do
        task_id="${container#${CONTAINER_PREFIX}-}"
        status=$(docker ps -a --format '{{.Names}}\t{{.Status}}' | grep "^$container\t" | cut -f2)
        echo "  - $task_id ($status)"
    done
    echo ""

    if [[ "$FORCE" != "true" ]]; then
        echo -n "Stop all these workers? [y/N]: "
        read -r confirm
        if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
            echo "Cancelled."
            exit 0
        fi
    fi

    echo ""
    for container in $containers; do
        stop_container "$container" "true"
    done

    echo ""
    echo -e "${GREEN}✅ All workers stopped.${NC}"
else
    # Stop single worker
    [[ -z "$TASK_ID" ]] && usage
    validate_task_id "$TASK_ID" || exit 1

    CONTAINER_NAME=$(get_container_name "$TASK_ID")

    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Stopping Worker: ${TASK_ID}${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""

    stop_container "$CONTAINER_NAME" "$FORCE"
fi
