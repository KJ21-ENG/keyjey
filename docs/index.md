---
layout: home

hero:
  name: "⚓ KeyJey"
  text: ""
  tagline: Bring Starship's power to Claude Code Status Line, with native Codex CLI footer setup.
  actions:
    - theme: brand
      text: Get Started
      link: /#install-curl
    - theme: alt
      text: Configure
      link: /configuration

features:
  - icon: 🎨
    title: Fully Customizable
    details: Configure every module with Starship-compatible TOML. Colors, symbols, thresholds — your statusline, your rules.
  - icon: ⚡
    title: Blazing Fast
    details: Written in Rust with a ≤10ms render budget.
  - icon: 🔌
    title: Starship Passthrough
    details: Embed any Starship module (git_branch, directory, language runtimes) right next to native KeyJey modules.
  - icon: 💰
    title: Session Insights
    details: Track cost, context window usage, API limits, vim mode, agent name, and more — all from Claude Code's live JSON feed.
  - icon: 🧭
    title: Codex CLI Setup
    details: Auto-detect Codex CLI and configure its native footer status line when no Codex status line exists yet.
---

## What is KeyJey?

`keyjey` renders a live statusline for [Claude Code](https://claude.ai/code) sessions and can configure [Codex CLI](https://developers.openai.com/codex/cli/) to use Codex's native status-line footer.

It reads Claude Code's session JSON from stdin and renders styled text using a simple TOML config file — the same format as [Starship](https://starship.rs).

If you've already invested in Starship customization, KeyJey slots right in: add `[keyjey.*]` sections to your existing `starship.toml` (or use a dedicated `~/.config/keyjey.toml`), reference native KeyJey modules alongside any Starship module, and get a unified statusline that speaks both languages.

Codex CLI does not expose Claude Code's external statusline renderer hook, so KeyJey configures Codex's supported `tui.status_line` setting instead of rendering arbitrary custom text inside Codex.

## Install {#install-curl}

### Quick Install (recommended)

```sh
curl -fsSL https://keyjey.dev/install.sh | bash
```

Auto-detects your OS and architecture (macOS arm64/x86_64, Linux x86_64/aarch64), downloads the binary to `~/.local/bin/keyjey`, creates a starter config at `~/.config/keyjey.toml`, wires the `statusLine` entry in `~/.claude/settings.json`, configures Codex CLI's native `tui.status_line` when Codex is detected and unconfigured, and optionally installs [Starship](https://starship.rs) and `libsecret-tools` (Linux only, needed for usage limits).

You can rerun setup later with:

```sh
keyjey setup
```

To intentionally replace an existing Codex footer with a KeyJey-managed preset:

```sh
keyjey setup --codex-preset rich --codex-force
```

Available Codex presets are `rich`, `compact`, `minimal`, and `off`. The `off` preset writes an empty Codex status-line list. KeyJey creates a timestamped backup before replacing an existing Codex `tui.status_line`.

### Cargo Install {#install-cargo}

Requires the Rust toolchain.

```sh
cargo install keyjey
```

After installing with `cargo`, wire the statusline manually in `~/.claude/settings.json`:

```json
{
  "statusLine": { "type": "command", "command": "keyjey" }
}
```

## Nerd Fonts (optional)

KeyJey supports [Nerd Fonts](https://www.nerdfonts.com) — patched fonts that add thousands of icons your terminal can render as glyphs. With a Nerd Font active, you can use icon symbols as `symbol` values in any module config instead of plain text or emoji.

**Install a Nerd Font:**

1. Download any font from **[nerdfonts.com](https://www.nerdfonts.com/font-downloads)** (popular picks: JetBrainsMono Nerd Font, FiraCode Nerd Font, Hack Nerd Font)
2. Install it on your system and set it as your terminal's font
3. Use Nerd Font glyphs in your `keyjey.toml`:

```toml
[keyjey.model]
symbol = "󰚩 "   # nf-md-robot

[keyjey.context_bar]
symbol = " "   # nf-oct-cpu
```

::: tip Finding more glyphs
Browse [nerdfonts.com/cheat-sheet](https://www.nerdfonts.com/cheat-sheet) to find any icon and paste it directly into your `keyjey.toml`.
:::

→ The [Showcase](/showcase#_6-nerd-fonts) has a full Nerd Fonts config example.

## Quick Start

Create `~/.config/keyjey.toml`:

```toml
[keyjey]
lines = ["$keyjey.model  $keyjey.cost  $keyjey.context_bar"]

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

Open a Claude Code session — your statusline will show the model name, session cost (turning yellow at $2, red at $5), and a 10-character context bar (warming up at 40%, going critical at 70%).

→ [Full Configuration Reference](/configuration)
→ [Showcase — ready-to-use configs](/showcase)

## Debugging

Run `keyjey explain` to inspect what KeyJey sees from Claude Code's context JSON:

```sh
keyjey explain
```

This shows each module's current rendered value, the config file path in use, and any warnings about missing data or misconfiguration.

## Inspired by [Starship](https://starship.rs)
