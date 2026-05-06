#!/usr/bin/env node

const { runBinary, setupSupportedClis } = require("../scripts/npm-lib.cjs");

function setupUsage() {
  return [
    "Usage: keyjey setup [--codex-preset <rich|compact|minimal|off>] [--codex-force]",
    "",
    "Options:",
    "  --codex-preset <name>  Configure Codex CLI's native tui.status_line preset.",
    "  --codex-force          Replace an existing Codex status_line after creating a backup.",
    "  -h, --help             Show this help."
  ].join("\n");
}

function parseSetupArgs(args) {
  const options = { codex: {} };

  for (let i = 0; i < args.length; i += 1) {
    const arg = args[i];
    if (arg === "-h" || arg === "--help") {
      options.help = true;
      continue;
    }
    if (arg === "--codex-force") {
      options.codex.force = true;
      continue;
    }
    if (arg === "--codex-preset") {
      const value = args[i + 1];
      if (!value || value.startsWith("-")) {
        throw new Error("Missing value for --codex-preset.");
      }
      options.codex.preset = value;
      i += 1;
      continue;
    }
    if (arg.startsWith("--codex-preset=")) {
      options.codex.preset = arg.slice("--codex-preset=".length);
      continue;
    }

    throw new Error(`Unknown setup option: ${arg}`);
  }

  return options;
}

async function main(args) {
  if (args[0] === "setup") {
    const setupOptions = parseSetupArgs(args.slice(1));
    if (setupOptions.help) {
      console.log(setupUsage());
      return;
    }

    const { config, claude, codex } = await setupSupportedClis(setupOptions);
    if (config.created) {
      console.log(`Created default config at ${config.path}`);
    } else {
      console.log(`KeyJey config already exists at ${config.path}`);
    }

    if (claude.changed) {
      console.log(`Configured Claude Code statusLine in ${claude.path}`);
    } else {
      console.log(`Claude Code statusLine already exists at ${claude.path}`);
    }

    if (codex.changed) {
      if (codex.preset === "off") {
        console.log(`Disabled Codex CLI tui.status_line in ${codex.path}`);
      } else {
        console.log(`Configured Codex CLI tui.status_line preset "${codex.preset}" in ${codex.path}`);
      }
      if (codex.backupPath) {
        console.log(`Backed up existing Codex config to ${codex.backupPath}`);
      }
    } else if (codex.detected) {
      const forceHint = setupOptions.codex.preset && !setupOptions.codex.force ? "; use --codex-force to replace it" : "";
      console.log(`Codex CLI tui.status_line already exists at ${codex.path}${forceHint}.`);
    } else {
      console.log("Codex CLI not detected; skipped Codex config.");
    }
    return;
  }

  await runBinary(args);
}

main(process.argv.slice(2)).catch((error) => {
  console.error(error.message || String(error));
  process.exit(1);
});
