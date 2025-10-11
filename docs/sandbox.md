## Sandbox & approvals

### Approval modes

We've chosen a powerful default for how Codex works on your computer: `Full Access`. In this approval mode (`--sandbox danger-full-access` + `--ask-for-approval never`), Codex can read files, make edits, run commands, and access the network without prompting.

When you just want to chat, or if you want to plan before diving in, you can switch to `Read Only` mode with the `/approvals` command.

If you prefer automatic execution inside the repository but still want Codex to ask before leaving the workspace or taking riskier actions, use the `Auto` preset (`--sandbox workspace-write --ask-for-approval on-request`).

#### Defaults and recommendations

- Codex starts in **Full Access** by default: no sandbox restrictions and no approval prompts. Downgrade to `Auto` or `Read Only` if you want more guardrails.
- On launch, Codex detects whether the folder is version-controlled and recommends:
  - Version-controlled folders: `Auto` (workspace write + on-request approvals)
  - Non-version-controlled folders: `Read Only`
- The workspace includes the current directory and temporary directories like `/tmp`. Use the `/status` command to see which directories are in the workspace.
- You can set these explicitly:
  - `codex --sandbox workspace-write --ask-for-approval on-request`
  - `codex --sandbox read-only --ask-for-approval on-request`

### Can I run without ANY approvals?

Yes, you can disable all approval prompts with `--ask-for-approval never`. This option works with all `--sandbox` modes, so you still have full control over Codex's level of autonomy. It will make its best attempt with whatever constraints you provide.

### Common sandbox + approvals combinations

| Intent                             | Flags                                                                                       | Effect                                                                                                                                                |
| ---------------------------------- | ------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Safe read-only browsing            | `--sandbox read-only --ask-for-approval on-request`                                         | Codex can read files and answer questions. Codex requires approval to make edits, run commands, or access network.                                    |
| Read-only non-interactive (CI)     | `--sandbox read-only --ask-for-approval never`                                              | Reads only; never escalates                                                                                                                           |
| Let it edit the repo, ask if risky | `--sandbox workspace-write --ask-for-approval on-request`                                   | Codex can read files, make edits, run commands, and access the network in the workspace. Codex requires approval for actions outside the workspace.   |
| Auto (preset)                      | `--full-auto` (equivalent to `--sandbox workspace-write` + `--ask-for-approval on-failure`) | Codex can read files, make edits, and run commands in the workspace. Codex asks for approval only when a sandboxed command fails and needs escalation. |
| Full access (default)              | _No flags required_                                                                        | No sandbox; no prompts                                                                                                                                |
| YOLO (explicit)                    | `--dangerously-bypass-approvals-and-sandbox` (alias: `--yolo`)                              | Same as default; useful when you need to ensure overrides stick even if profiles are configured.                                                      |

#### Fine-tuning in `config.toml`

```toml
# approval mode
approval_policy = "untrusted"
sandbox_mode    = "read-only"

# full-auto mode
approval_policy = "on-request"
sandbox_mode    = "workspace-write"

# Optional: disable network in workspace-write mode
[sandbox_workspace_write]
network_access = false
```

You can also save presets as **profiles**:

```toml
[profiles.full_auto]
approval_policy = "on-request"
sandbox_mode    = "workspace-write"

[profiles.readonly_quiet]
approval_policy = "never"
sandbox_mode    = "read-only"
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

### Platform sandboxing details

The mechanism Codex uses to implement the sandbox policy depends on your OS:

- **macOS 12+** uses **Apple Seatbelt** and runs commands using `sandbox-exec` with a profile (`-p`) that corresponds to the `--sandbox` that was specified.
- **Linux** uses a combination of Landlock/seccomp APIs to enforce the `sandbox` configuration.

Note that when running Linux in a containerized environment such as Docker, sandboxing may not work if the host/container configuration does not support the necessary Landlock/seccomp APIs. In such cases, we recommend configuring your Docker container so that it provides the sandbox guarantees you are looking for and then running `codex` with `--sandbox danger-full-access` (or, more simply, the `--dangerously-bypass-approvals-and-sandbox` flag) within your container.
