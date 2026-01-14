#!/usr/bin/env bash
set -euo pipefail

echo "Checking required files..."

# Core files
test -f README.md || { echo "❌ README.md not found"; exit 1; }
test -f CLAUDE.md || { echo "❌ CLAUDE.md not found"; exit 1; }
test -f LICENSE || { echo "❌ LICENSE not found"; exit 1; }
test -f .gitignore || { echo "❌ .gitignore not found"; exit 1; }

# Claude Code settings
test -f .claude/settings.json || { echo "❌ .claude/settings.json not found"; exit 1; }

# Commands (12 commands)
test -f .claude/commands/aad/init.md || { echo "❌ /aad:init command not found"; exit 1; }
test -f .claude/commands/aad/tasks.md || { echo "❌ /aad:tasks command not found"; exit 1; }
test -f .claude/commands/aad/worktree.md || { echo "❌ /aad:worktree command not found"; exit 1; }
test -f .claude/commands/aad/status.md || { echo "❌ /aad:status command not found"; exit 1; }
test -f .claude/commands/aad/integrate.md || { echo "❌ /aad:integrate command not found"; exit 1; }
test -f .claude/commands/aad/orchestrate.md || { echo "❌ /aad:orchestrate command not found"; exit 1; }
test -f .claude/commands/aad/gate.md || { echo "❌ /aad:gate command not found"; exit 1; }
test -f .claude/commands/aad/context.md || { echo "❌ /aad:context command not found"; exit 1; }
test -f .claude/commands/aad/handoff.md || { echo "❌ /aad:handoff command not found"; exit 1; }
test -f .claude/commands/aad/retro.md || { echo "❌ /aad:retro command not found"; exit 1; }
test -f .claude/commands/aad/clone.md || { echo "❌ /aad:clone command not found"; exit 1; }
test -f .claude/commands/aad/half-clone.md || { echo "❌ /aad:half-clone command not found"; exit 1; }

# Scripts
test -f .claude/scripts/context-bar.sh || { echo "❌ context-bar.sh not found"; exit 1; }
test -f .claude/scripts/install-to-new.sh || { echo "❌ install-to-new.sh not found"; exit 1; }
test -f .claude/scripts/install-to-existing.sh || { echo "❌ install-to-existing.sh not found"; exit 1; }

# Docker
test -f .aad/container/Dockerfile || { echo "❌ Dockerfile not found"; exit 1; }
test -f .aad/container/docker-compose.yml || { echo "❌ docker-compose.yml not found"; exit 1; }
test -f .aad/container/.env.example || { echo "❌ .env.example not found"; exit 1; }

# Templates
test -f .aad/templates/SPEC-TEMPLATE.md || { echo "❌ SPEC-TEMPLATE.md not found"; exit 1; }
test -f .aad/templates/TASK-TEMPLATE.md || { echo "❌ TASK-TEMPLATE.md not found"; exit 1; }
test -f .aad/templates/RETRO-TEMPLATE.md || { echo "❌ RETRO-TEMPLATE.md not found"; exit 1; }
test -f .aad/templates/TEMPLATE.md || { echo "❌ TEMPLATE.md not found"; exit 1; }

# Documentation
test -f .aad/WORKFLOW.md || { echo "❌ WORKFLOW.md not found"; exit 1; }
test -f .aad/COMMANDS.md || { echo "❌ COMMANDS.md not found"; exit 1; }
test -f .aad/SETUP.md || { echo "❌ SETUP.md not found"; exit 1; }

echo "✅ All required files present"
