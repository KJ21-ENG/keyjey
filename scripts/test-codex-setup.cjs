const assert = require("node:assert/strict");
const fs = require("node:fs");
const fsp = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");

const { ensureCodexConfig } = require("./npm-lib.cjs");

const ORIGINAL_HOME = process.env.HOME;
const ORIGINAL_PATH = process.env.PATH;

async function withTempHome(fn) {
  const dir = await fsp.mkdtemp(path.join(os.tmpdir(), "keyjey-codex-test-"));
  try {
    process.env.HOME = dir;
    process.env.PATH = "";
    await fn(dir);
  } finally {
    process.env.HOME = ORIGINAL_HOME;
    process.env.PATH = ORIGINAL_PATH;
    await fsp.rm(dir, { recursive: true, force: true });
  }
}

async function addFakeCodex(home) {
  const binDir = path.join(home, "bin");
  await fsp.mkdir(binDir, { recursive: true });
  const codexPath = path.join(binDir, "codex");
  await fsp.writeFile(codexPath, "#!/bin/sh\nexit 0\n", "utf8");
  await fsp.chmod(codexPath, 0o755);
  process.env.PATH = binDir;
}

function codexConfigPath(home) {
  return path.join(home, ".codex", "config.toml");
}

function backupFiles(home) {
  const dir = path.join(home, ".codex");
  if (!fs.existsSync(dir)) return [];
  return fs.readdirSync(dir).filter((name) => name.startsWith("config.toml.backup.keyjey-"));
}

async function testNoCodexNoConfig() {
  await withTempHome(async (home) => {
    const result = await ensureCodexConfig();
    assert.equal(result.detected, false);
    assert.equal(result.changed, false);
    assert.equal(fs.existsSync(codexConfigPath(home)), false);
  });
}

