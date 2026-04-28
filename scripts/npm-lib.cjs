const fs = require("node:fs");
const fsp = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const { execFileSync, spawn } = require("node:child_process");
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

function homeDir() {
  return os.homedir();
}

function userConfigPath() {
  return path.join(homeDir(), ".config", "keyjey.toml");
}

function claudeSettingsPath() {
  return path.join(homeDir(), ".claude", "settings.json");
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

async function runBinaryCapture(args, stdinText) {
  const binary = await ensureBinary();
  return await new Promise((resolve, reject) => {
    const child = spawn(binary, args, { stdio: ["pipe", "pipe", "pipe"] });
    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (chunk) => {
      stdout += chunk.toString();
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });
    child.on("error", reject);
    child.on("exit", (code, signal) => {
      resolve({ code: code ?? 1, signal, stdout, stderr });
    });

    child.stdin.end(stdinText ?? "");
  });
}

async function ensureUserConfig() {
  const destination = userConfigPath();
  try {
    const stat = await fsp.stat(destination);
    if (stat.isFile() && stat.size > 0) {
      return { created: false, path: destination };
    }
  } catch (_) {
    // Create below.
  }

  const source = await fsp.readFile(path.join(packageRoot(), "keyjey.toml"), "utf8");
  await fsp.mkdir(path.dirname(destination), { recursive: true });
  await fsp.writeFile(destination, source, "utf8");
  return { created: true, path: destination };
}

async function ensureClaudeSettings() {
  const destination = claudeSettingsPath();
  await fsp.mkdir(path.dirname(destination), { recursive: true });

  let data = {};
  try {
    data = JSON.parse(await fsp.readFile(destination, "utf8"));
  } catch (error) {
    if (error.code !== "ENOENT") {
      throw new Error(`Failed to parse ${destination}: ${error.message}`);
    }
  }

  let changed = false;
  if (!data.statusLine) {
    data.statusLine = { type: "command", command: "keyjey-remaining" };
    changed = true;
  } else if (
    data.statusLine &&
    data.statusLine.type === "command" &&
    data.statusLine.command === "keyjey"
  ) {
    data.statusLine.command = "keyjey-remaining";
    changed = true;
  }

  if (changed) {
    await fsp.writeFile(destination, `${JSON.stringify(data, null, 2)}\n`, "utf8");
  }

  return { changed, path: destination };
}

function stripAnsi(text) {
  return text.replace(/\u001b\[[0-9;?]*[ -/]*[@-~]/g, "");
}

function readKeyjeyConfigSection() {
  let raw = "";
  try {
    raw = fs.readFileSync(userConfigPath(), "utf8");
  } catch (_) {
    return "";
  }

  const lines = raw.split(/\r?\n/);
  let inSection = false;
  const section = [];

  for (const line of lines) {
    const header = line.match(/^\s*\[([^\]]+)]\s*$/);
    if (header) {
      if (inSection) break;
      inSection = header[1].trim() === "keyjey";
      continue;
    }

    if (inSection) {
      section.push(line);
    }
  }

  return section.length > 0 ? section.join("\n") : raw;
}

function parseKeyjeyConfigBool(section, key) {
  const regex = new RegExp(`^\\s*${key}\\s*=\\s*(true|false)\\s*(?:#.*)?$`, "im");
  const match = section.match(regex);
  return match ? match[1] === "true" : null;
}

function parseKeyjeyConfigNumber(section, key) {
  const regex = new RegExp(`^\\s*${key}\\s*=\\s*(\\d+)\\s*(?:#.*)?$`, "im");
  const match = section.match(regex);
  if (!match) return null;
  const value = Number(match[1]);
  return Number.isFinite(value) && value > 0 ? value : null;
}

function ttyColumns() {
  if (Number.isFinite(process.stdout.columns) && process.stdout.columns > 0) {
    return process.stdout.columns;
  }

  const envColumns = Number(process.env.COLUMNS);
  if (Number.isFinite(envColumns) && envColumns > 0) {
    return envColumns;
  }

  try {
    const output = execFileSync("sh", ["-c", "stty size < /dev/tty"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"]
    }).trim();
    const [, cols] = output.split(/\s+/).map(Number);
    return Number.isFinite(cols) && cols > 0 ? cols : null;
  } catch (_) {
    return null;
  }
}

function detectFinalWidth() {
  const section = readKeyjeyConfigSection();
  if (parseKeyjeyConfigBool(section, "truncate") === false) {
    return null;
  }

  return parseKeyjeyConfigNumber(section, "max_width") ?? ttyColumns();
}

function visibleWidth(text) {
  return Array.from(stripAnsi(text)).length;
}

