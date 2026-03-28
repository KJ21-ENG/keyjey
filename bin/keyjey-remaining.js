#!/usr/bin/env node

const { runRemainingWrapper } = require("../scripts/npm-lib.cjs");

runRemainingWrapper(process.argv.slice(2)).catch((error) => {
  console.error(error.message || String(error));
  process.exit(1);
});
