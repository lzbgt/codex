# Multi-Agent System Implementation

This document captures the architecture of the codex multi-agent prototype. For live progress updates and backlog tracking, see [`IMPLEMENTATION_STATUS.md`](./IMPLEMENTATION_STATUS.md).

## Overview

A multi-agent session is launched from the CLI. The orchestrator loads the project configuration, provisions a conversation in codex-core, and hands the user objective to the dynamic planner. The planner calls into the selected LLM (DeepSeek or OpenAI) to derive domain context, standards, role assignments, task breakdown, and risk notes. The resulting plan is converted into agent records, workspace state, and a task queue. During execution, the orchestrator streams output events, allows casual human participation, and periodically persists the session. The default sandbox profile is `danger-full-access`, which assumes the invocation is inside a trusted test environment and enables unrestricted tool usage.

```
CLI → Orchestrator → LLM Planner → Agent Plan → Collaboration Loop → Persistence
```

## Planner Layer (`core/src/multi_agent/agents.rs`)

- Normalises the LLM response into a `RoleAnalysis` structure. Each role contains a standard identifier, human-friendly name, core competencies, and actionable responsibilities.
- Falls back to standards-based heuristics when the planner call fails or is explicitly disabled via `CODEX_DISABLE_ROLE_PLANNING_LLM`.
- Supplies helper utilities for creating agent profiles, estimating complexity, deriving default standards/risks, and enumerating taxonomy metadata.

## Orchestrator Layer (`core/src/multi_agent/casual.rs`)

- Publishes and tracks casual sessions (`publish_task`, `create_dynamic_team`, `start_agent_collaboration`).
- Converts the `RoleAnalysis` into runtime `AgentRole` records, automatically includes a human collaborator role, and prints a planning summary for observability.
- Manages per-agent conversations, rollout recorders, and message queues; human actions enter the system through the `CasualAction` enum.
- Uses the shared `ConversationManager` to provision conversations and rollouts for persistence/resume.

## Execution & Tooling

- `start_agent_collaboration` launches asynchronous tasks that stream model responses for each AI agent. Tool usage is still disabled; the orchestrator currently routes output as text-only events.
- The unified executor honours the session’s sandbox policy. Because the default is `danger-full-access`, tool invocations run without extra approval. Test fixtures that require stricter isolation must override the sandbox before launching a session.
- Apply-patch invocations surface detailed error messages; failures include guidance to install the `codex-run-as-apply-patch` helper so local tests know why an invocation could not be executed.

## State & Persistence

- Each session maintains `CasualTaskSession` state that tracks agents, status, recent messages, and stored artifacts.
- Rollouts are flushed on demand (`save_session_state`, `auto_save_session_state`) and reused during resume flows (`resume_session`).
- The orchestrator supports manual resume, but automated resume flows are still evolving (see `IMPLEMENTATION_STATUS.md` for progress).

## Key Types

- `RoleAnalysis`, `RoleAssignment`, `RiskRegisterEntry` – data returned by the planner.
- `AgentRole`, `AgentPlan`, `CasualAgent` – runtime agent representations.
- `CasualTaskSession`, `CasualAction`, `CasualMessage`, `CasualArtifact` – collaboration structures.

## Configuration Notes

- `Config::load_from_base_config_with_overrides` merges profile overrides, user-provided TOML, and CLI overrides. Profiles can define model/provider pairs, reasoning preferences, and prompt cache templates.
- `sandbox_mode` and `sandbox_workspace_write` control the executor’s sandbox; danger mode is the default to simplify multi-agent experiments.
- The planner honours project documentation (`AGENTS.md` hierarchy) through `Config::load_instructions` and the taxonomy definitions.

## Files of Interest

| File | Purpose |
| --- | --- |
| `core/src/multi_agent/agents.rs` | Planner taxonomy, LLM invocation, and fallbacks |
| `core/src/multi_agent/casual.rs` | Orchestrator, agent lifecycle, session state |
| `core/src/tools/mod.rs` | Tool execution harness (shell/apply_patch) |
| `core/src/task_planner.rs` | Task decomposition helpers used by agents |
| `core/src/config.rs` | Configuration loading, profile handling, sandbox selection |
| `docs/multi-agent/ARCHITECTURE.md` | High-level architecture diagrams and component relationships |

## Related Documents

- [`IMPLEMENTATION_STATUS.md`](./IMPLEMENTATION_STATUS.md) – progress tracking and backlog checkpoints.
- [`PLANNING.md`](./PLANNING.md) – heuristics, dependency mapping, and future roadmap.
- [`ARCHITECTURE.md`](./ARCHITECTURE.md) – conceptual diagrams and system boundaries.
