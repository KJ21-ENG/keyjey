const {
  ensureBinary,
  resolveTarget,
  releaseTag,
  setupSupportedClis
} = require("./npm-lib.cjs");

ensureBinary()
  .then(async (binaryPath) => {
    console.log(`KeyJey ${releaseTag()} installed for ${resolveTarget()} at ${binaryPath}`);

    if (process.env.npm_config_global === "true") {
      const { config, claude, codex } = await setupSupportedClis();

      if (config.created) {
        console.log(`Created default config at ${config.path}`);
      }
      if (claude.changed) {
        console.log(`Configured Claude Code statusLine in ${claude.path}`);
      }
      if (codex.changed) {
        console.log(`Configured Codex CLI tui.status_line in ${codex.path}`);
        if (codex.backupPath) {
          console.log(`Backed up existing Codex config to ${codex.backupPath}`);
        }
      }
      if (!config.created && !claude.changed && !codex.changed) {
        console.log("Claude Code and Codex CLI config already exist or Codex was not detected; no user files changed.");
      }
    }
  })
  .catch((error) => {
    console.error(error.message || String(error));
    process.exit(1);
  });
