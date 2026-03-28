const {
  ensureBinary,
  ensureClaudeSettings,
  ensureUserConfig,
  resolveTarget,
  releaseTag
} = require("./npm-lib.cjs");

ensureBinary()
  .then(async (binaryPath) => {
    console.log(`KeyJey ${releaseTag()} installed for ${resolveTarget()} at ${binaryPath}`);

    if (process.env.npm_config_global === "true") {
      const config = await ensureUserConfig();
      const settings = await ensureClaudeSettings();

      if (config.created) {
        console.log(`Created default config at ${config.path}`);
      }
      if (settings.changed) {
        console.log(`Configured Claude Code statusLine in ${settings.path}`);
      }
      if (!config.created && !settings.changed) {
        console.log("Claude Code config already exists; no user files changed.");
      }
    }
  })
  .catch((error) => {
    console.error(error.message || String(error));
    process.exit(1);
  });
