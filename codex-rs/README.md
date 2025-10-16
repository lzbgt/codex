# Codex CLI (Rust Implementation)

We provide Codex CLI as a standalone, native executable to ensure a zero-dependency install.

## Installing Codex

Today, the easiest way to install Codex is via `npm`:

```shell
npm i -g @openai/codex
codex
```

You can also install via Homebrew (`brew install codex`) or download a platform-specific release directly from our [GitHub Releases](https://github.com/openai/codex/releases).

### Building the CLI from source

If you are iterating inside this repository, build the native CLI directly:

```bash
cd codex-rs
cargo build -p codex-cli --release

# Run the freshly built binary
./target/release/codex --help
```

The build produces the `codex` binary under `codex-rs/target/release/`. Use that path in the examples below when testing locally.

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

You can steer Codex toward a specific provider by passing configuration overrides (they apply to the base CLI and every subcommand):

```bash
# Use the defaults (OpenAI responses API)
codex

# Target DeepSeek's chat-completions API
codex -c model_provider=deepseek -c model=deepseek-reasoner

# Run the multi-agent prototype against DeepSeek
codex -c model_provider=deepseek -c model=deepseek-reasoner \
  multi-agent --objective "Explore the repository layout"
```

The same overrides can be baked into `~/.codex/config.toml` under `[model_providers]` if you prefer permanent settings.

## Multi-Agent Task Collaboration

Codex includes two powerful multi-agent systems for different collaboration needs:

### Casual Multi-Agent Collaboration (Experimental)

The experimental casual multi-agent system launches a small team of AI agents for a single objective. It currently streams text output only—tool execution, dependency-aware scheduling, and artifact sharing are still under active development.

#### Basic Usage

```bash
# Launch with a prompted objective (monitor mode by default)
codex multi-agent

# Headless run with an explicit objective
codex multi-agent --objective "Build a React frontend with Node.js backend for a todo app"

# Stream progress in the terminal (default behaviour)
codex multi-agent --objective "Create a full-stack web application"

# Allow ad-hoc human guidance when the system requests attention
codex multi-agent --objective "Develop a machine learning pipeline" --interactive

When running from a local build, replace `codex` with `./target/release/codex` (or the full path to your binary). Configure providers inline with `-c` overrides, e.g.:

```bash
export DEEPSEEK_API_KEY="sk-your-deepseek-key"
./target/release/codex \
  -c model_provider=deepseek \
  -c model=deepseek-reasoner \
  multi-agent --objective "Refine the repository README" --monitor
```

Every run is also logged to `<cwd>/.codex-logs/multi-agent-*.log`, making it easy to review what happened afterwards.

#### Current Capabilities

- **Dynamic Role Planning**: Agent roles and task breakdowns come from the configured LLM provider (OpenAI or DeepSeek) with heuristic fallback.
- **Session Persistence Hooks**: Each agent gets its own transcript and rollout path; manual resume APIs exist but still need polish.
- **Monitoring Loop (beta)**: The `--monitor` flag prints periodic progress snapshots and reaches 100 % once every AI role finishes its tasks.

#### Known Limitations

- Apply-patch, file writes, and web search remain disabled—only the standard `shell` tool is wired up today.
- Tool output is summarized for the activity feed; full transcripts still live in the rollout logs.

### Future Work

Config-driven multi-agent orchestration (`agents.toml`, custom schedulers, richer tooling) is still on the roadmap. The previous documentation for that flow has been removed until the implementation is available—follow `docs/multi-agent/PLANNING.md` for updates.

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
