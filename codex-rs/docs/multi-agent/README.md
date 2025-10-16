# Multi-Agent System

> For day-to-day status updates, see [`IMPLEMENTATION_STATUS.md`](./IMPLEMENTATION_STATUS.md). This README is a stable orientation guide.

## Quick Start

```bash
codex multi-agent --objective "build feature X" --monitor
```

## System Highlights
- **Dynamic Role Planning** – Objectives are normalised via the universal prompt and mapped to a standards-informed taxonomy. The planner integrates with OpenAI or DeepSeek and falls back to deterministic heuristics when disabled.
- **Casual Collaboration** – Humans can drop information, supply data, or suggest tasks without formally joining. Messages and artifacts are captured in the session state for later review.
- **Persistence Hooks** – Sessions flush rollout logs and agent transcripts; manual resume is supported while richer recovery flows continue to evolve.
- **Danger-Mode Sandbox** – The CLI defaults to `danger-full-access`, enabling unrestricted read/write/execute and tool usage. Override the sandbox if you need stricter isolation.

## Documentation
- [IMPLEMENTATION_STATUS.md](./IMPLEMENTATION_STATUS.md) – Future work, current gaps, and backlog checkpoints
- [IMPLEMENTATION.md](./IMPLEMENTATION.md) – Architecture and component details
- [PLANNING.md](./PLANNING.md) – Roadmap and dependency planning
- [ARCHITECTURE.md](./ARCHITECTURE.md) – Conceptual diagrams and system boundaries
