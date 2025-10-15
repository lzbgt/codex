# Codex CLI (Rust Implementation)

We provide Codex CLI as a standalone, native executable to ensure a zero-dependency install.

## Installing Codex

Today, the easiest way to install Codex is via `npm`:

```shell
npm i -g @openai/codex
codex
```

You can also install via Homebrew (`brew install codex`) or download a platform-specific release directly from our [GitHub Releases](https://github.com/openai/codex/releases).

## What's new in the Rust CLI

The Rust implementation is now the maintained Codex CLI and serves as the default experience. It includes a number of features that the legacy TypeScript CLI never supported.

### Config

Codex supports a rich set of configuration options. Note that the Rust CLI uses `config.toml` instead of `config.json`. See [`docs/config.md`](../docs/config.md) for details.

### Model Context Protocol Support

#### MCP client

Codex CLI functions as an MCP client that allows the Codex CLI and IDE extension to connect to MCP servers on startup. See the [`configuration documentation`](../docs/config.md#mcp_servers) for details.

#### MCP server (experimental)

Codex can be launched as an MCP _server_ by running `codex mcp-server`. This allows _other_ MCP clients to use Codex as a tool for another agent.

Use the [`@modelcontextprotocol/inspector`](https://github.com/modelcontextprotocol/inspector) to try it out:

```shell
npx @modelcontextprotocol/inspector codex mcp-server
```

Use `codex mcp` to add/list/get/remove MCP server launchers defined in `config.toml`, and `codex mcp-server` to run the MCP server directly.

### Notifications

You can enable notifications by configuring a script that is run whenever the agent finishes a turn. The [notify documentation](../docs/config.md#notify) includes a detailed example that explains how to get desktop notifications via [terminal-notifier](https://github.com/julienXX/terminal-notifier) on macOS.

### `codex exec` to run Codex programmatically/non-interactively

To run Codex non-interactively, run `codex exec PROMPT` (you can also pass the prompt via `stdin`) and Codex will work on your task until it decides that it is done and exits. Output is printed to the terminal directly. You can set the `RUST_LOG` environment variable to see more about what's going on.

### Use `@` for file search

Typing `@` triggers a fuzzy-filename search over the workspace root. Use up/down to select among the results and Tab or Enter to replace the `@` with the selected path. You can use Esc to cancel the search.

### Esc–Esc to edit a previous message

When the chat composer is empty, press Esc to prime “backtrack” mode. Press Esc again to open a transcript preview highlighting the last user message; press Esc repeatedly to step to older user messages. Press Enter to confirm and Codex will fork the conversation from that point, trim the visible transcript accordingly, and pre‑fill the composer with the selected user message so you can edit and resubmit it.

In the transcript preview, the footer shows an `Esc edit prev` hint while editing is active.

### `--cd`/`-C` flag

Sometimes it is not convenient to `cd` to the directory you want Codex to use as the "working root" before running Codex. Fortunately, `codex` supports a `--cd` option so you can specify whatever folder you want. You can confirm that Codex is honoring `--cd` by double-checking the **workdir** it reports in the TUI at the start of a new session.

### Shell completions

Generate shell completion scripts via:

```shell
codex completion bash
codex completion zsh
codex completion fish
```

### Experimenting with the Codex Sandbox

To test to see what happens when a command is run under the sandbox provided by Codex, we provide the following subcommands in Codex CLI:

```
# macOS
codex sandbox macos [--full-auto] [COMMAND]...

# Linux
codex sandbox linux [--full-auto] [COMMAND]...

# Legacy aliases
codex debug seatbelt [--full-auto] [COMMAND]...
codex debug landlock [--full-auto] [COMMAND]...
```

### Selecting a sandbox policy via `--sandbox`

The Rust CLI exposes a dedicated `--sandbox` (`-s`) flag that lets you pick the sandbox policy **without** having to reach for the generic `-c/--config` option:

```shell
# Run Codex with the default, read-only sandbox
codex --sandbox read-only

# Allow the agent to write within the current workspace while still blocking network access
codex --sandbox workspace-write

# Danger! Disable sandboxing entirely (only do this if you are already running in a container or other isolated env)
codex --sandbox danger-full-access
```

The same setting can be persisted in `~/.codex/config.toml` via the top-level `sandbox_mode = "MODE"` key, e.g. `sandbox_mode = "workspace-write"`.

## Model Providers

Codex supports multiple AI model providers including OpenAI and DeepSeek. You can configure providers in your `~/.codex/config.toml`:

```toml
[model_providers.openai]
name = "OpenAI"
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"

[model_providers.deepseek]
name = "DeepSeek"
base_url = "https://api.deepseek.com"
env_key = "DEEPSEEK_API_KEY"
```

Set your API keys as environment variables:
```bash
export OPENAI_API_KEY="your-openai-key"
export DEEPSEEK_API_KEY="your-deepseek-key"
```

### Using Different Model Providers

You can specify which model provider to use:

```bash
# Use OpenAI models
codex --provider openai

# Use DeepSeek models
codex --provider deepseek

# Use specific model
codex --model gpt-4o
codex --model deepseek-reasoner
```

## Multi-Agent Task Collaboration

Codex includes two powerful multi-agent systems for different collaboration needs:

### 1. Casual Multi-Agent Collaboration (Recommended)

The casual multi-agent system provides dynamic agent team creation with casual human engagement. No pre-configuration required - just provide your objective and the system handles everything.

#### Basic Usage

```bash
# Start a casual multi-agent task
codex casual --objective "Build a React frontend with Node.js backend for a todo app"

# Monitor progress in real-time
codex casual --objective "Create a full-stack web application" --monitor

# Interactive mode for casual human engagement
codex casual --objective "Develop a machine learning pipeline" --interactive
```

#### Key Features

- **Dynamic Agent Creation**: LLM analyzes your objective and creates the perfect team
- **Web Search Integration**: Agents prefer web search for research before human referral
- **Session Persistence**: Dedicated session IDs for each agent with auto-save
- **Token Optimization**: Local information sharing and cached data reuse
- **Casual Human Engagement**: Drop in/out anytime without formal joining

#### Example Output

```
🚀 Starting casual multi-agent collaboration
Objective: Build a React frontend with Node.js backend for a todo app
Mode: Background

✅ Task published successfully!
Task ID: 0dd1e380-d1a2-48d8-a428-db1fb17aa4f2
Status: Planning phase...

📊 Progress Update:
  Status: InProgress
  Progress: 50%
  Active Agents: ["Frontend Developer", "Backend Developer", "Project Coordinator"]
  Recent Activity: system: Dynamic agent team created with 4 roles. Starting collaboration...
  Human Attention Needed: false

💬 Recent Messages:
  frontend-developer-1: Need information about best practices. Using web search to research...
  frontend-developer-1: Found relevant information via web search. Continuing with implementation...
  backend-developer-2: Blocked: Need API credentials for external service. Web search cannot provide this.
```

#### Casual Human Engagement

```bash
# Quick progress peek anytime
codex casual --monitor --task-id 0dd1e380-d1a2-48d8-a428-db1fb17aa4f2

# Interactive engagement when human attention is needed
codex casual --interactive --task-id 0dd1e380-d1a2-48d8-a428-db1fb17aa4f2
```

### 2. Traditional Multi-Agent Orchestration

For users who prefer explicit agent configuration, the traditional multi-agent system allows you to define specific agent profiles.

#### Creating Agent Configurations

Define agents in a TOML file (e.g., `agents.toml`):

```toml
[[agents]]
name = "backend-dev"
role = "Backend Developer"
capabilities = ["python", "fastapi", "backend", "api-design"]
model_provider = "deepseek"
model = "deepseek-reasoner"
instructions = "You are a backend developer focused on Python and FastAPI development."

[[agents]]
name = "frontend-dev"
role = "Frontend Developer"
capabilities = ["javascript", "react", "ui", "frontend"]
model_provider = "openai"
model = "gpt-4o"
instructions = "You are a frontend developer focused on React and JavaScript."

[[agents]]
name = "coordinator"
role = "Project Coordinator"
capabilities = ["planning", "coordination", "project-management"]
model_provider = "deepseek"
model = "deepseek-reasoner"
instructions = "You coordinate between different agents and manage task dependencies."
```

#### Running Multi-Agent Tasks

```bash
# Run multi-agent collaboration
codex multi-agent --agents agents.toml --objective "Create a web application for task management"

# With custom configuration
codex multi-agent --config ~/.codex/config.toml --agents agents.toml --max-turns 10
```

The system will:
1. **Decompose** the objective into subtasks based on agent capabilities
2. **Assign** tasks to the most suitable agents
3. **Execute** tasks using agent-specific model providers
4. **Coordinate** dependencies between tasks
5. **Share** context and artifacts between agents

#### Example Output

```
Multi-agent system initialized with objective: Create a web application for task management
Available agents: ["backend-dev", "frontend-dev", "coordinator"]
Task plan created with 3 subtasks
  - planning: Plan overall approach and requirements...
  - backend_implementation: Implement backend components...
  - frontend_implementation: Implement frontend components...
Agent assignments: {"backend-dev": ["backend_implementation"], "frontend-dev": ["frontend_implementation"], "coordinator": ["planning"]}

=== Turn 1 ===
Assigning task 'planning' to agent 'coordinator'
Task 'planning' completed by agent 'coordinator'

=== Turn 2 ===
Assigning task 'backend_implementation' to agent 'backend-dev'
Assigning task 'frontend_implementation' to agent 'frontend-dev'
Task 'backend_implementation' completed by agent 'backend-dev'
Task 'frontend_implementation' completed by agent 'frontend-dev'

Execution completed after 2 turns
Tasks completed: 3/3
```

## Normal Task Usage

For regular single-agent tasks, Codex provides several usage patterns:

### Interactive Mode

```bash
# Start interactive session
codex

# With specific provider
codex --provider deepseek

# With custom working directory
codex --cd /path/to/project
```

### Non-Interactive Mode

```bash
# Execute a single task
codex exec "Write a Python function to calculate fibonacci numbers"

# Pipe input
echo "Create a React component for a login form" | codex exec

# With specific model
codex exec --model deepseek-reasoner "Analyze this code for performance issues"
```

### File Operations

```bash
# Search for files with @
codex
# Then type: @component

# Edit previous messages with Esc-Esc
codex
# Press Esc twice to edit previous user messages
```

## Code Organization

This folder is the root of a Cargo workspace. It contains quite a bit of experimental code, but here are the key crates:

- [`core/`](./core) contains the business logic for Codex. Ultimately, we hope this to be a library crate that is generally useful for building other Rust/native applications that use Codex.
- [`exec/`](./exec) "headless" CLI for use in automation.
- [`tui/`](./tui) CLI that launches a fullscreen TUI built with [Ratatui](https://ratatui.rs/).
- [`cli/`](./cli) CLI multitool that provides the aforementioned CLIs via subcommands.
