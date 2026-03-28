const fs = require("node:fs");
const fsp = require("node:fs/promises");
const path = require("node:path");
const { spawn } = require("node:child_process");
const { pipeline } = require("node:stream/promises");
const { Readable } = require("node:stream");

const REPO = "KJ21-ENG/keyjey";

function packageRoot() {
  return path.resolve(__dirname, "..");
}

function packageJson() {
  return JSON.parse(fs.readFileSync(path.join(packageRoot(), "package.json"), "utf8"));
}

function packageVersion() {
  return packageJson().version;
}

function releaseTag() {
  return `v${packageVersion()}`;
}

function resolveTarget() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "darwin" && arch === "arm64") {
    return "aarch64-apple-darwin";
  }
  if (platform === "darwin" && arch === "x64") {
    return "x86_64-apple-darwin";
  }
  if (platform === "linux" && arch === "x64") {
    return "x86_64-unknown-linux-musl";
  }
  if (platform === "linux" && arch === "arm64") {
    return "aarch64-unknown-linux-musl";
  }
  if (platform === "win32") {
    throw new Error("KeyJey does not support native Windows yet. Use WSL2 and install the Linux package there.");
  }

  throw new Error(`KeyJey does not support ${platform}/${arch}.`);
}

function binaryName() {
  return process.platform === "win32" ? "keyjey.exe" : "keyjey";
}

function installedBinaryPath() {
  if (process.env.KEYJEY_BINARY_PATH) {
    return path.resolve(process.env.KEYJEY_BINARY_PATH);
  }
  return path.join(packageRoot(), ".keyjey-bin", resolveTarget(), binaryName());
}

function binaryUrl() {
  return `https://github.com/${REPO}/releases/download/${releaseTag()}/keyjey-${resolveTarget()}`;
}

async function downloadBinary(destination) {
  const response = await fetch(binaryUrl(), { redirect: "follow" });
  if (!response.ok || !response.body) {
    throw new Error(
      `Failed to download KeyJey ${releaseTag()} for ${resolveTarget()} (${response.status} ${response.statusText}). ` +
        `Publish the GitHub release before using npm/npx.`
    );
  }

  await fsp.mkdir(path.dirname(destination), { recursive: true });
  const stream = fs.createWriteStream(destination, { mode: 0o755 });
  await pipeline(Readable.fromWeb(response.body), stream);
  if (process.platform !== "win32") {
    await fsp.chmod(destination, 0o755);
  }
}

async function ensureBinary() {
  const destination = installedBinaryPath();
  try {
    const stat = await fsp.stat(destination);
    if (stat.isFile() && stat.size > 0) {
      return destination;
    }
  } catch (_) {
    // Download below.
  }

  await downloadBinary(destination);
  return destination;
}

async function runBinary(args) {
  const binary = await ensureBinary();
  await new Promise((resolve, reject) => {
    const child = spawn(binary, args, { stdio: "inherit" });
    child.on("error", reject);
    child.on("exit", (code, signal) => {
      if (signal) {
        process.kill(process.pid, signal);
        return;
      }
      process.exitCode = code ?? 1;
      resolve();
    });
  });
}

module.exports = {
  ensureBinary,
  installedBinaryPath,
  packageVersion,
  releaseTag,
  resolveTarget,
  runBinary
};
