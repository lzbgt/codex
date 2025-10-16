# Multi-Agent System Architecture (Prototype)

> The diagram below describes the intended design. The current implementation uses heuristic role planning, no tool execution, and minimal collaboration. Treat LLM references as future work unless explicitly noted. For up-to-date progress, refer to [PLANNING.md](./PLANNING.md); this architecture document is intended to change infrequently.

## Overview

**Dual-Mode System**:

1. **Original Codex** – Single-session usage
2. **Casual Multi-Agent (Prototype)** – Heuristic team creation with casual human engagement

## Casual Multi-Agent (Target Flow)

```
Objective → Planner (future: LLM) → Dynamic Team → Casual Collaboration → Result
                     ↖              ↖              ↖
                    Human drops in   Human provides  Human suggests
                    with info/data   guidance        new task
```

**Current Implementation Highlights**

- `CasualMultiAgentOrchestrator` – Manages sessions and heuristic agent execution.
- `AgentManager` – Provides rule-based role taxonomy and assignments.
- `CasualAgent` – Each agent gets a `ConversationId`; rollout persistence is experimental.
- Tool usage, web search, and auto-confirmation flows are not yet implemented.

**CLI Usage (Prototype)**: `codex multi-agent --objective "task" --monitor`

## Session Management

- Agents operate under dedicated `ConversationId`s.
- Rollout recorder flushes transcripts; resume logic remains limited.
- Auto-save occurs opportunistically during execution.

## Planned Enhancements

- LLM-driven role planning and task decomposition.
- Tool-enabled collaboration (shell, file operations, web search, apply_patch).
- Human-in-the-loop approval and replanning workflow.
- Conflict resolution, dependency management, and artifact review.

## File Map

```
core/src/multi_agent/
├── mod.rs                  # Module exports and structure
├── casual.rs               # Orchestrator, session management, agent execution
├── agents.rs               # Role taxonomy and heuristic planner
├── communication.rs        # Message types and in-memory queues
└── task_planner.rs         # Placeholder task planner
```

## Integration Points

- `ConversationManager` and `RolloutRecorder` for session persistence.
- Model provider stack delivers streamed responses (no tool calls yet).
- CLI (`cli/src/main.rs`) initialises the orchestrator and polling loop.

## Risks & Considerations

- Lack of tool execution prevents agents from modifying code or running tests.
- Blocking stdin confirmation hangs unattended runs; needs async refactor.
- Testing coverage is minimal, increasing regression risk.

## Testing (Current vs Planned)

- **Current:** `test_multi_agent.rs` smoke test.
- **Planned:** Unit tests for planner and messaging, integration tests for tool execution and session recovery, end-to-end scenarios covering replanning and human approvals.
