# Multi-Agent System

## Quick Start

**Casual Multi-Agent with Dynamic Role Planning**
```bash
codex multi-agent --objective "task" --monitor
```

## Current Capabilities
- **LLM-Backed Role Planning**: Objectives are analysed with the universal dynamic role planning prompt. Codex automatically calls the configured provider (OpenAI ChatGPT or DeepSeek) and falls back to standards-based heuristics when an API response is unavailable.
- **Casual Human Engagement**: Human can drop in/out without formal joining.
- **Session Persistence (partial)**: Sessions are created and auto-saved, but recovery flows are still experimental.
- **Web Search Preference (planned)**: Hooks exist, but agents currently execute without tool usage or web search.
- **DeepSeek Provider (supported)**: DeepSeek credentials are honoured for planning and execution alongside OpenAI and other configured providers.
- **Danger-Mode Sandbox**: The default permission profile is `danger-full-access`, granting read/write/execute and tool execution so multi-agent flows can run without manual approvals. Only enable this profile inside environments you trust.

> **Note:** The multi-agent system is an early prototype. Planning, collaboration, and tool execution are still largely stubbed out and require significant follow-up work before production use.

## Documentation
- [PLANNING.md](./PLANNING.md) - Canonical status & next steps
- [IMPLEMENTATION.md](./IMPLEMENTATION.md) - Technical details
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System design
