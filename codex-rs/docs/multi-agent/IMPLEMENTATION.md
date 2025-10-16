# Multi-Agent System Implementation

## Current Status

### Prototype Scope

- Role planning uses the universal LLM prompt (OpenAI or DeepSeek) with standards-based fallback when the provider is unavailable.
- Agents stream responses but cannot run tools or collaborate.
- Session persistence is partially wired; resume support is minimal.
- CLI provisions monitoring output but still contains blocking prompts.
- Default sandbox mode is `danger-full-access`, granting write/execute and unrestricted tool use; only run the multi-agent CLI inside trusted sandboxes.

### Key Limitations

1. **No Tool Integration** – `Prompt` objects contain empty tool lists, so agents cannot execute filesystem/shell actions.
2. **Planner Depth** – Planner produces roles/tasks but lacks dependency tracking, replanning, and metric generation.
3. **Limited Coordination** – Messaging exists, but dependency handling and artifact review are unimplemented.
4. **Blocking Confirmation** – `start_planning_phase` still reads from stdin, which stalls background execution.
5. **Testing Gaps** – Only a smoke test exists; there is no coverage for planning or messaging flows.

### DeepSeek Integration Status

- DeepSeek and OpenAI providers are fully wired for role planning; the system selects whichever is configured and gracefully falls back to heuristics if the call fails.
- Environment variable `DEEPSEEK_API_KEY` must be provided by the user; no shared test key is bundled.
- Target work: surface provider/model selection per agent during execution and ensure downstream tool requests honour per-role preferences.

## Key Files
- `core/src/multi_agent/casual.rs` - Casual multi-agent with dynamic role planning
- `core/src/multi_agent/agents.rs` - Dynamic role taxonomy and AgentManager
- `core/src/multi_agent/communication.rs` - Inter-agent communication system
- `cli/src/main.rs` - CLI command implementations

## Next Steps

### HIGH PRIORITY
1. Provide tool specs (shell/file/apply_patch/web) and process tool events.
2. Deepen planner output (dependency graphs, success metrics, replanning hooks).
3. Remove blocking stdin flow; add asynchronous confirmation hooks.
4. Introduce automated tests for planning, messaging, and artifact workflows.

## Technical Architecture Notes

### Current Flow

1. CLI command → `run_casual_multi_agent_command()`
2. Load config → `Config::load_with_cli_overrides()`
3. Initialize casual multi-agent system → `api::init()`
4. Publish task → `api::publish_task()`
5. Dynamic role planning → `AgentManager::analyze_objective()` (LLM-backed with standards fallback)
6. Agent creation → `AgentManager::create_agents_from_analysis()`
7. Task execution → Streams model output (tool calls not yet enabled)

### Key Files Modified

- `core/src/multi_agent/casual.rs` - Casual multi-agent orchestrator with dynamic role planning
- `core/src/multi_agent/agents.rs` - Dynamic role taxonomy and AgentManager
- `cli/src/main.rs` - CLI command implementation
- `core/src/task_planner.rs` - Task planning system (not yet integrated)

### Integration Points Implemented

- Model provider system (`model_provider_info.rs`)
- Authentication system (`CodexAuth`)
- Tool calling infrastructure
- API client (`reqwest`)

## Implementation Details

### Casual Multi-Agent Collaboration with Dynamic Role Planning

#### Role Analysis (Current)
- Universal LLM prompt (documented in `core/dynamic_role_planning_prompt.md`) generates domain, standards, roles, task breakdown, and risk register.
- Standards-aligned taxonomy provides fallback output when the provider is unavailable.
- Complexity estimation blends objective length with team size; results are logged for operator awareness.
- Tests can disable live LLM calls by exporting `CODEX_DISABLE_ROLE_PLANNING_LLM=1`, forcing the fallback planner.

#### Standard Role Taxonomy
```rust
pub struct RoleTaxonomy {
    pub roles: HashMap<String, RoleDefinition>,
    pub domain_mappings: HashMap<String, Vec<String>>,
}

pub struct RoleDefinition {
    pub standard_role: String,            // "project_manager", "technical_lead", ...
    pub default_title: String,            // Human-friendly label
    pub description: String,              // Summary grounded in standards
    pub capabilities: Vec<String>,        // Capabilities used for agent expertise
    pub core_competencies: Vec<String>,   // Highlights used in summaries
    pub default_responsibilities: Vec<String>, // Fallback responsibilities
    pub suggested_provider: Option<String>,
    pub suggested_model: Option<String>,
}
```

#### Dynamic Agent Creation
- Planner output is mapped onto taxonomy entries to assemble agent metadata (capabilities, provider hints, instructions).
- Agents receive dedicated session IDs and rollout recorders.
- Human collaborator role is always included.

#### Session Management
- Each agent operates under an independent `ConversationId`.
- Rollout recorder flushes output to disk; resume flows are still basic.

