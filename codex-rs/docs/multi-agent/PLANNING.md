# Multi-Agent System Planning

> **Status:** This document describes the target architecture. The current prototype still uses keyword-based planning, no tool execution, and limited collaboration. Treat the LLM references below as future work unless otherwise noted.

## Current Implementation Status

### Prototype Snapshot

- Role planning uses keyword heuristics; LLM calls are not connected yet.
- Agents stream model text but cannot run tools or coordinate on artifacts.
- Session persistence and resume flows remain experimental.
- CLI offers monitoring, but confirmation prompts still block on stdin.

### Near-Term Focus

1. Enable tool usage (shell/file/search) and handle tool call events.
2. Replace heuristic planning with DeepSeek-backed (or richer heuristic) task decomposition.
3. Remove blocking confirmation and expose async human-in-the-loop controls.
4. Build automated tests that cover planning, messaging, and artifact workflows.

## Key Files
- `core/src/multi_agent/casual.rs` - Casual multi-agent implementation with dynamic role planning
- `core/src/multi_agent/agents.rs` - Dynamic role taxonomy and AgentManager
- `core/src/multi_agent/communication.rs` - Inter-agent communication system
- `cli/src/main.rs` - CLI command definitions

## Usage Examples
```bash
# Casual multi-agent with dynamic role planning
codex multi-agent --objective "Build React frontend with Node.js backend" --monitor

# Interactive mode for casual human engagement
codex multi-agent --objective "Create a data analysis pipeline" --interactive
```

## Architecture Design

### Casual Multi-Agent Workflow

```
User Task → LLM Planner → Dynamic Agent Team → Collaborative Execution → Final Result
                     ↖              ↖              ↖
                    Human drops in   Human provides  Human suggests
                    with info/data   guidance        new task
```

### Key Components

#### 1. Task Publisher

```rust
pub struct TaskPublisher {
    pub async fn publish_task(objective: String) -> TaskSession
}

pub struct TaskSession {
    pub task_id: String,
    pub objective: String,
    pub status: TaskStatus,
    pub agents: Vec<Agent>,
    pub messages: Vec<AgentMessage>,
    pub artifacts: HashMap<String, Artifact>,
}
```

#### 2. Planner

- **Current:** Keyword heuristics assign canned roles and tasks.
- **Target:** LLM analyses the objective, creates task breakdowns, and assigns roles dynamically.

#### 3. Agent System

```rust
pub struct Agent {
    pub id: String,
    pub role: String,           // "backend-developer", "frontend-developer", "qa-engineer", "human"
    pub expertise: Vec<String>, // Dynamic from LLM planning
    pub status: AgentStatus,
    pub current_task: Option<String>,
    pub agent_type: AgentType,  // AI or Human
}

pub enum AgentType {
    AI,
    Human,
}

pub enum AgentStatus {
    Idle,
    Working,
    Blocked,
    Completed,
    WaitingForHumanInput,
}
```

#### 4. Communication System

```rust
pub enum AgentMessage {
    TaskUpdate {
        agent_id: String,
        task_id: String,
        status: String,
        progress: String,
        blockers: Vec<String>,
    },
    CoordinationRequest {
        from_agent: String,
        to_agent: String,
        request: String,
        context: String,
    },
    ArtifactShared {
        artifact_id: String,
        name: String,
        content: String,
        shared_with: Vec<String>,
    },
    StatusReport {
        agent_id: String,
        current_work: String,
        next_steps: String,
        help_needed: bool,
    },
    HumanInputRequest {
        from_agent: String,
        request: String,
        context: String,
        urgency: UrgencyLevel,
    },
    HumanGuidance {
        from_agent: String,  // "human"
        to_agent: Option<String>,  // None = broadcast
        guidance: String,
        data: Option<String>,
        new_task_suggestion: Option<String>,
    },
    TaskCreation {
        created_by: String,
        new_objective: String,
        priority: PriorityLevel,
    },
}

pub enum UrgencyLevel {
    Low,
    Medium,
    High,
    Critical,
}

pub enum PriorityLevel {
    Low,
    Normal,
    High,
}
```

## Implementation Architecture

### Casual Multi-Agent System Design

#### Zero Breaking Changes

