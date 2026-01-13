#!/usr/bin/env bash
# worker-common.sh - Shared functions and configuration for worker management
#
# This file should be sourced by other worker management scripts.

set -euo pipefail

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Constants
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

export CONTAINER_PREFIX="aad"
export IMAGE_NAME="autonomous-dev:latest"
export NETWORK_NAME="autonomous-net"

# Colors
export GREEN='\033[0;32m'
export BLUE='\033[0;34m'
export YELLOW='\033[1;33m'
export RED='\033[0;31m'
export CYAN='\033[0;36m'
export NC='\033[0m'

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Functions
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# Get container name from task ID
# Input: SPEC-001-T01
# Output: aad-SPEC-001-T01
get_container_name() {
    local task_id="$1"
    echo "${CONTAINER_PREFIX}-${task_id}"
}

# Validate task ID format
# Valid formats: SPEC-001-T01, SPEC-002-T03
# Returns 0 if valid, 1 if invalid
validate_task_id() {
    local task_id="$1"
    if [[ ! "$task_id" =~ ^SPEC-[0-9]{3}-T[0-9]{2}$ ]]; then
        echo -e "${RED}ERROR${NC}: Invalid task ID format. Expected: SPEC-XXX-TXX (e.g., SPEC-001-T01)" >&2
        return 1
    fi
    return 0
}

# Check if container exists (running or stopped)
# Returns 0 if exists, 1 otherwise
container_exists() {
    local container_name="$1"
    docker ps -a --format '{{.Names}}' | grep -q "^${container_name}$"
}

# Check if container is running
# Returns 0 if running, 1 otherwise
container_running() {
    local container_name="$1"
    docker ps --format '{{.Names}}' | grep -q "^${container_name}$"
}

# Load environment variables from .env file
# Sets: CLAUDE_CODE_OAUTH_TOKEN, ANTHROPIC_API_KEY, GITHUB_TOKEN, GIT_USER_NAME, GIT_USER_EMAIL
load_env() {
    local script_dir="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"
    local env_file="${script_dir}/../.env"

    if [[ -f "$env_file" ]]; then
        # Parse .env file line by line
        while IFS= read -r line || [[ -n "$line" ]]; do
            # Skip comments and empty lines
            [[ "$line" =~ ^[[:space:]]*# ]] && continue
            [[ "$line" =~ ^[[:space:]]*$ ]] && continue

            # Extract key and value
            if [[ "$line" =~ ^[[:space:]]*([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*=[[:space:]]*(.*)[[:space:]]*$ ]]; then
                key="${BASH_REMATCH[1]}"
                value="${BASH_REMATCH[2]}"

                # Remove surrounding quotes (single or double)
                if [[ "$value" =~ ^\"(.*)\"$ ]] || [[ "$value" =~ ^\'(.*)\'$ ]]; then
                    value="${BASH_REMATCH[1]}"
                fi

                # Trim trailing whitespace after removing quotes
                value="${value%"${value##*[![:space:]]}"}"

                # Export specific variables
                case "$key" in
                    CLAUDE_CODE_OAUTH_TOKEN)
                        export CLAUDE_CODE_OAUTH_TOKEN="${CLAUDE_CODE_OAUTH_TOKEN:-$value}"
                        ;;
                    ANTHROPIC_API_KEY)
                        export ANTHROPIC_API_KEY="${ANTHROPIC_API_KEY:-$value}"
                        ;;
                    GITHUB_TOKEN)
                        export GITHUB_TOKEN="${GITHUB_TOKEN:-$value}"
                        ;;
                    GIT_USER_NAME)
                        export GIT_USER_NAME="${GIT_USER_NAME:-$value}"
                        ;;
                    GIT_USER_EMAIL)
                        export GIT_USER_EMAIL="${GIT_USER_EMAIL:-$value}"
                        ;;
                esac
            fi
        done < "$env_file"
    fi

    # Set defaults
    export GIT_USER_NAME="${GIT_USER_NAME:-Claude AI}"
    export GIT_USER_EMAIL="${GIT_USER_EMAIL:-claude@example.com}"
}

# Ensure Docker network exists
# Creates the network if it doesn't exist
ensure_network() {
    if ! docker network ls --format '{{.Name}}' | grep -q "^${NETWORK_NAME}$"; then
        echo -e "${BLUE}Creating Docker network: ${NETWORK_NAME}${NC}"
        docker network create "${NETWORK_NAME}" >/dev/null 2>&1 || true
    fi
}

# Get absolute path
# Converts relative path to absolute path
get_absolute_path() {
    local path="$1"
    if [[ -d "$path" ]]; then
        local dir
        dir=$(cd "$path" && pwd)
        echo "$dir"
    elif [[ -f "$path" ]]; then
        local dir filename
        dir=$(cd "$(dirname "$path")" && pwd)
        filename=$(basename "$path")
        echo "${dir}/${filename}"
    else
        echo "$path"
    fi
}

# Extract task number from task ID
# SPEC-001-T01 -> 1
# SPEC-001-T02 -> 2
get_task_number() {
    local task_id="$1"
    echo "$task_id" | sed -E 's/^SPEC-[0-9]{3}-T0?([0-9]+)$/\1/'
}
