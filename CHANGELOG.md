# Changelog

## 0.1.2 - 2026-04-28

- Added `$keyjey.session_name`, derived from the transcript's first user message.
- Added `$keyjey.reasoning`, resolved from Claude Code `.claude/settings.json` `effortLevel`.
- Added terminal-width truncation with `truncate` and `max_width` config options.
- Moved `$keyjey.usage_limits` to a shared OS cache at `keyjey/usage-limits.json`, with legacy per-transcript cache fallback.
- Note: idle Claude Code windows can still show stale statusline text until Claude Code re-invokes KeyJey in that window; the shared usage-limits cache only removes duplicate HTTP fetches and cross-window cache drift on the next render.
