# Starship Passthrough

KeyJey lets you embed any [Starship](https://starship.rs) module directly in your statusline layout, right next to native KeyJey modules.

## How It Works

Any token in a `lines` format string that doesn't start with `keyjey.` is treated as a Starship module name. KeyJey invokes `starship module <name>` as a subprocess, captures its stdout, and splices the output into your statusline.

```toml
[keyjey]
lines = [
  "$directory $git_branch $git_status",
  "$keyjey.model  $keyjey.cost  $keyjey.context_bar",
]
```

In the example above, `$directory`, `$git_branch`, and `$git_status` are Starship passthrough modules. `$keyjey.model`, `$keyjey.cost`, and `$keyjey.context_bar` are native KeyJey modules.

**Prerequisite:** [Starship](https://starship.rs) must be installed and on your `$PATH`.

## KEYJEY_* Environment Variables

Before each Starship subprocess call, keyjey sets the following environment variables so your Starship modules can access Claude Code session data:

| Variable | Type | Example | Description |
|----------|------|---------|-------------|
| `KEYJEY_MODEL` | string | `claude-sonnet-4-5` | Active model display name |
| `KEYJEY_MODEL_ID` | string | `claude-sonnet-4-5-20251022` | Full model identifier |
| `KEYJEY_COST_USD` | float | `1.234` | Session cost in USD |
| `KEYJEY_CONTEXT_PCT` | float | `42.5` | Context window used (%) |
| `KEYJEY_CONTEXT_REMAINING_PCT` | float | `57.5` | Context window remaining (%) |
| `KEYJEY_VIM_MODE` | string | `NORMAL` | Current vim mode (empty if inactive) |
| `KEYJEY_AGENT_NAME` | string | `claude-code` | Active agent name (empty if none) |
| `KEYJEY_SESSION_ID` | string | `abc123...` | Session UUID |
| `KEYJEY_CWD` | string | `/home/user/project` | Current working directory |

These variables are available inside any custom Starship module you write. For example, you could create a Starship module that changes colour based on `KEYJEY_COST_USD`.


## Cache Behaviour

Passthrough module output is cached for **5 seconds per session** to avoid spawning a new Starship subprocess on every statusline render.

Cache path: `{dirname(transcript_path)}/keyjey/{transcript_stem}-starship-{module_name}`

The cache is keyed by session transcript path, so different Claude Code sessions maintain independent caches. The cache directory is created automatically if missing.

## Process Details

- The subprocess runs with the working directory set to `workspace.current_dir`
- Stderr from the Starship subprocess is discarded
- If Starship is not found or the subprocess fails, the module renders empty (silent failure — no error shown in the statusline)
- The first call in a session may take a moment; subsequent calls within 5s use the cache

## Example: Mixed Native + Passthrough Config

```toml
[keyjey]
lines = [
  """
  $directory\
  $git_branch\
  $git_status\
  $python\
  $nodejs\
  $rust
  """,
  "$keyjey.model $keyjey.agent",
  "$keyjey.context_bar $keyjey.cost $keyjey.usage_limits",
]

[keyjey.model]
symbol = "🤖 "
style  = "bold fg:#7aa2f7"

[keyjey.agent]
symbol = "↳ "
style  = "fg:#9ece6a"

[keyjey.context_bar]
width              = 10
style              = "fg:#7dcfff"
warn_threshold     = 40.0
warn_style         = "fg:#e0af68"
critical_threshold = 70.0
critical_style     = "bold fg:#f7768e"

[keyjey.cost]
style              = "fg:#a9b1d6"
warn_threshold     = 2.0
warn_style         = "fg:#e0af68"
critical_threshold = 5.0
critical_style     = "bold fg:#f7768e"
```

The first row uses a multi-line TOML string with `\` line continuations to combine several Starship passthrough modules without spaces between them (Starship handles its own spacing). The second and third rows are pure native keyjey modules.

## Caveats

- Starship must be installed. The KeyJey curl installer can optionally install it for you.
- Passthrough adds a subprocess call overhead. The 5s cache keeps this negligible after the first render.
- `KEYJEY_*` variables reflect the values at render time. Starship modules that consume them will update every 5s (cache TTL).
