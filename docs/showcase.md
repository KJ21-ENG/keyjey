# Showcase

Ready-to-use `keyjey.toml` configurations — from minimal to full-featured. Each can be dropped into `~/.config/keyjey.toml`.


---

## 1. Minimal

One clean row. Model, cost with colour thresholds, context bar.


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

---

## 2. Git-Aware Developer

Two rows: Starship git status on top, Claude session below.

Starship passthrough (`$directory`, `$git_branch`, `$git_status`) requires [Starship](https://starship.rs) to be installed.


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

---

## 3. Cost Guardian

Shows cost, lines changed, and rolling API usage limits all at once. Colour escalates as budgets fill.


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

---

## 4. Material Hex

Every style value is a `fg:#rrggbb` hex colour — no named colours anywhere. Amber warns, coral criticals.


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

---

## 5. Tokyo Night

Three-row layout for polyglot developers. Starship handles language runtimes and git; keyjey handles session data. Styled with the [Tokyo Night](https://github.com/folke/tokyonight.nvim) colour palette.


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

---

## 6. Nerd Fonts

Requires a [Nerd Font](https://www.nerdfonts.com) in your terminal. Icons are embedded as `symbol` values on each module and as literal characters in the format string for Starship passthrough rows.


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
symbol = " " # nf-fa-microchip
style  = "bold fg:#7aa2f7"

[keyjey.cost]
symbol             = "💰 "
style              = "fg:#a9b1d6"
warn_threshold     = 2.0
warn_style         = "fg:#e0af68"
critical_threshold = 5.0
critical_style     = "bold fg:#f7768e"

[keyjey.context_bar]
symbol             = " " # nf-fa-database
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

---

## Submit Your Config

Have a beautiful KeyJey setup? Share it with the community!

Open a pull request to [KJ21-ENG/keyjey](https://github.com/KJ21-ENG/keyjey) adding your config to this page.

Include:
- A screenshot or GIF of your statusline in action
- Your full annotated `keyjey.toml`
- A short description of the design choices
