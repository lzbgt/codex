# Multi-Agent System Implementation

## Current Status

### Prototype Scope

- Role planning uses keyword heuristics; no live LLM calls yet.
- Agents stream responses but cannot run tools or collaborate.
- Session persistence is partially wired; resume support is minimal.
- CLI provisions monitoring output but still contains blocking prompts.

### Key Limitations

1. **No Tool Integration** – `Prompt` objects contain empty tool lists, so agents cannot execute filesystem/shell actions.
2. **Placeholder Planning** – `TaskPlanner` returns canned subtasks and assignments.
3. **Limited Coordination** – Messaging exists, but dependency handling and artifact review are unimplemented.
4. **Blocking Confirmation** – `start_planning_phase` still reads from stdin, which stalls background execution.
5. **Testing Gaps** – Only a smoke test exists; there is no coverage for planning or messaging flows.

### DeepSeek Integration Status

- The DeepSeek provider is defined in the model registry, but multi-agent planning still uses heuristic role assignment. No API calls are made to DeepSeek today.
- Environment variable `DEEPSEEK_API_KEY` must be provided by the user; no shared test key is bundled.
- Target work: introduce DeepSeek-backed role planning, surface provider/model selection per agent, and ensure requests send tool definitions compatible with the Chat Completions API.

## Key Files
- `core/src/multi_agent/casual.rs` - Casual multi-agent with dynamic role planning
- `core/src/multi_agent/agents.rs` - Dynamic role taxonomy and AgentManager
- `core/src/multi_agent/communication.rs` - Inter-agent communication system
- `cli/src/main.rs` - CLI command implementations

## Next Steps

### HIGH PRIORITY
1. Provide tool specs (shell/file/apply_patch/web) and process tool events.
2. Replace heuristic planning with LLM-backed analysis (DeepSeek) or richer heuristics.
3. Remove blocking stdin flow; add asynchronous confirmation hooks.
4. Introduce automated tests for planning, messaging, and artifact workflows.

## Technical Architecture Notes

### Current Flow

1. CLI command → `run_casual_multi_agent_command()`
2. Load config → `Config::load_with_cli_overrides()`
3. Initialize casual multi-agent system → `api::init()`
4. Publish task → `api::publish_task()`
5. Dynamic role planning → `AgentManager::analyze_objective()` (rule-based)
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
- Predefined role taxonomy and domain mappings.
- Keyword-based classification converts objectives into role assignments.
- Complexity estimation scales with objective length and team size.
- LLM prompts are planned but not yet invoked.

#### Standard Role Taxonomy
```rust
pub struct RoleTaxonomy {
    pub roles: HashMap<String, RoleDefinition>,
    pub domain_mappings: HashMap<String, Vec<String>>,
}

pub struct RoleDefinition {
    pub name: String,                    // "backend-developer", "data-scientist"
    pub description: String,             // Role description
    pub capabilities: Vec<String>,       // Core capabilities
    pub suggested_provider: String,      // "deepseek", "openai"
    pub suggested_model: String,         // "deepseek-coder", "gpt-4o"
    pub common_tasks: Vec<String>,       // Typical tasks for this role
}
```

#### Dynamic Agent Creation
- Rule-based planner determines required roles from taxonomy.
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
- **⚠️ MEDIUM**: Rule-based planner may generate irrelevant teams/tasks.

## Success Metrics Achieved

### Functional Requirements

1. ⚠️ **Role analysis** – Heuristic only; no LLM integration.
2. ⚠️ **Execution** – Agents stream model output but cannot use tools.
3. ✅ **Agent identification** – Agent metadata recorded with sessions.
4. ⚠️ **Collaboration** – No dependency tracking or shared editing.
5. ✅ **Configuration integration** – CLI/core share configuration plumbing.

### Performance Requirements

- ✅ **Role analysis completes quickly** - Rule-based analysis is fast
- ⚠️ **Limited execution** - Streams model text only; no tool calls
- ✅ **Memory usage scales with number of agents** - Uses lightweight session maps.
- ⚠️ **Session persistence** - Rollouts flushed, but resume logic is incomplete.

## Implementation Summary

The casual multi-agent system currently:

1. **Analyzes objectives** using rule-based role assignment with the shared taxonomy.
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

2. **Enhanced LLM Planning** - Replace rule-based with actual LLM role assignment using standard role specifications
   - Use actual LLM calls for dynamic role analysis
   - Implement intelligent task decomposition
   - Add replanning capability when user requests changes
   - Ensure LLM uses consistent role naming and capability definitions from standard taxonomies

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
