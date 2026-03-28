const { ensureBinary, resolveTarget, releaseTag } = require("./npm-lib.cjs");

ensureBinary()
  .then((binaryPath) => {
    console.log(`KeyJey ${releaseTag()} installed for ${resolveTarget()} at ${binaryPath}`);
  })
  .catch((error) => {
    console.error(error.message || String(error));
    process.exit(1);
  });