- **Existing Codex CLI** - `codex`, `codex exec`, etc. work exactly as before
- **OpenAI/DeepSeek integration** - All model providers continue working
- **Configuration** - Existing `config.toml` format unchanged
- **Backward compatibility** - Zero breaking changes

#### Multi-Agent Module (✅ IMPLEMENTED)

##### 1. Task Publisher & Casual Human Interface

```rust
// Multi-agent module with dynamic role planning
pub mod multi_agent {
    // Human can casually engage without formal joining
    pub async fn publish_task(objective: String) -> Result<TaskSession>

    // Casual human interaction - no formal "joining" required
    pub async fn casually_engage(task_id: &str, action: CasualAction) -> Result<()>

    // Real-time monitoring - human can peek anytime
    pub async fn peek_at_progress(task_id: &str) -> Result<TaskSnapshot>
    pub async fn get_recent_messages(task_id: &str) -> Result<Vec<AgentMessage>>
}

// Casual human actions - lightweight, no commitment
pub enum CasualAction {
    DropInfo { info: String, context: Option<String> },
    ProvideData { data_type: String, content: String },
    SuggestTask { objective: String },
    QuickGuidance { message: String, to_agent: Option<String> },
}

// Lightweight snapshot for casual checking
pub struct TaskSnapshot {
    pub status: String,
    pub active_agents: Vec<String>,
    pub recent_activity: String,
    pub human_attention_needed: bool,
}
```

#### 2. Planning Engine

- **Current:** Heuristic role analysis picks canned roles and subtasks.
- **Target:** LLM-driven analysis breaks objectives into collaborative subtasks, spawns AI agents, and includes a human collaborator.

#### 3. Collaborative Communication System

- **Multi-channel messaging** - AI↔AI, AI↔Human, Human↔AI
- **Urgency-based routing** - Critical requests get human attention
- **Context preservation** - Full conversation history
- **Real-time updates** - Live status and progress

#### 4. Casual Human Engagement Patterns

- **Web Search First** - Agents prefer web search for research before human referral
- **Human for Blocking Issues** - Only ask humans for real blocking issues (credentials, decisions)
- **Drop-in participation** - Human can casually provide info/data anytime
- **Lightweight guidance** - Quick suggestions without formal coordination
- **Task suggestions** - Casual "what if we also..." ideas
- **Data drops** - Share files, credentials, API keys when convenient
- **Progress peeking** - Quick status checks without commitment

## Roadmap

| Horizon | Focus | Notes |
| --- | --- | --- |
| **Now** | Tool execution, DeepSeek planning, async confirmation | Provide shell/file/search/apply_patch tooling, wire DeepSeek planning path, replace blocking stdin confirmation. |
| **Next** | Collaboration workflows & replanning | Add dependency management, artifact review, human approval, and replanning hooks. |
| **Later** | Performance & observability | Improve session recovery, telemetry, and scaling for longer tasks. |

> Architecture and implementation reference documents are kept stable; use this planning document for incremental updates.

## Success Criteria (Target)

- Single objective produces a coordinated multi-agent plan.
- Tool-enabled agents generate, edit, and test code.
- Human approvals and replanning loops integrate smoothly.
- Planning and collaboration remain responsive for typical tasks.
- Sessions can be paused/resumed with minimal data loss.

_The current prototype does **not** yet satisfy these criteria; see the roadmap above for active work items._

## Expected Experience

- **Today:** Agents stream model responses but cannot modify code or tests; human input is manual via CLI.
- **Future:** Tool-enabled agents collaborate autonomously, surface approval prompts, and leverage DeepSeek planning for richer task decomposition.

### Casual Human Engagement

- **Drop in/out** - No formal joining, just casual participation
- **Data drops** - Share credentials, files when convenient
- **Lightweight suggestions** - Quick "what if" ideas without commitment
- **Progress peeking** - Quick status checks, no continuous monitoring
- **Minimal coordination** - AI agents handle most collaboration autonomously

### Key Benefit: Zero Breaking Changes

- **Standalone Codex** - All existing workflows preserved
- **OpenAI/DeepSeek** - All model providers continue working
- **Configuration** - No changes to existing setup
- **CLI Commands** - All existing commands unchanged
- **Multi-Agent** - New capability, optional to use

Users can continue using Codex exactly as they do today, while having the option to use the new multi-agent collaboration system when needed for complex projects.
