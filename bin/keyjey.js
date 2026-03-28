#!/usr/bin/env node

const { runBinary } = require("../scripts/npm-lib.cjs");

runBinary(process.argv.slice(2)).catch((error) => {
  console.error(error.message || String(error));
  process.exit(1);
});
