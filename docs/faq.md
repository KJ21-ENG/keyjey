# FAQ

## Does keyjey conflict with Starship?

**No.** keyjey and Starship serve different purposes:

- **Starship** renders your *shell prompt* (the line before your commands in the terminal).
- **KeyJey** renders Claude Code's *statusline* (the line shown at the bottom of the Claude Code UI, inside the AI session).

They operate in completely separate contexts and never interfere with each other. In fact, they work better together — KeyJey can invoke any Starship module as a passthrough, so your Starship-configured git status, directory, and language runtime indicators can appear right in your Claude Code statusline.

---

## Why not just use Starship for the Claude Code statusline?

Starship is a shell prompt renderer — it reads shell context (environment variables, git state, file system). It has no knowledge of Claude Code internals like session cost, context window usage, model name, or API limits.

Claude Code exposes this session data via a JSON feed piped to the statusline command on every render cycle. KeyJey is purpose-built to consume that JSON feed and render it with the same TOML-based customization model you already know from Starship.

In short: Starship knows about your *shell*, KeyJey knows about your *Claude Code session*. Together they cover everything.

---

## How do I debug my config?

Run `keyjey explain`:

```sh
keyjey explain
```

This shows:
- Which config file was loaded (and from where)
- Each module's current rendered value
- Any warnings about missing data, misconfiguration, or disabled modules

`keyjey explain` reads from `~/.config/keyjey/sample-context.json` if no stdin is piped, so it works outside of a Claude Code session. On first run, KeyJey auto-creates this file with representative values.

---

## How do I set up usage limits on Linux/WSL2? {#usage-limits-linux}

The KeyJey `usage_limits` module fetches data from the Anthropic API using your Claude Code OAuth token, which is stored in the OS credential store.

**Prerequisites:**

1. Install `libsecret-tools`:
   ```sh
   # Debian/Ubuntu/WSL2
   sudo apt-get install -y libsecret-tools
   ```

2. Store your Claude Code OAuth token with `secret-tool`:
   ```sh
   secret-tool store --label="Claude Code" service "claude.ai" account "claude-code"
   ```
   When prompted for a password, paste your OAuth token.

   You can find your token in `~/.claude/.credentials.json` (look for the `access_token` field) or by logging out and back into Claude Code.

3. Run `keyjey explain` to verify the token is found and the usage limits module is rendering.

**macOS:** KeyJey reads the OAuth token from the macOS Keychain automatically — no manual setup required.

---

## Why is my cost or context not updating?

**Cost and context window** data comes from Claude Code's JSON feed, which is updated on every statusline render (every time Claude Code calls `keyjey`). If these values appear stuck, check:

- The statusline command is correctly set in `~/.claude/settings.json`:
  ```json
  { "statusLine": { "type": "command", "command": "keyjey" } }
  ```
- Run `keyjey explain` to confirm keyjey is receiving a valid JSON context.

**Usage limits** data is cached:
- Cache TTL: **configurable (default 60 seconds)**, or until the rate-limit reset window passes (whichever comes first). Set `[keyjey.usage_limits] ttl` to increase the cache interval if you run many concurrent sessions.
- The first call in a session always fetches fresh data; subsequent calls within the configured TTL return the cached value.
- If the cache seems stale, check that your OAuth token is valid (re-login to Claude Code if needed).

You can see the current cache state by running `keyjey explain` — it shows the usage limits value being rendered and any warnings if the API call failed.