function truncateAnsiLine(line, maxWidth) {
  if (!Number.isFinite(maxWidth) || maxWidth <= 0 || visibleWidth(line) <= maxWidth) {
    return maxWidth === 0 ? "" : line;
  }

  const budget = maxWidth > 1 ? maxWidth - 1 : maxWidth;
  let width = 0;
  let out = "";
  let emittedAnsi = false;

  for (let i = 0; i < line.length;) {
    if (line.charCodeAt(i) === 0x1b) {
      const match = line.slice(i).match(/^\u001b\[[0-9;?]*[ -/]*[@-~]/);
      if (match) {
        emittedAnsi = true;
        out += match[0];
        i += match[0].length;
        continue;
      }
    }

    const ch = Array.from(line.slice(i))[0];
    const nextWidth = width + 1;
    if (nextWidth > budget) {
      break;
    }
    out += ch;
    width = nextWidth;
    i += ch.length;
  }

  if (maxWidth > 1) {
    out += "…";
  }
  if (emittedAnsi) {
    out += "\u001b[0m";
  }
  return out;
}

function truncateOutputToWidth(output, width) {
  if (!Number.isFinite(width) || width <= 0) return output;
  return output
    .split(/\r?\n/)
    .map((line) => truncateAnsiLine(line, width))
    .join("\n");
}

function clamp(value) {
  const n = Number(value);
  if (!Number.isFinite(n)) return 0;
  return Math.max(0, Math.min(100, n));
}

function bar(percent) {
  const width = 16;
  const normalized = clamp(percent);
  const filled = Math.round((normalized * width) / 100);
  return `${"█".repeat(filled)}${"░".repeat(width - filled)}`;
}

function parseDurationSeconds(reset) {
  if (!reset) return null;
  let total = 0;
  let matched = false;
  for (const [, raw, unit] of reset.matchAll(/(\d+)\s*([dhms])/g)) {
    const n = Number(raw);
    matched = true;
    if (unit === "d") total += n * 86400;
    if (unit === "h") total += n * 3600;
    if (unit === "m") total += n * 60;
    if (unit === "s") total += n;
  }
  return matched ? total : null;
}

function formatLocalDate(epochMillis) {
  const d = new Date(epochMillis);
  const dd = String(d.getDate()).padStart(2, "0");
  const mm = String(d.getMonth() + 1).padStart(2, "0");
  const yyyy = d.getFullYear();
  const hh = String(d.getHours()).padStart(2, "0");
  const min = String(d.getMinutes()).padStart(2, "0");
  return `${dd}-${mm}-${yyyy} ${hh}:${min}`;
}

function resetAtFromDuration(reset) {
  const secs = parseDurationSeconds(reset);
  if (secs == null) return "";
  return formatLocalDate(Date.now() + secs * 1000);
}

function isoToEpochMillis(iso) {
  if (!iso) return null;
  const millis = Date.parse(iso);
  return Number.isFinite(millis) ? millis : null;
}

function isoToLocalReset(iso) {
  const millis = isoToEpochMillis(iso);
  if (millis == null) return "";
  return formatLocalDate(millis);
}

function formatUsage(label, used, resetDuration) {
  const remaining = clamp(100 - Number(used));
  const resetAt = resetDuration ? resetAtFromDuration(resetDuration) : "";
  return `${label}[${bar(remaining)}]${remaining}%${resetAt ? ` · ${resetAt}` : ""}`;
}

function rateLimitedUsage(label) {
  return `${label}[................]n/a · rate-limited`;
}

function formatUsageFromCache(label, used, resetIso) {
  if (used == null) {
    return rateLimitedUsage(label);
  }
  const resetMillis = isoToEpochMillis(resetIso);
  if (resetMillis == null || resetMillis <= Date.now()) {
    return rateLimitedUsage(label);
  }
  const remaining = clamp(100 - Number(used));
  return `${label}[${bar(remaining)}]${remaining}% · ${isoToLocalReset(resetIso)}`;
}

async function readStdin() {
  return await new Promise((resolve, reject) => {
    let data = "";
    process.stdin.setEncoding("utf8");
    process.stdin.on("data", (chunk) => {
      data += chunk;
    });
    process.stdin.on("end", () => resolve(data));
    process.stdin.on("error", reject);
    if (process.stdin.isTTY) {
      resolve("");
    }
  });
}

function findNewestUsageCache(rawOutput) {
  const candidates = [];
  const cwdSlug = process.cwd().replace(/[^A-Za-z0-9]+/g, "-");
  if (cwdSlug && cwdSlug !== "-") {
    candidates.push(path.join(homeDir(), ".claude", "projects", cwdSlug, "cship"));
  }

  const firstLine = stripAnsi(rawOutput).split(/\r?\n/).find(Boolean) || "";
  if (firstLine.includes(" on ")) {
    const repoHint = firstLine.split(" on ")[0];
    const projectsRoot = path.join(homeDir(), ".claude", "projects");
    try {
      for (const entry of fs.readdirSync(projectsRoot, { withFileTypes: true })) {
        if (entry.isDirectory() && entry.name.includes(repoHint)) {
          candidates.push(path.join(projectsRoot, entry.name, "cship"));
        }
      }
    } catch (_) {
      // Ignore.
    }
  }

  const seen = new Set();
  let newest = null;
  for (const dir of candidates) {
    if (!dir || seen.has(dir)) continue;
    seen.add(dir);
    try {
      for (const name of fs.readdirSync(dir)) {
        if (!name.endsWith("-usage-limits")) continue;
        const file = path.join(dir, name);
        const stat = fs.statSync(file);
        if (!newest || stat.mtimeMs > newest.mtimeMs) {
          newest = { file, mtimeMs: stat.mtimeMs };
        }
      }
    } catch (_) {
      // Ignore.
    }
  }

  if (!newest) {
    const projectsRoot = path.join(homeDir(), ".claude", "projects");
    try {
      for (const project of fs.readdirSync(projectsRoot, { withFileTypes: true })) {
        if (!project.isDirectory()) continue;
        const dir = path.join(projectsRoot, project.name, "cship");
        try {
          for (const name of fs.readdirSync(dir)) {
            if (!name.endsWith("-usage-limits")) continue;
            const file = path.join(dir, name);
            const stat = fs.statSync(file);
            if (!newest || stat.mtimeMs > newest.mtimeMs) {
              newest = { file, mtimeMs: stat.mtimeMs };
            }
          }
        } catch (_) {
          // Ignore.
        }
      }
    } catch (_) {
      // Ignore.
    }
  }

  if (!newest) return null;
  try {
    return JSON.parse(fs.readFileSync(newest.file, "utf8"));
  } catch (_) {
    return null;
  }
}

function applyUsageTransforms(output, fallbackCache) {
  let text = output;
  const livePatterns = [
    { regex: /(5h:)(\d{1,3})%\(([^)]*)\)/g, label: "5h:" },
    { regex: /(7d:)(\d{1,3})%\(([^)]*)\)/g, label: "7d:" },
    { regex: /(5h\s+)(\d{1,3})%\(([^)]*)\)/g, label: "5h " },
    { regex: /(7d\s+)(\d{1,3})%\(([^)]*)\)/g, label: "7d " }
  ];

  for (const { regex } of livePatterns) {
    text = text.replace(regex, (_, label, used, reset) => formatUsage(label, used, reset));
  }

  const fallbackPatterns = [
    { regex: /(5h:)(\d{1,3})%/g },
    { regex: /(7d:)(\d{1,3})%/g },
    { regex: /(5h\s+)(\d{1,3})%/g },
    { regex: /(7d\s+)(\d{1,3})%/g }
  ];

  for (const { regex } of fallbackPatterns) {
    text = text.replace(regex, (_, label, used) => formatUsage(label, used, null));
  }

  if (!fallbackCache || /5h:|7d:/.test(text)) {
    return text;
  }

  const data = fallbackCache.data || {};
  const five = formatUsageFromCache("5h:", data.five_hour_pct, data.five_hour_resets_at);
  const seven = formatUsageFromCache("7d:", data.seven_day_pct, data.seven_day_resets_at);
  const usage = [five, seven].filter(Boolean).join(" | ");
  if (!usage) return text;

  const lines = text.split(/\r?\n/);
  for (let i = lines.length - 1; i >= 0; i -= 1) {
    if (!lines[i].trim()) continue;
    if (/\|\s*$/.test(lines[i])) {
      lines[i] = lines[i].replace(/\|\s*$/, `| ${usage}`);
      return lines.join("\n");
    }
    lines[i] = `${lines[i]} | ${usage}`;
    return lines.join("\n");
  }
  return text;
}

async function runRemainingWrapper(args) {
  const stdinText = await readStdin();
  const result = await runBinaryCapture(args, stdinText);
  const fallbackCache = findNewestUsageCache(result.stdout);
  const transformed = truncateOutputToWidth(applyUsageTransforms(result.stdout, fallbackCache), detectFinalWidth());

  if (result.stderr) {
    process.stderr.write(result.stderr);
  }
  if (transformed) {
    process.stdout.write(transformed);
  }

  if (result.signal) {
    process.kill(process.pid, result.signal);
    return;
  }
  process.exitCode = result.code;
}

module.exports = {
  ensureBinary,
  ensureClaudeSettings,
  ensureUserConfig,
  homeDir,
  installedBinaryPath,
  packageVersion,
  releaseTag,
  resolveTarget,
  runBinary,
  runBinaryCapture,
  runRemainingWrapper
};
