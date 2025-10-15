# Multi-Agent System Architecture

## Overview

**Dual-Mode System**:

1. **Original Codex** - Single session usage
2. **Casual Multi-Agent** - Dynamic team creation with casual human engagement

## Casual Multi-Agent

**Architecture**:

```
Objective → LLM Planning → Dynamic Team → Casual Collaboration → Result
                     ↖              ↖              ↖
                    Human drops in   Human provides  Human suggests
                    with info/data   guidance        new task
```

**Key Components**:

- `CasualMultiAgentOrchestrator` - Dynamic team creation using LLM-based role planning
- `AgentManager` - Dynamic role taxonomy and analysis
- `CasualAgent` - Dedicated ConversationId per agent
- Session persistence with `RolloutRecorder`
- Web search preference before human referral

**Usage**: `codex multi-agent --objective "task" --monitor --interactive`

**Dynamic Role Planning**:
- LLM analyzes objective and determines required roles
- Standard role taxonomy provides consistent role definitions
- No hardcoded roles - dynamically planned based on objective
- Supports domains like web-development, data-analysis, documentation, etc.
- **User Confirmation Workflow**: After LLM planning, waits for user confirmation before execution
  - Displays planned roles and tasks to user
  - Allows user to modify role assignments or request replanning
  - Supports backend selection (OpenAI/DeepSeek) for each role

## Session Management

- Each AI agent gets dedicated `ConversationId`
- Auto-save during execution
- Session recovery capabilities

## Token Optimization

- Local information sharing between agents
- Web search preference before human referral
- Cached data reuse

## Implementation Details

### File Structure

```
core/src/multi_agent/
├── mod.rs                  # Module exports and structure
├── casual.rs               # Casual multi-agent collaboration system
├── agents.rs               # Dynamic role planning and AgentManager
├── communication.rs        # Inter-agent communication
└── task_planner.rs         # Task planning system (future integration)
```

### Key Integration Points

- `ConversationManager` for session persistence
- `RolloutRecorder` for auto-save functionality
- `AgentManager` for dynamic role planning using standard role taxonomy
- Model provider system for agent execution
- Tool calling infrastructure for web search

## Performance Considerations

### Scalability

- Memory usage scales with number of active agents
- Session persistence optimized for large projects
- Efficient coordination minimizes overhead

### Token Efficiency

- Local information sharing reduces LLM calls
- Cached data reuse minimizes redundant requests
- Web search preference reduces human referral costs

### Session Recovery

- Individual agent session persistence
- Main task session coordination
- Resume capability for interrupted collaborations

## Testing Strategy

### Unit Tests

- Agent profile creation and validation
- Task decomposition logic
- Agent assignment algorithms

### Integration Tests

- Multi-agent collaboration scenarios
- Session persistence and recovery
- Cross-agent communication

### End-to-End Tests

- Complete multi-agent workflows
- Real-world use cases
- Error handling and recovery scenarios
