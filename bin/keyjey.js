#!/usr/bin/env node

const { runBinary, setupSupportedClis } = require("../scripts/npm-lib.cjs");

async function main(args) {
  if (args[0] === "setup") {
    const { config, claude, codex } = await setupSupportedClis();
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
      console.log(`Configured Codex CLI tui.status_line in ${codex.path}`);
      if (codex.backupPath) {
        console.log(`Backed up existing Codex config to ${codex.backupPath}`);
      }
    } else if (codex.detected) {
      console.log(`Codex CLI tui.status_line already exists at ${codex.path}`);
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
