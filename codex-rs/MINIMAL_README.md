# Codex CLI (Rust) - Minimal Docs

## Quick Start

```bash
# Install
npm i -g @openai/codex

# Interactive mode
codex

# Non-interactive
codex exec "your task"
```

## Multi-Agent Systems (Experimental)

### Casual Multi-Agent
```bash
# Prompted objective (monitoring by default)
codex multi-agent

# Explicit objective
codex multi-agent --objective "your task"

# Opt into interactive guidance
codex multi-agent --objective "your task" --interactive
```

Session transcripts are also written to `<cwd>/.codex-logs/multi-agent-*.log`.

### DeepSeek Test Path
```bash
export DEEPSEEK_API_KEY=your_key
codex -c model_provider=deepseek -c model=deepseek-reasoner \
  multi-agent --objective "Test DeepSeek" --monitor
```

> Only the `shell` tool is wired up today; apply_patch and richer artifact tooling are still under development.

## Key Features
- Multi-agent collaboration
- OpenAI & DeepSeek providers
- File search with `@`
- Edit history with `Esc-Esc`
- MCP server/client support

## Config
`~/.codex/config.toml`
```toml
[model_providers.openai]
env_key = "OPENAI_API_KEY"

[model_providers.deepseek]
env_key = "DEEPSEEK_API_KEY"
```