#### Token Optimisation (Planned)
- Share intermediate context between agents.
- Reuse cached summaries to reduce duplicate LLM calls.
- Prefer web search before escalating to humans.

#### Casual Human Engagement
```rust
pub enum CasualAction {
    DropInfo { info: String, context: Option<String> },
    ProvideData { data_type: String, content: String },
    SuggestTask { objective: String },
    QuickGuidance { message: String, to_agent: Option<String> },
}

## Testing Status

- ⚠️ `test_multi_agent.rs` provides only a happy-path smoke test.
- ⚠️ No automated coverage for planning, messaging, or artifact workflows.
- ⚠️ Tool execution paths remain untested.

## Risk Assessment

- **⚠️ HIGH**: Lack of tool execution prevents agents from producing artifacts.
- **⚠️ MEDIUM**: Blocking stdin flow can hang headless executions.
- **⚠️ MEDIUM**: Planner output lacks dependency modelling and may omit critical follow-up tasks.

## Success Metrics Achieved

### Functional Requirements

1. ⚠️ **Role analysis** – LLM-backed planning is live, but dependency modelling and replanning are still missing.
2. ⚠️ **Execution** – Agents stream model output but cannot use tools.
3. ✅ **Agent identification** – Agent metadata recorded with sessions.
4. ⚠️ **Collaboration** – No dependency tracking or shared editing.
5. ✅ **Configuration integration** – CLI/core share configuration plumbing.

### Performance Requirements

- ✅ **Role analysis completes quickly** - Single LLM turn with fallback keeps latency manageable
- ⚠️ **Limited execution** - Streams model text only; no tool calls
- ✅ **Memory usage scales with number of agents** - Uses lightweight session maps.
- ⚠️ **Session persistence** - Rollouts flushed, but resume logic is incomplete.

## Implementation Summary

The casual multi-agent system currently:

1. **Analyzes objectives** using the universal LLM prompt (with standards-based fallback) to produce domains, roles, tasks, and risks.
2. **Creates agent teams** with per-agent sessions and rollout recording.
3. **Streams model output** without tool usage or collaboration.
4. **Exposes monitoring** via CLI polling APIs.
5. **Flushes session data** to disk, though resumptions are still experimental.

The system remains an unfinished prototype; significant engineering is required before the multi-agent mode can be relied upon for real work.

## Usage Examples

### Casual Multi-Agent with Dynamic Role Planning
```bash
# Start casual multi-agent collaboration with dynamic role planning
codex multi-agent --objective "Build a React frontend with Node.js backend"

# Monitor progress continuously
codex multi-agent --objective "Create a web application" --monitor

# Interactive engagement for casual human input
codex multi-agent --objective "Develop a machine learning pipeline" --interactive
```

## Next Steps

### HIGH PRIORITY

1. **Real Model Execution** - Replace simulated execution with real model calls
   - Integrate with Codex model provider system
   - Use agent-specific model providers from role taxonomy
   - Handle API errors and retries

2. **Planner Depth & Replanning** - Extend dynamic planning beyond initial role/task enumeration
   - Implement intelligent task decomposition and dependency tracking
   - Add replanning capability when objectives change mid-session
   - Surface success criteria and metrics derived from referenced standards

3. **Enhanced Coordination** - Implement sophisticated inter-agent communication
   - Real inter-agent messaging and collaboration
   - Shared artifact management
   - Cross-agent dependency resolution

### MEDIUM PRIORITY

1. **Session Management** - Improve session persistence and recovery
   - Full session persistence implementation
   - Session recovery capabilities
   - Auto-save during execution

2. **Token Optimization** - Enhance local information sharing and caching
   - Advanced local information sharing between agents
   - Improved cached data reuse to minimize redundant requests
   - Web search preference before human referral

## DeepSeek API Testing

### Configuring API Access
Set the `DEEPSEEK_API_KEY` environment variable with your own DeepSeek credential before running integration tests or CLI flows.

### Example API Call
```bash
curl https://api.deepseek.com/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${DEEPSEEK_API_KEY}" \
  -d '{
        "model": "deepseek-chat",
        "messages": [
          {"role": "system", "content": "You are a helpful assistant."},
          {"role": "user", "content": "Hello!"}
        ],
        "stream": false
      }'
```

### LLM System Prompt for Role Planning
When implementing real LLM planning, use a system prompt that explicitly instructs the LLM to use standard role specifications:

```
You are a role planning expert. Analyze the given objective and determine the required roles using standard international/government role specifications.

Use consistent role naming and capability definitions from standard taxonomies. Provide role assignments with clear descriptions, required capabilities, and estimated effort levels.

Standard role categories include: backend-developer, frontend-developer, data-scientist, content-writer, technical-writer, qa-engineer, security-auditor, project-coordinator.
```

### FUTURE ENHANCEMENTS

1. **Advanced Features**
   - Advanced token optimization
   - Enhanced monitoring and debugging tools
   - Performance optimization for large teams