async function testCodexDetectedCreatesConfig() {
  await withTempHome(async (home) => {
    await addFakeCodex(home);
    const result = await ensureCodexConfig();
    const config = await fsp.readFile(codexConfigPath(home), "utf8");

    assert.equal(result.detected, true);
    assert.equal(result.changed, true);
    assert.match(config, /\[tui]/);
    assert.match(config, /status_line = \[/);
    assert.match(config, /"model-with-reasoning"/);
    assert.match(config, /"weekly-limit"/);
  });
}

async function testExistingTuiGetsStatusLineAndBackup() {
  await withTempHome(async (home) => {
    const configPath = codexConfigPath(home);
    await fsp.mkdir(path.dirname(configPath), { recursive: true });
    await fsp.writeFile(configPath, 'model = "gpt-5.4"\n\n[tui]\nanimations = false\n\n[mcp_servers.docs]\ncommand = "docs"\n', "utf8");

    const result = await ensureCodexConfig();
    const config = await fsp.readFile(configPath, "utf8");

    assert.equal(result.detected, true);
    assert.equal(result.changed, true);
    assert.match(config, /\[tui]\nanimations = false\n\s*status_line = \[/);
    assert.match(config, /\[mcp_servers\.docs]/);
    assert.equal(backupFiles(home).length, 1);
  });
}

async function testDottedStatusLinePreserved() {
  await withTempHome(async (home) => {
    const configPath = codexConfigPath(home);
    const original = 'model = "gpt-5.4"\ntui.status_line = ["model-with-reasoning"]\n';
    await fsp.mkdir(path.dirname(configPath), { recursive: true });
    await fsp.writeFile(configPath, original, "utf8");

    const result = await ensureCodexConfig();
    const config = await fsp.readFile(configPath, "utf8");

    assert.equal(result.detected, true);
    assert.equal(result.changed, false);
    assert.equal(config, original);
    assert.equal(backupFiles(home).length, 0);
  });
}

async function testSectionStatusLinePreserved() {
  await withTempHome(async (home) => {
    const configPath = codexConfigPath(home);
    const original = '[tui]\nstatus_line = ["current-dir"]\nanimations = true\n';
    await fsp.mkdir(path.dirname(configPath), { recursive: true });
    await fsp.writeFile(configPath, original, "utf8");

    const result = await ensureCodexConfig();
    const config = await fsp.readFile(configPath, "utf8");

    assert.equal(result.detected, true);
    assert.equal(result.changed, false);
    assert.equal(config, original);
    assert.equal(backupFiles(home).length, 0);
  });
}

async function testExplicitPresetCreatesConfigWithoutCodexOnPath() {
  await withTempHome(async (home) => {
    const result = await ensureCodexConfig({ preset: "minimal" });
    const config = await fsp.readFile(codexConfigPath(home), "utf8");

    assert.equal(result.detected, true);
    assert.equal(result.changed, true);
    assert.equal(result.preset, "minimal");
    assert.match(config, /"model-with-reasoning"/);
    assert.match(config, /"current-dir"/);
    assert.match(config, /"git-branch"/);
    assert.doesNotMatch(config, /"weekly-limit"/);
  });
}

async function testPresetWithoutForcePreservesExistingStatusLine() {
  await withTempHome(async (home) => {
    const configPath = codexConfigPath(home);
    const original = '[tui]\nstatus_line = ["current-dir"]\nanimations = true\n';
    await fsp.mkdir(path.dirname(configPath), { recursive: true });
    await fsp.writeFile(configPath, original, "utf8");

    const result = await ensureCodexConfig({ preset: "compact" });
    const config = await fsp.readFile(configPath, "utf8");

    assert.equal(result.changed, false);
    assert.equal(result.reason, "status_line exists");
    assert.equal(config, original);
    assert.equal(backupFiles(home).length, 0);
  });
}

async function testForceReplacesSectionStatusLineWithPresetAndBackup() {
  await withTempHome(async (home) => {
    const configPath = codexConfigPath(home);
    await fsp.mkdir(path.dirname(configPath), { recursive: true });
    await fsp.writeFile(configPath, '[tui]\nstatus_line = ["current-dir"]\nanimations = true\n\n[tools]\nweb_search = true\n', "utf8");

    const result = await ensureCodexConfig({ preset: "compact", force: true });
    const config = await fsp.readFile(configPath, "utf8");

    assert.equal(result.changed, true);
    assert.equal(result.forced, true);
    assert.equal(result.preset, "compact");
    assert.match(config, /\[tui]\nanimations = true\n\s*status_line = \[/);
    assert.match(config, /"context-remaining"/);
    assert.doesNotMatch(config, /"weekly-limit"/);
    assert.match(config, /\[tools]/);
    assert.equal(backupFiles(home).length, 1);
  });
}

async function testForceReplacesDottedStatusLineWithOffPresetAndBackup() {
  await withTempHome(async (home) => {
    const configPath = codexConfigPath(home);
    await fsp.mkdir(path.dirname(configPath), { recursive: true });
    await fsp.writeFile(configPath, 'model = "gpt-5.4"\ntui.status_line = ["model-with-reasoning"]\n', "utf8");

    const result = await ensureCodexConfig({ preset: "off", force: true });
    const config = await fsp.readFile(configPath, "utf8");

    assert.equal(result.changed, true);
    assert.equal(result.preset, "off");
    assert.doesNotMatch(config, /tui\.status_line/);
    assert.match(config, /\[tui]\nstatus_line = \[]/);
    assert.equal(backupFiles(home).length, 1);
  });
}

async function main() {
  await testNoCodexNoConfig();
  await testCodexDetectedCreatesConfig();
  await testExistingTuiGetsStatusLineAndBackup();
  await testDottedStatusLinePreserved();
  await testSectionStatusLinePreserved();
  await testExplicitPresetCreatesConfigWithoutCodexOnPath();
  await testPresetWithoutForcePreservesExistingStatusLine();
  await testForceReplacesSectionStatusLineWithPresetAndBackup();
  await testForceReplacesDottedStatusLineWithOffPresetAndBackup();
  console.log("Codex setup tests passed");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
