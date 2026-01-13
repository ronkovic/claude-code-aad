# Autonomous AI-Driven Development Container
# Claude Code + (Optional) Gemini CLI
#
# Build:
#   docker build -t autonomous-dev .
#
# Run:
#   docker run -it -e CLAUDE_CODE_OAUTH_TOKEN="xxx" autonomous-dev

FROM node:20-bookworm

# Install system dependencies
RUN apt-get update && apt-get install -y \
    tmux \
    jq \
    git \
    curl \
    && apt-get autoremove -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install GitHub CLI
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | \
    dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg && \
    chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | \
    tee /etc/apt/sources.list.d/github-cli.list > /dev/null && \
    apt-get update && apt-get install gh -y && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Claude Code (latest version)
# If you need a specific version, change @latest to @2.1.1 etc.
RUN npm install -g @anthropic-ai/claude-code@latest && \
    npm cache clean --force

# Optional: Install Gemini CLI (uncomment if needed for blocked sites)
# RUN npm install -g @google/gemini-cli && \
#     npm cache clean --force

# Create claude user (don't run as root)
RUN useradd -m -s /bin/bash claude

# Set up directories
RUN mkdir -p /home/claude/.claude/scripts /home/claude/.claude/skills /home/claude/.gemini && \
    chown -R claude:claude /home/claude

USER claude
WORKDIR /home/claude

# Set up aliases for convenience
RUN echo "alias c='claude'" >> ~/.bashrc && \
    echo "alias g='gemini'" >> ~/.bashrc

# Set up status line in settings.json (using project-local path)
RUN echo '{\"statusLine\":{\"type\":\"command\",\"command\":\"~/.claude/scripts/context-bar.sh\"}}' > ~/.claude/settings.json

# Optional: Set up Gemini settings (uncomment if using Gemini)
# RUN echo '{\"security\":{\"auth\":{\"selectedType\":\"oauth-personal\"}},\"general\":{\"previewFeatures\":true},\"model\":{\"name\":\"gemini-2.0-flash\"}}' > ~/.gemini/settings.json

# Copy setup script
COPY --chown=claude:claude setup.sh /home/claude/setup.sh
RUN chmod +x /home/claude/setup.sh

# Working directory
# setup.shで環境変数に基づいて動的にディレクトリ変更するため、ここでは/home/claudeまで
WORKDIR /home/claude

# Health check - verify claude command is available and node is running
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
  CMD claude --version > /dev/null 2>&1 || exit 1

# Run setup on container start
CMD ["/home/claude/setup.sh"]
