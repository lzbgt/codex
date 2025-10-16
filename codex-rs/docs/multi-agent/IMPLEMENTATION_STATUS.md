# Multi-Agent Implementation Status

_Last updated: October 16, 2025_

## Snapshot

- **LLM-backed planning is live.** The planner invokes the configured provider (OpenAI or DeepSeek) to compute domains, standards, role assignments, tasks, and risks. Fallback heuristics remain available through `CODEX_DISABLE_ROLE_PLANNING_LLM`.
- **Progress reporting reflects agent completion.** Agents mark themselves complete after tool runs, letting sessions advance to `TaskStatus::Completed` and pushing monitors to 100 % when all AI roles finish.
- **Provider overrides honoured.** Multi-agent model calls now consult the active config before falling back to built-ins, so DeepSeek base URLs and headers from `config.toml` are respected.
- **Shell tool execution enabled.** Agents can invoke the standard `shell` tool during collaboration; apply_patch and richer tooling remain on the roadmap.
- **Danger-mode sandbox by default.** Multi-agent sessions now start with `danger-full-access`, enabling unrestricted read/write/execute permissions and tool usage. Only run the CLI inside trusted sandboxes.
- **Docs temporarily conservative.** The README and DeepSeek test guides now match the current CLI surface area and highlight missing tooling support; richer feature docs stay paused until the implementation catches up.

## In Flight

1. **Tool execution (apply_patch & artifacts)** – Wire apply_patch and file-writing helpers into the orchestrator so agents can create artifacts instead of plain text summaries.
2. **Dependency-aware scheduling** – Extend the planner/task planner to model task dependencies and support replanning when humans intervene.
3. **Resume pathways** – Harden session resume flows and finish the rollback/persistence story.

## Recent Changes

- Refreshed multi-agent documentation and separated long-lived design docs from this status tracker.
- Normalised task planner readiness checks so completed tasks are excluded from subsequent runs.
- Added default sandbox overrides to core test helpers and updated prompt-caching tests for danger-mode serialization.

## Next Checkpoint

Track progress here whenever a major capability lands (tool execution, dependency graphs, persistence, human-in-the-loop UX). Design details live in [`IMPLEMENTATION.md`](./IMPLEMENTATION.md); roadmap thoughts belong in [`PLANNING.md`](./PLANNING.md).
