# Multi-Agent Implementation Status

_Last updated: October 16, 2025_

## Snapshot

- **LLM-backed planning is live.** The planner invokes the configured provider (OpenAI or DeepSeek) to compute domains, standards, role assignments, tasks, and risks. Fallback heuristics are available through `CODEX_DISABLE_ROLE_PLANNING_LLM`.
- **Danger-mode sandbox by default.** Multi-agent sessions now start with `danger-full-access`, enabling unrestricted read/write/execute permissions and tool usage. Only run the CLI inside trusted sandboxes.
- **Apply-patch guidance surfaced.** When the apply_patch shim is missing, failures include an explicit hint to install `codex-run-as-apply-patch` so tooling tests can diagnose the issue.
- **Collaboration loop remains text-only.** Agents stream model output, but tool execution and cross-agent coordination workflows are still being wired up.
- **Tests expanded.** Config precedence, prompt-caching, and tool harness suites were updated to match the new planner output and sandbox defaults; full end-to-end coverage is still in progress.

## In Flight

1. **Tool execution** – Wire shell/file/apply_patch tools into the orchestrator so agents can create artifacts instead of plain text.
2. **Dependency-aware scheduling** – Extend the planner/task planner to model task dependencies and support replanning when humans intervene.
3. **Resume pathways** – Harden session resume flows and finish the rollback/persistence story.

## Recent Changes

- Refreshed multi-agent documentation and separated long-lived design docs from this status tracker.
- Normalised task planner readiness checks so completed tasks are excluded from subsequent runs.
- Added default sandbox overrides to core test helpers and updated prompt-caching tests for danger-mode serialization.

## Next Checkpoint

Track progress here whenever a major capability lands (tool execution, dependency graphs, persistence, human-in-the-loop UX). Design details live in [`IMPLEMENTATION.md`](./IMPLEMENTATION.md); roadmap thoughts belong in [`PLANNING.md`](./PLANNING.md).
