#!/usr/bin/env bash
set -euo pipefail

# Allow KEYJEY_TEST_ROOT to override HOME for all path resolution (testability)
ROOT="${KEYJEY_TEST_ROOT:-$HOME}"
INSTALL_DIR="$ROOT/.local/bin"

# ── 1. OS / Arch Detection ────────────────────────────────────────────────────
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64)  TARGET="aarch64-apple-darwin" ;;
      x86_64) TARGET="x86_64-apple-darwin" ;;
      *)      echo "Unsupported macOS arch: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      x86_64)  TARGET="x86_64-unknown-linux-musl" ;;
      aarch64) TARGET="aarch64-unknown-linux-musl" ;;
      *)       echo "Unsupported Linux arch: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS" >&2; exit 1 ;;
esac

echo "Detected: $OS/$ARCH → target: $TARGET"

# ── 2. Download Binary ────────────────────────────────────────────────────────
BINARY_URL="https://github.com/KJ21-ENG/keyjey/releases/latest/download/keyjey-${TARGET}"
mkdir -p "$INSTALL_DIR"
echo "Downloading keyjey from $BINARY_URL ..."
curl -fsSL "$BINARY_URL" -o "${INSTALL_DIR}/keyjey"
chmod +x "${INSTALL_DIR}/keyjey"
if [ ! -s "${INSTALL_DIR}/keyjey" ]; then
  echo "Error: downloaded binary is empty — check network or release URL" >&2
  rm -f "${INSTALL_DIR}/keyjey"
  exit 1
fi
echo "Installed keyjey to ${INSTALL_DIR}/keyjey"

# ── 3. Linux: libsecret-tools check (usage limits dependency) ─────────────────
if [ "$OS" = "Linux" ] && ! command -v secret-tool >/dev/null 2>&1; then
  printf "Install libsecret-tools? (required for usage limits on Linux) [Y/n] "
  read -r answer </dev/tty
  case "$answer" in
    [Nn]*) echo "Skipping — usage limits module unavailable until installed manually." ;;
    *)     sudo apt-get install -y libsecret-tools ;;
  esac
fi

# ── 4. Starship detection and optional install ────────────────────────────────
if ! command -v starship >/dev/null 2>&1; then
  printf "Starship not found. Install Starship? (required for passthrough modules) [Y/n] "
  read -r answer </dev/tty
  case "$answer" in
    [Nn]*) echo "Skipping Starship install. Native keyjey modules will still work." ;;
    *)     curl -sS https://starship.rs/install.sh | sh ;;
  esac
fi

# ── 5. keyjey.toml — create minimal config (idempotent) ───────────────────────
KEYJEY_CONFIG="$ROOT/.config/keyjey.toml"
mkdir -p "$(dirname "$KEYJEY_CONFIG")"

KEYJEY_BLOCK='# keyjey — Claude Code statusline
# Full config reference: https://keyjey.dev
[keyjey]
lines = [
  "$directory$git_branch$git_status$python$nodejs$rust",
  "$keyjey.model $keyjey.cost $keyjey.context_bar $keyjey.usage_limits"
]

[keyjey.model]
symbol = "🤖 "
style  = "bold cyan"

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
five_hour_format   = "⌛ 5h {pct}% ({reset})"
seven_day_format   = "📅 7d {pct}% ({reset})"
separator          = " "
warn_threshold     = 60.0
warn_style         = "fg:#e0af68"
critical_threshold = 80.0
critical_style     = "bold fg:#f7768e"
'

if [ -f "$KEYJEY_CONFIG" ]; then
  echo "keyjey.toml already exists at $KEYJEY_CONFIG, skipping."
else
  printf '%s' "$KEYJEY_BLOCK" > "$KEYJEY_CONFIG"
  echo "Created minimal keyjey config at $KEYJEY_CONFIG"
fi

# ── 6. ~/.claude/settings.json — wire statusline (via python3) ───────────────
SETTINGS="$ROOT/.claude/settings.json"
if ! command -v python3 >/dev/null 2>&1; then
  echo "Warning: python3 not found. Skipping settings.json update."
  echo "To wire keyjey manually, add \"statusline\": \"keyjey\" to $SETTINGS"
elif [ -f "$SETTINGS" ]; then
  python3 - "$SETTINGS" <<'PYEOF' || echo "Warning: failed to update settings.json — add statusLine manually."
import json, sys
path = sys.argv[1]
try:
    with open(path) as f:
        d = json.load(f)
except (json.JSONDecodeError, ValueError) as e:
    print('Warning: ' + path + ' contains invalid JSON: ' + str(e))
    sys.exit(1)
if 'statusLine' not in d:
    d['statusLine'] = {'type': 'command', 'command': 'keyjey'}
    with open(path, 'w') as f:
        json.dump(d, f, indent=2)
        f.write('\n')
    print('Added statusLine config to ' + path)
else:
    print('"statusLine" already set in ' + path + ', skipping.')
PYEOF
else
  echo "settings.json not found at $SETTINGS — skipping (Claude Code may not be installed yet)."
fi

# ── 7. First-run preview ──────────────────────────────────────────────────────
echo ""
echo "Running keyjey explain..."
"$INSTALL_DIR/keyjey" explain || true

echo ""
echo "keyjey installation complete!"
echo "If ~/.local/bin is not in your PATH, add: export PATH=\"\$HOME/.local/bin:\$PATH\""
