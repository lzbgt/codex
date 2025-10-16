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

## Multi-Agent Systems

### Casual Multi-Agent (Recommended)
```bash
codex multi-agent --objective "your task" --monitor
```

### DeepSeek Test
```bash
export DEEPSEEK_API_KEY=your_key
codex multi-agent --objective "Test DeepSeek" --monitor --config model_provider=deepseek --config model=deepseek-reasoner
```

### Traditional Multi-Agent
```bash
codex multi-agent --agents agents.toml --objective "task"
```

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