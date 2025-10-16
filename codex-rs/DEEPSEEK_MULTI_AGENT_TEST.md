# DeepSeek Multi-Agent Test (Work in Progress)

The multi-agent prototype does not yet call DeepSeek; role planning still uses heuristic logic and no tool execution occurs. The commands below are retained for future validation once integration work lands.

## Prerequisites

```bash
export DEEPSEEK_API_KEY="sk-your-own-key"
```

## Planned Test Command

```bash
target/debug/codex multi-agent \
  --objective "create a simple python script to add two numbers and run test" \
  --monitor \
  --config model_provider=deepseek \
  --config model=deepseek-reasoner
```

## Current Status

- ❌ Role planning still uses heuristic rules; no DeepSeek API calls occur.
- ❌ Tool execution is disabled, so agents cannot modify or test code.
- ✅ Command-line plumbing exists and accepts DeepSeek provider flags.

Refer to [docs/multi-agent/PLANNING.md](docs/multi-agent/PLANNING.md) for the latest integration roadmap.
