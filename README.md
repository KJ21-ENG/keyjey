# ⚓ KeyJey

[![CI](https://img.shields.io/github/actions/workflow/status/KJ21-ENG/keyjey/ci.yml?branch=main&label=CI&style=flat-square)](https://github.com/KJ21-ENG/keyjey/actions/workflows/ci.yml)
[![GitHub release](https://img.shields.io/github/v/release/KJ21-ENG/keyjey?style=flat-square)](https://github.com/KJ21-ENG/keyjey/releases/latest)
[![GitHub downloads](https://img.shields.io/github/downloads/KJ21-ENG/keyjey/total?label=github%20downloads&style=flat-square)](https://github.com/KJ21-ENG/keyjey/releases)
[![License](https://img.shields.io/github/license/KJ21-ENG/keyjey?style=flat-square)](https://github.com/KJ21-ENG/keyjey/blob/main/LICENSE)

**Beautiful, Blazing-fast, Customizable Claude Code Statusline with Codex CLI setup support.**


`keyjey` renders a live statusline for [Claude Code](https://claude.ai/code) sessions, showing session cost, context window usage, model name, API usage limits, and more — all configurable via a simple TOML file. It can also configure [Codex CLI](https://developers.openai.com/codex/cli/) to use Codex's native `tui.status_line` footer with a practical model/context/git/limit layout.

### Key features:
- 🎨 Fully Customizable: Configure every module with Starship-compatible TOML. Colors, symbols, thresholds — your statusline, your rules.
- ⚡ Blazing Fast: Written in Rust with a ≤10ms render budget.
- 🔌 Starship Passthrough: Embed any [Starship](https://starship.rs) module (git_branch, directory, language runtimes) right next to native KeyJey modules.
- 💰 Session Insights: Track cost, context window usage, API limits, vim mode, agent name, and more — all from Claude Code's live JSON feed. Implement custom warn and critical thresholds with custom colors for each. 
- 🧭 Codex CLI Setup: Auto-detect Codex CLI during global npm install and configure its native `tui.status_line` when no Codex status line exists yet.

## 🚀 Install

### ⚡ Method 1: npm / npx

```sh
npm i -g keyjey
```

Or run it without a global install:

```sh
npx keyjey --help
```

`npm`/`npx` installs a small launcher that downloads the correct prebuilt Rust binary for your platform from GitHub Releases.

When installed globally with `npm i -g keyjey`, the package also:

- creates `~/.config/keyjey.toml` if it does not exist
- wires `~/.claude/settings.json` to use `keyjey-remaining` if no status line is configured yet
- detects Codex CLI and adds a native `tui.status_line` to `~/.codex/config.toml` if Codex has no status line configured yet
- installs both `keyjey` and `keyjey-remaining` commands

You can rerun setup later without reinstalling:

```sh
keyjey setup
```

KeyJey preserves existing Codex footer preferences by default. To intentionally apply a KeyJey-managed Codex preset, use:

```sh
keyjey setup --codex-preset rich --codex-force
```

Available Codex presets are `rich`, `compact`, `minimal`, and `off`. `--codex-force` creates a timestamped backup before replacing an existing `tui.status_line`.

For Claude Code, wire `~/.claude/settings.json`:

```json
{
  "statusLine": { "type": "command", "command": "keyjey" }
}
```

For Codex CLI, KeyJey uses Codex's supported native footer configuration instead of rendering custom text inside Codex:

```toml
[tui]
status_line = [
  "model-with-reasoning",
  "fast-mode",
  "current-dir",
  "git-branch",
  "context-remaining",
  "five-hour-limit",
  "weekly-limit",
]
```

### 📦 Method 2: Build from source

Requires the Rust toolchain.

```sh
git clone https://github.com/KJ21-ENG/keyjey.git
cd keyjey
cargo build --release
install -m 0755 ./target/release/keyjey ~/.local/bin/keyjey
```

Then wire the statusline manually in `~/.claude/settings.json`:

```json
{
  "statusLine": { "type": "command", "command": "keyjey" }
}
```

Native Windows is not supported yet. For Windows machines, use WSL2 and install the Linux package inside WSL.

## ⚙️ Configuration

- The default config file is `~/.config/keyjey.toml`.
- You can also place a `keyjey.toml` in your project root for per-project overrides.
- Use `keyjey.toml` and `$keyjey.*` tokens for all configuration.
- The `lines` array defines the rows of your statusline. 
- Each element is a format string mixing `$keyjey.<module>` tokens (native keyjey modules) with Starship module tokens (e.g. `$git_branch`).

A minimal working example:

```toml
[keyjey]
lines = ["$keyjey.model $keyjey.cost $keyjey.context_bar"]
```

### 🎨 Styling example

```toml
[keyjey]
lines = ["$keyjey.model $keyjey.cost $keyjey.context_bar"]

[keyjey.cost]
warn_threshold = 1.0
warn_style = "bold yellow"
critical_threshold = 5.0
critical_style = "bold red"
```

### 🧩 Available modules

Everything in the [Claude Code status line documentation](https://code.claude.com/docs/en/statusline#available-data) is available as a `$keyjey.<module>` token for you to mix and match in the `lines` format strings. Here are the most popular ones:

| Token | Description |
|-------|-------------|
| `$keyjey.model` | Claude model name |
| `$keyjey.cost` | Session cost in USD ($X.XX) |
| `$keyjey.context_bar` | Visual progress bar of context window usage |
| `$keyjey.context_window` | Context window tokens (used/total) |
| `$keyjey.usage_limits` | API usage limits (5hr / 7-day) |
| `$keyjey.session_name` | Session name derived from the transcript's first user message |
| `$keyjey.reasoning` | Claude Code reasoning effort from `.claude/settings.json` |
| `$keyjey.agent` | Sub-agent name |
| `$keyjey.session` | Session identity info |
| `$keyjey.workspace` | Workspace/project directory |

### Terminal width

KeyJey truncates each rendered statusline row to the detected terminal width by default. This prevents narrow terminals from wrapping the statusline into the prompt area. You can override or disable it:

```toml
[keyjey]
truncate = true
max_width = 80
```

Set `truncate = false` to restore the old unbounded behavior.

### Usage limits cache note

`$keyjey.usage_limits` uses a shared OS cache at `keyjey/usage-limits.json`, so sibling Claude Code windows can reuse fresh usage data instead of making duplicate HTTP calls. An idle Claude Code window still cannot update by itself; it will show stale text until Claude Code invokes KeyJey in that window again. The shared cache only ensures the next render sees data fetched by another window.

Full configuration reference: **https://github.com/KJ21-ENG/keyjey**

## 🔍 Debugging

Run `keyjey explain` to inspect what keyjey sees from Claude Code's context JSON — useful when a module shows nothing or behaves unexpectedly.

```sh
keyjey explain
```

## ✨ Showcase

Six ready-to-use configurations — from minimal to full-featured. Each can be dropped into `~/.config/keyjey.toml`.

---

### 1. 🪶 Minimal

One clean row. Model, cost with colour thresholds, context bar.


<details>
<summary>View config</summary>

```toml
[keyjey]
lines = ["$keyjey.model  $keyjey.cost  $keyjey.context_bar"]

[keyjey.cost]
style              = "green"
warn_threshold     = 2.0
warn_style         = "yellow"
critical_threshold = 5.0
critical_style     = "bold red"

[keyjey.context_bar]
width              = 10
warn_threshold     = 40.0
warn_style         = "yellow"
critical_threshold = 70.0
critical_style     = "bold red"
```

</details>

---

### 2. 🌿 Git-Aware Developer

Two rows: Starship git status on top, Claude session below. Starship passthrough (`$directory`, `$git_branch`, `$git_status`) requires [Starship](https://starship.rs) to be installed.


<details>
<summary>View config</summary>

```toml
[keyjey]
lines = [
  "$directory $git_branch $git_status",
  "$keyjey.model  $keyjey.cost  $keyjey.context_bar",
]

[keyjey.model]
symbol = "🤖 "
style  = "bold cyan"

[keyjey.cost]
warn_threshold     = 2.0
warn_style         = "yellow"
critical_threshold = 5.0
critical_style     = "bold red"

[keyjey.context_bar]
width              = 10
warn_threshold     = 40.0
warn_style         = "yellow"
critical_threshold = 70.0
critical_style     = "bold red"
```

</details>

---

### 3. 💰 Cost Guardian

Shows cost, lines changed, and rolling API usage limits all at once. Colour escalates as budgets fill.


<details>
<summary>View config</summary>

```toml
[keyjey]
lines = [
  "$keyjey.model $keyjey.cost +$keyjey.cost.total_lines_added -$keyjey.cost.total_lines_removed",
  "$keyjey.context_bar $keyjey.usage_limits",
]

[keyjey.model]
style = "bold purple"

[keyjey.cost]
warn_threshold     = 1.0
warn_style         = "bold yellow"
critical_threshold = 3.0
critical_style     = "bold red"

[keyjey.context_bar]
width              = 10
warn_threshold     = 40.0
warn_style         = "yellow"
critical_threshold = 70.0
critical_style     = "bold red"

[keyjey.usage_limits]
ttl                = 60        # cache TTL in seconds; increase if running many concurrent sessions
five_hour_format   = "5h {pct}%"
seven_day_format   = "7d {pct}%"
separator          = " "
warn_threshold     = 70.0
warn_style         = "bold yellow"
critical_threshold = 90.0
critical_style     = "bold red"
```

</details>

---

### 4. 🎨 Material Hex

Every style value is a `fg:#rrggbb` hex colour — no named colours anywhere. Amber warns, coral criticals.


<details>
<summary>View config</summary>

```toml
[keyjey]
lines = [
  "$keyjey.model $keyjey.cost",
  "$keyjey.context_bar $keyjey.usage_limits",
]

[keyjey.model]
style = "fg:#c3e88d"

[keyjey.cost]
style              = "fg:#82aaff"
warn_threshold     = 2.0
warn_style         = "fg:#ffcb6b"
critical_threshold = 6.0
critical_style     = "bold fg:#f07178"

[keyjey.context_bar]
width              = 10
style              = "fg:#89ddff"
warn_threshold     = 40.0
warn_style         = "fg:#ffcb6b"
critical_threshold = 70.0
critical_style     = "bold fg:#f07178"

[keyjey.usage_limits]
five_hour_format   = "5h {pct}%"
seven_day_format   = "7d {pct}%"
separator          = " "
warn_threshold     = 70.0
warn_style         = "fg:#ffcb6b"
critical_threshold = 90.0
critical_style     = "bold fg:#f07178"
```

</details>

---

### 5. 🌃 Tokyo Night

Three-row layout for polyglot developers. Starship handles language runtimes and git; keyjey handles session data. Styled with the [Tokyo Night](https://github.com/folke/tokyonight.nvim) colour palette.


<details>
<summary>View config</summary>

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
symbol             = "💰 "
style              = "fg:#a9b1d6"
warn_threshold     = 2.0
warn_style         = "fg:#e0af68"
critical_threshold = 5.0
critical_style     = "bold fg:#f7768e"

[keyjey.usage_limits]
five_hour_format   = "⌛ 5h {pct}%"
seven_day_format   = "📅 7d {pct}%"
separator          = " "
warn_threshold     = 70.0
warn_style         = "fg:#e0af68"
critical_threshold = 90.0
critical_style     = "bold fg:#f7768e"
```

</details>

---

### 6. 🔤 Nerd Fonts

Requires a [Nerd Font](https://www.nerdfonts.com) in your terminal. Icons are embedded as `symbol` values on each module and as literal characters in the format string for Starship passthrough rows. You can use `format` to control how the symbol and value are combined for each module exactly like you'd do with Starship.


<details>
<summary>View config</summary>

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
  "$keyjey.model $keyjey.cost $keyjey.context_bar $keyjey.usage_limits",
]

[keyjey.model]
symbol = " "
style  = "bold fg:#7aa2f7"

[keyjey.cost]
symbol             = "💰 "
style              = "fg:#a9b1d6"
warn_threshold     = 2.0
warn_style         = "fg:#e0af68"
critical_threshold = 5.0
critical_style     = "bold fg:#f7768e"

[keyjey.context_bar]
symbol             = " "
format             = "[$symbol$value]($style)"
width              = 10
style              = "fg:#7dcfff"
warn_threshold     = 40.0
warn_style         = "fg:#e0af68"
critical_threshold = 70.0
critical_style     = "bold fg:#f7768e"

[keyjey.usage_limits]
five_hour_format   = "⌛ 5h {pct}%"
seven_day_format   = "📅 7d {pct}%"
separator          = " "
warn_threshold     = 70.0
warn_style         = "fg:#e0af68"
critical_threshold = 90.0
critical_style     = "bold fg:#f7768e"
```

</details>

---

## 📚 Full documentation

→ **[keyjey.dev](https://github.com/KJ21-ENG/keyjey)**

Complete configuration reference, format string syntax, all module options, and examples.

---

If you found this project useful, please give us a star ⭐ on [GitHub](https://github.com/KJ21-ENG/keyjey)!

If you find bugs or have suggestions, open an issue or submit a pull request. Contributions are very welcome!

## 💡 Inspiration
- Inspired by [starship](https://starship.rs), built with Rust and the [Claude Code status line API](https://code.claude.com/docs/en/statusline).

## 📄 License

Apache-2.0
