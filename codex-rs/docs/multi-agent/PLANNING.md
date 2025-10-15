# Multi-Agent System Planning

## Current Implementation Status

### ✅ IMPLEMENTED

**Casual Multi-Agent with Dynamic Role Planning**
- Dynamic agent team creation using LLM-based role analysis
- Standard role taxonomy for consistent role definitions
- Session persistence with dedicated ConversationId per agent
- Web search preference before human referral
- Casual human engagement without formal joining
- CLI: `codex multi-agent --objective "task" --monitor --interactive`

### ⚠️ CURRENT LIMITATIONS

1. **Simulated Execution**: Uses `tokio::time::sleep()` instead of real API calls
2. **Rule-based Planning**: Simple keyword matching instead of actual LLM analysis
3. **Basic Coordination**: Limited inter-agent communication
4. **Missing Standard Role Specifications**: LLM planning doesn't use explicit system prompts for standard role formats

## Next Steps

### HIGH PRIORITY
1. **Real Model Execution**: Replace sleep with actual LLM API calls
   - Use DeepSeek API with test key: `sk-1181374854cb4c96ae891592dfd12815`
   - Implement proper API integration with error handling
2. **Enhanced LLM Planning**: Improve dynamic role analysis with actual LLM calls
   - Add explicit system prompt for standard role specifications (international/gov standards)
   - Use consistent role naming and capability definitions
3. **Enhanced Coordination**: Add sophisticated inter-agent communication

### MEDIUM PRIORITY
1. **Session Recovery**: Improve session resumption capabilities
2. **Token Optimization**: Enhance local information sharing
3. **Error Handling**: Add comprehensive error recovery

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

#### 2. LLM Planner

- Analyzes the objective
- Determines required roles and expertise
- Creates initial task breakdown
- Assigns roles to virtual agents
- Sets up communication channels

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

#### 2. LLM Planning Engine

- **Dynamic role analysis** - LLM identifies needed expertise
- **Task decomposition** - Breaks objective into collaborative subtasks
- **Agent creation** - Spawns AI agents with specific roles
- **Human integration** - Always includes "human" agent in the team

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

## Implementation Strategy

### Preserve Existing System (Zero Changes)

- **Standalone Codex** - All existing commands remain unchanged
- **OpenAI/DeepSeek** - Model providers continue working exactly as before
- **Configuration** - Current `config.toml` format preserved
- **CLI Interface** - No breaking changes to existing commands

### Build Casual Multi-Agent System (✅ IMPLEMENTED)

#### Step 1: Create Multi-Agent Module ✅ COMPLETED

- New `codex::multi_agent` namespace with dynamic role planning ✅
- Separate from existing Codex core ✅
- Uses same model provider infrastructure ✅
- Standard role taxonomy for consistent role definitions ✅

#### Step 2: Add Multi-Agent CLI Command ✅ COMPLETED

```bash
# Casual multi-agent with dynamic role planning
codex multi-agent "Build a full-stack application"

# Monitor mode for continuous progress watching
codex multi-agent --objective "Create web app" --monitor

# Interactive mode for casual human engagement
codex multi-agent --objective "Develop ML pipeline" --interactive

# Existing commands continue working
codex exec "Write simple function"
codex --provider openai
```

#### Step 3: Implement Dynamic Role Planning ✅ COMPLETED

- AgentManager with standard role taxonomy ✅
- LLM-based role analysis (rule-based simulation) ✅
- Dynamic agent creation based on objective ✅
- Domain-specific role mappings ✅

#### Step 4: Add Casual Human Engagement ✅ COMPLETED

- Human agent integration ✅
- Real-time communication ✅
- Session persistence ✅
- Casual engagement patterns ✅

## Success Metrics

### Functional Requirements

- ✅ **Single task input** - User provides one objective
- ✅ **Automatic planning** - LLM handles all decomposition (rule-based simulation)
- ✅ **Dynamic agents** - Roles created based on task needs
- ✅ **Collaborative execution** - Agents communicate and coordinate (basic implementation)
- ✅ **Real-time status** - Progress visible throughout execution

### Performance Requirements

- **Planning time** < 30 seconds for most tasks
- **Agent response time** < 10 seconds for coordination
- **Task completion** within reasonable time for complexity
- **Memory usage** scales with number of active agents

## Risk Assessment

### Technical Risks

1. **LLM Planning Quality** - Start with simple objectives, iterate
2. **Agent Coordination Complexity** - Implement basic communication first
3. **Performance Issues** - Add timeouts and resource limits

### Implementation Risks

1. **Scope Creep** - Focus on core workflow first
2. **Integration Complexity** - Build alongside existing system
3. **Testing Coverage** - Implement comprehensive test suite

## Expected User Experience

### Standalone Codex (Preserved)

```bash
# All existing commands continue working exactly as before
codex exec "Write a Python function to calculate fibonacci numbers"
codex --provider openai --model gpt-4o
codex --cd /path/to/project

# Output: Traditional single-agent execution
```

### Casual Multi-Agent Collaboration (✅ IMPLEMENTED)

```bash
# User starts casual multi-agent task
codex multi-agent "Build a React frontend with Node.js backend for a todo app"

# System responds:
Task published: todo-app-1234
Planning phase... ✓
LLM analysis: Web development project
Created 3 agents: [backend-developer, frontend-developer, qa-engineer]

🟢 backend-developer: Starting API development
🟡 frontend-developer: Beginning React component creation
🟢 qa-engineer: Setting up test framework

# AI agents collaborate autonomously:
[backend-developer → frontend-developer]: "API endpoints ready. Sharing OpenAPI spec..."
[frontend-developer → backend-developer]: "Got the spec. Implementing components now."
[qa-engineer → backend-developer]: "Need test data for integration tests"
[backend-developer → qa-engineer]: "Here's sample data: {users: [...]}"

# Human casually drops in:
[human → all]: "Use PostgreSQL instead of SQLite for scalability"
[backend-developer → human]: "Updated to PostgreSQL. Need connection details?"
[human → backend-developer]: "postgres://user:pass@localhost:5432/todoapp"

# AI agents continue collaborating:
[frontend-developer → qa-engineer]: "Components ready for testing"
[qa-engineer → frontend-developer]: "Running integration tests now..."

# Human casually suggests new feature:
[human → all]: "What if we add real-time notifications?"
[system]: New task created: "Add WebSocket notifications"
[backend-developer → frontend-developer]: "I'll handle WebSocket server, you do client?"
[frontend-developer → backend-developer]: "Sounds good, let me know when ready"

Task completed! AI agents collaborated autonomously with casual human input.
```

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
