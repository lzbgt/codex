# DeepSeek Multi-Agent Test (Work in Progress)

The casual multi-agent prototype now streams planning output via DeepSeek when configured, but several gaps remain:

- ✅ Role planning and per-agent execution call DeepSeek's chat-completions API when `model_provider=deepseek`.
- ✅ Agents track completion, so monitors reach 100 % when all DeepSeek-backed roles finish.
- ⚠️ Only the `shell` tool is wired up; apply-patch, web search, and artifact creation remain disabled.
- ✅ User-defined DeepSeek overrides in `config.toml` are honoured during execution.

## Prerequisites

```bash
export DEEPSEEK_API_KEY="sk-your-own-key"
```

## Planned Test Command

```bash
target/debug/codex -c model_provider=deepseek -c model=deepseek-reasoner \
  multi-agent --objective "create a simple python script to add two numbers and run test" \
  --monitor
```

## Current Status

- ✅ DeepSeek API calls occur for planning and agent execution.
- ✅ Progress reporting reaches completion once DeepSeek agents finish their tasks.
- ⚠️ Only the `shell` tool runs today; apply-patch and richer tooling are pending.
- ✅ Provider overrides (custom base URL, headers) are respected.

Refer to [docs/multi-agent/IMPLEMENTATION_STATUS.md](docs/multi-agent/IMPLEMENTATION_STATUS.md) and [docs/multi-agent/PLANNING.md](docs/multi-agent/PLANNING.md) for the latest integration roadmap.
