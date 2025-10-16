# Multi-Agent System

## Quick Start

**Casual Multi-Agent with Dynamic Role Planning**
```bash
codex multi-agent --objective "task" --monitor
```

## Current Capabilities
- **Rule-Based Role Planning (prototype)**: Objectives are analysed with keyword heuristics to pick canned roles; LLM integration is not wired up yet.
- **Casual Human Engagement**: Human can drop in/out without formal joining.
- **Session Persistence (partial)**: Sessions are created and auto-saved, but recovery flows are still experimental.
- **Web Search Preference (planned)**: Hooks exist, but agents currently execute without tool usage or web search.
- **DeepSeek Provider (planned)**: Provider metadata exists, but multi-agent planning still uses heuristic logic and does not call DeepSeek today.

> **Note:** The multi-agent system is an early prototype. Planning, collaboration, and tool execution are still largely stubbed out and require significant follow-up work before production use.

## Documentation
- [PLANNING.md](./PLANNING.md) - Canonical status & next steps
- [IMPLEMENTATION.md](./IMPLEMENTATION.md) - Technical details
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System design
