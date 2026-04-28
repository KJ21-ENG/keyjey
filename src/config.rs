use serde::Deserialize;

/// Root configuration for KeyJey, loaded from the `[keyjey]` section of `starship.toml`.
#[derive(Debug, Deserialize, Default)]
pub struct KeyjeyConfig {
    /// `lines` array — each element is a format string for one statusline row.
    /// Example: `["$keyjey.model $git_branch", "$keyjey.cost"]`
    pub lines: Option<Vec<String>>,
    /// Starship-compatible top-level format string. Split on `$line_break` to produce
    /// multiple rows. Takes priority over `lines` when both are set.
    pub format: Option<String>,
    /// Configuration for the `[keyjey.model]` section.
    pub model: Option<ModelConfig>,
    pub cost: Option<CostConfig>,
    pub context_bar: Option<ContextBarConfig>,
    pub context_window: Option<ContextWindowConfig>,
    pub vim: Option<VimConfig>,
    pub agent: Option<AgentConfig>,
    pub session: Option<SessionConfig>,
    pub workspace: Option<WorkspaceConfig>,
    pub usage_limits: Option<UsageLimitsConfig>,
    pub session_name: Option<SessionNameConfig>,
    pub reasoning: Option<ReasoningConfig>,
    /// Enable terminal-width truncation. Defaults to true when absent.
    pub truncate: Option<bool>,
    /// Explicit max render width. Overrides terminal detection when set.
    pub max_width: Option<u32>,
}

/// Per-module config fields shared by all native KeyJey modules.
/// These map to `[keyjey.model]` in `starship.toml`.
#[derive(Debug, Deserialize, Default)]
pub struct ModelConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    /// When `true`, prepends the module name as a label.
    pub label: Option<bool>,
    pub format: Option<String>,
}

/// Configuration for `[keyjey.cost]` — convenience alias for total cost display.
#[derive(Debug, Deserialize, Default)]
pub struct CostConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    /// Reserved — not yet rendered; included for config schema consistency with other modules.
    pub label: Option<String>,
    pub warn_threshold: Option<f64>,
    pub warn_style: Option<String>,
    pub critical_threshold: Option<f64>,
    pub critical_style: Option<String>,
    pub format: Option<String>,
    // Sub-field per-display configs (map to [keyjey.cost.total_cost_usd] etc.)
    pub total_cost_usd: Option<CostSubfieldConfig>,
    pub total_duration_ms: Option<CostSubfieldConfig>,
    pub total_api_duration_ms: Option<CostSubfieldConfig>,
    pub total_lines_added: Option<CostSubfieldConfig>,
    pub total_lines_removed: Option<CostSubfieldConfig>,
}

/// Configuration for individual `[keyjey.cost.*]` sub-field modules.
#[derive(Debug, Deserialize, Default)]
pub struct CostSubfieldConfig {
    pub style: Option<String>,
    /// Reserved — not yet rendered; included for config schema consistency.
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    /// Reserved — not yet rendered; included for config schema consistency.
    pub label: Option<String>,
    pub warn_threshold: Option<f64>,
    pub warn_style: Option<String>,
    pub critical_threshold: Option<f64>,
    pub critical_style: Option<String>,
    pub format: Option<String>,
}

/// Configuration for individual `[keyjey.context_window.*]` sub-field modules.
#[derive(Debug, Deserialize, Default)]
pub struct ContextWindowSubfieldConfig {
    pub style: Option<String>,
    /// Used only in the format path (via `$symbol`); ignored in the default render path.
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub warn_threshold: Option<f64>,
    pub warn_style: Option<String>,
    pub critical_threshold: Option<f64>,
    pub critical_style: Option<String>,
    pub format: Option<String>,
    /// When `true`, fires threshold styles when value is BELOW the threshold.
    /// Use for decreasing-health indicators like `remaining_percentage` (low = bad).
    /// When set, parent-level thresholds are NOT inherited (they are in the non-inverted domain).
    pub invert_threshold: Option<bool>,
}

/// Configuration for `[keyjey.context_bar]` — visual progress bar with thresholds.
/// Implemented in Story 2.2. Defined here so all Epic 2 config is available.
#[derive(Debug, Deserialize, Default)]
pub struct ContextBarConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub label: Option<String>,
    pub warn_threshold: Option<f64>,
    pub warn_style: Option<String>,
    pub critical_threshold: Option<f64>,
    pub critical_style: Option<String>,
    pub width: Option<u32>,
    pub format: Option<String>,
}

/// Configuration for `[keyjey.context_window]` sub-field modules.
/// Implemented in Story 2.2. Defined here so all Epic 2 config is available.
#[derive(Debug, Deserialize, Default)]
pub struct ContextWindowConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub label: Option<String>,
    pub warn_threshold: Option<f64>,
    pub warn_style: Option<String>,
    pub critical_threshold: Option<f64>,
    pub critical_style: Option<String>,
    pub format: Option<String>,
    // Per-sub-field configs (map to [keyjey.context_window.used_percentage] etc.)
    pub used_percentage: Option<ContextWindowSubfieldConfig>,
    pub remaining_percentage: Option<ContextWindowSubfieldConfig>,
    pub size: Option<ContextWindowSubfieldConfig>,
    pub total_input_tokens: Option<ContextWindowSubfieldConfig>,
    pub total_output_tokens: Option<ContextWindowSubfieldConfig>,
    pub current_usage_input_tokens: Option<ContextWindowSubfieldConfig>,
    pub current_usage_output_tokens: Option<ContextWindowSubfieldConfig>,
    pub current_usage_cache_creation_input_tokens: Option<ContextWindowSubfieldConfig>,
    pub current_usage_cache_read_input_tokens: Option<ContextWindowSubfieldConfig>,
}

/// Configuration for `[keyjey.vim]` — vim mode display.
/// Implemented in Story 2.3. Defined here so all Epic 2 config is available.
#[derive(Debug, Deserialize, Default)]
pub struct VimConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub label: Option<String>,
    pub normal_style: Option<String>,
    pub insert_style: Option<String>,
    pub format: Option<String>,
}

/// Configuration for `[keyjey.agent]` — agent name display.
/// Implemented in Story 2.3. Defined here so all Epic 2 config is available.
#[derive(Debug, Deserialize, Default)]
pub struct AgentConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub label: Option<String>,
    pub format: Option<String>,
}

/// Configuration for session identity modules (cwd, session_id, transcript_path, etc.).
/// Implemented in Story 2.4. Defined here so all Epic 2 config is available.
#[derive(Debug, Deserialize, Default)]
pub struct SessionConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub label: Option<String>,
    pub format: Option<String>,
}

/// Configuration for workspace modules (workspace.current_dir, workspace.project_dir).
/// Implemented in Story 2.4. Defined here so all Epic 2 config is available.
#[derive(Debug, Deserialize, Default)]
pub struct WorkspaceConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub label: Option<String>,
    pub format: Option<String>,
}

/// Configuration for `[keyjey.usage_limits]`.
/// Story 5.1 defines the struct; Stories 5.2 and 5.3 implement the render logic.
#[derive(Debug, Deserialize, Default)]
pub struct UsageLimitsConfig {
    pub disabled: Option<bool>,
    pub style: Option<String>,
    pub warn_threshold: Option<f64>,
    pub warn_style: Option<String>,
    pub critical_threshold: Option<f64>,
    pub critical_style: Option<String>,
    /// Cache refresh interval in seconds. Default: 60.
    /// Increase to reduce API pressure with many concurrent sessions.
    pub ttl: Option<u64>,
    /// Reserved — not yet rendered. Use `five_hour_format`, `seven_day_format`,
    /// and `separator` for per-section format control.
    pub format: Option<String>,
    pub five_hour_format: Option<String>,
    pub seven_day_format: Option<String>,
    pub separator: Option<String>,
}

/// Configuration for `[keyjey.session_name]`.
#[derive(Debug, Deserialize, Default)]
pub struct SessionNameConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub format: Option<String>,
    /// Maximum visible characters from the first user message. Default: 40.
    pub max_length: Option<usize>,
}

/// Configuration for `[keyjey.reasoning]`.
#[derive(Debug, Deserialize, Default)]
pub struct ReasoningConfig {
    pub style: Option<String>,
    pub symbol: Option<String>,
    pub disabled: Option<bool>,
    pub format: Option<String>,
    /// Label rendered when no Claude Code effortLevel can be found. Default: "?".
    pub unknown_label: Option<String>,
}

/// Result of a config load operation — includes the loaded config and its source.
pub struct ConfigLoadResult {
    pub config: KeyjeyConfig,
    pub source: ConfigSource,
}

/// Describes where the config was loaded from.
pub enum ConfigSource {
    ProjectLocal(std::path::PathBuf),
    Global(std::path::PathBuf),
    Override(std::path::PathBuf),
    /// Config loaded from a dedicated `keyjey.toml` file.
    /// Supports both canonical `[keyjey]` section format and legacy wrapper-free format.
    DedicatedFile(std::path::PathBuf),
    Default,
}

impl std::fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigSource::ProjectLocal(p) | ConfigSource::Global(p) | ConfigSource::Override(p) => {
                write!(f, "{}", p.display())
            }
            ConfigSource::DedicatedFile(p) => write!(
                f,
                "{} ({})",
                p.display(),
                p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("dedicated config")
            ),
            ConfigSource::Default => write!(f, "(default — no config file found)"),
        }
    }
}

/// Discover and load config, returning both the config and where it was loaded from.
/// Used by `explain.rs` to show the user which config was loaded.
/// Implements the same discovery chain as `discover_and_load` (AC1).
pub fn load_with_source(
    override_path: Option<&std::path::Path>,
    workspace_dir: Option<&str>,
) -> ConfigLoadResult {
    // Step 1: --config flag override
    if let Some(path) = override_path {
        let config = load_override(path).unwrap_or_else(|e| {
            tracing::warn!("keyjey: failed to load config from {}: {e}", path.display());
            KeyjeyConfig::default()
        });
        return ConfigLoadResult {
            config,
            source: if is_dedicated_filename(path) {
                ConfigSource::DedicatedFile(path.to_path_buf())
            } else {
                ConfigSource::Override(path.to_path_buf())
            },
        };
    }

    // Step 2: Walk up from workspace_dir — check keyjey.toml, then starship.toml
    if let Some(dir) = workspace_dir {
        let mut current = std::path::Path::new(dir);
        loop {
            let keyjey_candidate = current.join("keyjey.toml");
            if keyjey_candidate.exists() {
                let config = load_keyjey_toml(&keyjey_candidate).unwrap_or_else(|e| {
                    tracing::warn!("keyjey: failed to load dedicated config: {e}");
                    KeyjeyConfig::default()
                });
                return ConfigLoadResult {
                    config,
                    source: ConfigSource::DedicatedFile(keyjey_candidate),
                };
            }
            let candidate = current.join("starship.toml");
            if candidate.exists() {
                let config = load_from_path(&candidate).unwrap_or_else(|e| {
                    tracing::warn!("keyjey: failed to load project-local config: {e}");
                    KeyjeyConfig::default()
                });
                return ConfigLoadResult {
                    config,
                    source: ConfigSource::ProjectLocal(candidate),
                };
            }
            match current.parent() {
                Some(parent) => current = parent,
                None => break,
            }
        }
    }

    // Step 3: Global fallback — check ~/.config/keyjey.toml, then ~/.config/starship.toml
    if let Ok(home) = std::env::var("HOME") {
        let keyjey_global = std::path::Path::new(&home)
            .join(".config")
            .join("keyjey.toml");
        if keyjey_global.exists() {
            let config = load_keyjey_toml(&keyjey_global).unwrap_or_else(|e| {
                tracing::warn!("keyjey: failed to load global dedicated config: {e}");
                KeyjeyConfig::default()
            });
            return ConfigLoadResult {
                config,
                source: ConfigSource::DedicatedFile(keyjey_global),
            };
        }
        let global = std::path::Path::new(&home)
            .join(".config")
            .join("starship.toml");
        if global.exists() {
            let config = load_from_path(&global).unwrap_or_else(|e| {
                tracing::warn!("keyjey: failed to load global config: {e}");
                KeyjeyConfig::default()
            });
            return ConfigLoadResult {
                config,
                source: ConfigSource::Global(global),
            };
        }
    }

    // Step 4: No config found — use defaults
    tracing::debug!("no config file found; using default KeyjeyConfig");
    ConfigLoadResult {
        config: KeyjeyConfig::default(),
        source: ConfigSource::Default,
    }
}

/// Private wrapper so `toml::from_str` can extract `[keyjey]` sections
/// from a full `starship.toml` that contains many other sections.
/// Serde silently ignores all non-`keyjey` top-level keys.
#[derive(Debug, Deserialize, Default)]
struct StarshipToml {
    keyjey: Option<KeyjeyConfig>,
}

fn is_dedicated_filename(path: &std::path::Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|name| name == "keyjey.toml")
        .unwrap_or(false)
}

/// Load `KeyjeyConfig` from a `starship.toml`-format file at `path`.
/// Returns an error if the file cannot be read OR if the TOML is malformed.
/// Returns default `KeyjeyConfig` if `[keyjey]` section is absent (not an error).
fn load_from_path(path: &std::path::Path) -> anyhow::Result<KeyjeyConfig> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read config file {}: {e}", path.display()))?;
    let wrapper: StarshipToml = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("malformed TOML in {}: {e}", path.display()))?;
    tracing::debug!("loaded config from {}", path.display());
    Ok(wrapper.keyjey.unwrap_or_default())
}

/// Load `KeyjeyConfig` from a dedicated `keyjey.toml` file at `path`.
///
/// The canonical format uses a `[keyjey]` section header (parsed via `StarshipToml` wrapper).
/// Files without a `[keyjey]` header are treated as legacy wrapper-free format and parsed
/// directly as `KeyjeyConfig` for backwards compatibility.
fn load_keyjey_toml(path: &std::path::Path) -> anyhow::Result<KeyjeyConfig> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read config file {}: {e}", path.display()))?;
    let value: toml::Value = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("malformed TOML in {}: {e}", path.display()))?;
    if value.get("keyjey").is_some() {
        tracing::debug!("loading dedicated config with [keyjey] section via wrapper");
        let wrapper: StarshipToml = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("malformed TOML in {}: {e}", path.display()))?;
        tracing::debug!("loaded config from {} (via wrapper)", path.display());
        return Ok(wrapper.keyjey.unwrap_or_default());
    }
    tracing::debug!("loading dedicated config in legacy wrapper-free format");
    let config: KeyjeyConfig = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("malformed TOML in {}: {e}", path.display()))?;
    tracing::debug!("loaded config from {}", path.display());
    Ok(config)
}

/// Load `KeyjeyConfig` from an explicit `--config` override path.
/// Routes to `load_keyjey_toml` if the filename is `keyjey.toml`, otherwise
/// uses the `StarshipToml` wrapper (standard starship.toml format).
fn load_override(path: &std::path::Path) -> anyhow::Result<KeyjeyConfig> {
    if is_dedicated_filename(path) {
        load_keyjey_toml(path)
    } else {
        load_from_path(path)
    }
}

/// Discover and load `KeyjeyConfig` using the discovery chain.
///
/// Priority order:
/// 1. If `config_path` is `Some`, load that file directly (bypasses discovery).
/// 2. Walk up the directory tree from `workspace_dir`, checking `keyjey.toml` then `starship.toml`.
/// 3. Fall back to `$HOME/.config/keyjey.toml`, then `$HOME/.config/starship.toml`.
/// 4. If nothing found, return `KeyjeyConfig::default()` without error.
///
/// Returns `Err` only if a file is found but fails to parse (malformed TOML or unreadable).
pub fn discover_and_load(
    workspace_dir: Option<&str>,
    config_path: Option<&str>,
) -> anyhow::Result<KeyjeyConfig> {
    // Step 1: explicit override — propagate parse errors (caller handles exit)
    if let Some(path) = config_path {
        return load_override(std::path::Path::new(path));
    }
    // Steps 2–4: delegate to load_with_source (workspace walk-up → global → default)
    Ok(load_with_source(None, workspace_dir).config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const VALID_TOML: &str = include_str!("../tests/fixtures/sample_starship.toml");

    #[test]
    fn test_parse_valid_config() {
        let wrapper: StarshipToml = toml::from_str(VALID_TOML).unwrap();
        let cfg = wrapper.keyjey.unwrap();
        let lines = cfg.lines.as_ref().unwrap();
        assert!(!lines.is_empty(), "lines should be populated");
        let model = cfg.model.as_ref().unwrap();
        assert!(model.style.is_some(), "model.style should be present");
        assert_eq!(model.disabled, Some(false));
    }

    #[test]
    fn test_no_keyjey_section_returns_default() {
        let toml_without_keyjey = "[git_branch]\nstyle = \"bold green\"\n";
        let wrapper: StarshipToml = toml::from_str(toml_without_keyjey).unwrap();
        assert!(wrapper.keyjey.is_none());
        let cfg = wrapper.keyjey.unwrap_or_default();
        assert!(cfg.lines.is_none());
        assert!(cfg.model.is_none());
    }

    #[test]
    fn test_malformed_toml_returns_error() {
        let result: Result<StarshipToml, _> = toml::from_str("lines = [unclosed");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_nonexistent_path_returns_error() {
        let result = load_from_path(std::path::Path::new("/nonexistent/path/starship.toml"));
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("cannot read config file"),
            "error message: {msg}"
        );
    }

    #[test]
    fn test_discover_config_override_bypasses_discovery() {
        // Write a temp toml file
        let dir = std::env::temp_dir();
        let path = dir.join("keyjey_test_override.toml");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "[keyjey]\nlines = [\"$keyjey.model\"]").unwrap();

        let cfg = discover_and_load(None, path.to_str()).unwrap();
        assert_eq!(cfg.lines.as_ref().unwrap()[0], "$keyjey.model");

        std::fs::remove_file(&path).ok();
    }

    // NOTE: "no config found → returns default" is NOT unit-tested here because it
    // depends on HOME env var. If the dev's ~/.config/starship.toml exists, the
    // global fallback fires and the test would fail. This scenario is covered by
    // the integration test `test_no_config_file_found_uses_default_and_exits_zero`
    // which uses a JSON fixture with a non-existent workspace path and does not
    // depend on env vars. Mutating HOME in unit tests is unsafe with parallel
    // test execution (Rust 2024 marks set_var as unsafe for this reason).

    #[test]
    fn test_discover_walks_up_directory_tree() {
        // Create a temp dir hierarchy: /tmp/keyjey_test_walk/subdir/
        // Put starship.toml in the parent, workspace_dir is subdir
        let parent = std::env::temp_dir().join("keyjey_test_walk");
        let subdir = parent.join("subdir");
        std::fs::create_dir_all(&subdir).unwrap();
        let toml_path = parent.join("starship.toml");
        let mut f = std::fs::File::create(&toml_path).unwrap();
        writeln!(f, "[keyjey]\nlines = [\"$keyjey.model\"]").unwrap();

        let cfg = discover_and_load(subdir.to_str(), None).unwrap();
        assert_eq!(cfg.lines.as_ref().unwrap()[0], "$keyjey.model");

        // cleanup
        std::fs::remove_dir_all(&parent).ok();
    }

    // --- Task 6: keyjey.toml discovery tests ---
    //
    // NOTE: AC#3 (~/.config/keyjey.toml global fallback) is not directly unit-tested.
    // The global path uses the same `load_keyjey_toml` function tested below, and
    // mutating HOME in parallel tests is unsafe (Rust 2024). The workspace walk-up
    // path (AC#4) exercises the same discovery+load logic.  AC#3 correctness is
    // verified by code review of `load_with_source` Step 3.

    #[test]
    fn test_keyjey_toml_takes_priority_over_starship_toml() {
        // AC#1: keyjey.toml in workspace dir is loaded instead of starship.toml
        let dir = tempfile::tempdir().unwrap();

        let keyjey_path = dir.path().join("keyjey.toml");
        let mut f = std::fs::File::create(&keyjey_path).unwrap();
        writeln!(f, "lines = [\"$keyjey.cost\"]").unwrap();

        let starship_path = dir.path().join("starship.toml");
        let mut g = std::fs::File::create(&starship_path).unwrap();
        writeln!(g, "[keyjey]\nlines = [\"$keyjey.model\"]").unwrap();

        let cfg = discover_and_load(dir.path().to_str(), None).unwrap();
        // Should load from keyjey.toml (cost), not starship.toml (model)
        assert_eq!(cfg.lines.as_ref().unwrap()[0], "$keyjey.cost");
    }

    #[test]
    fn test_keyjey_toml_absent_falls_through_to_starship_toml() {
        // AC#2: without keyjey.toml, existing starship.toml discovery is unchanged
        let dir = tempfile::tempdir().unwrap();

        let starship_path = dir.path().join("starship.toml");
        let mut f = std::fs::File::create(&starship_path).unwrap();
        writeln!(f, "[keyjey]\nlines = [\"$keyjey.model\"]").unwrap();

        let cfg = discover_and_load(dir.path().to_str(), None).unwrap();
        assert_eq!(cfg.lines.as_ref().unwrap()[0], "$keyjey.model");
    }

    #[test]
    fn test_keyjey_toml_walk_up_from_subdirectory() {
        // AC#4: keyjey.toml in parent directory is discovered via walk-up
        let parent = tempfile::tempdir().unwrap();
        let subdir = parent.path().join("child");
        std::fs::create_dir_all(&subdir).unwrap();

        let keyjey_path = parent.path().join("keyjey.toml");
        let mut f = std::fs::File::create(&keyjey_path).unwrap();
        writeln!(f, "lines = [\"$keyjey.workspace\"]").unwrap();

        let cfg = discover_and_load(subdir.to_str(), None).unwrap();
        assert_eq!(cfg.lines.as_ref().unwrap()[0], "$keyjey.workspace");
    }

    #[test]
    fn test_keyjey_toml_with_wrapper_section_still_parses() {
        // AC#5: canonical [keyjey] format is parsed correctly via StarshipToml wrapper
        let dir = tempfile::tempdir().unwrap();

        let keyjey_path = dir.path().join("keyjey.toml");
        let mut f = std::fs::File::create(&keyjey_path).unwrap();
        writeln!(f, "[keyjey]\nlines = [\"$keyjey.agent\"]").unwrap();

        let cfg = load_keyjey_toml(&keyjey_path).unwrap();
        // [keyjey] is the canonical format — config is extracted correctly
        assert_eq!(cfg.lines.as_ref().unwrap()[0], "$keyjey.agent");
    }

    #[test]
    fn test_keyjey_toml_canonical_format_loads_correctly() {
        // AC#5: keyjey.toml with [keyjey] header uses the primary (debug) path
        let dir = tempfile::tempdir().unwrap();

        let keyjey_path = dir.path().join("keyjey.toml");
        let mut f = std::fs::File::create(&keyjey_path).unwrap();
        writeln!(
            f,
            "[keyjey]\nlines = [\"$keyjey.model $keyjey.cost $keyjey.context_bar\"]"
        )
        .unwrap();

        // The warn path has been removed; canonical [keyjey] format loads successfully
        let cfg = load_keyjey_toml(&keyjey_path).unwrap();
        assert_eq!(
            cfg.lines.as_ref().unwrap()[0],
            "$keyjey.model $keyjey.cost $keyjey.context_bar"
        );
    }

    #[test]
    fn test_keyjey_toml_direct_keyjeyconfig_parsing() {
        // Legacy wrapper-free format (backwards compatible) — no [keyjey] section header
        let dir = tempfile::tempdir().unwrap();

        let keyjey_path = dir.path().join("keyjey.toml");
        let mut f = std::fs::File::create(&keyjey_path).unwrap();
        writeln!(
            f,
            "lines = [\"$keyjey.model\"]\n\n[model]\nstyle = \"bold green\""
        )
        .unwrap();

        let cfg = load_keyjey_toml(&keyjey_path).unwrap();
        assert_eq!(cfg.lines.as_ref().unwrap()[0], "$keyjey.model");
        assert_eq!(
            cfg.model.as_ref().unwrap().style.as_deref(),
            Some("bold green")
        );
    }

    #[test]
    fn test_dedicated_file_config_source_display() {
        // AC#6: DedicatedFile variant displays path with (keyjey.toml) annotation
        let path = std::path::PathBuf::from("/home/user/.config/keyjey.toml");
        let source = ConfigSource::DedicatedFile(path);
        let display = source.to_string();
        assert!(display.contains("keyjey.toml"), "display: {display}");
        assert!(
            display.contains("(keyjey.toml)"),
            "should have annotation: {display}"
        );
    }

    #[test]
    fn test_override_keyjey_toml_routes_correctly() {
        // AC#5: --config override pointing to keyjey.toml parses directly as KeyjeyConfig
        let dir = tempfile::tempdir().unwrap();

        let keyjey_path = dir.path().join("keyjey.toml");
        let mut f = std::fs::File::create(&keyjey_path).unwrap();
        writeln!(f, "lines = [\"$keyjey.session\"]").unwrap();

        let cfg = discover_and_load(None, keyjey_path.to_str()).unwrap();
        assert_eq!(cfg.lines.as_ref().unwrap()[0], "$keyjey.session");
    }

    #[test]
    fn test_load_with_source_returns_dedicated_file_variant() {
        // M1: Verify discovery flow returns ConfigSource::DedicatedFile for keyjey.toml
        let dir = tempfile::tempdir().unwrap();

        let keyjey_path = dir.path().join("keyjey.toml");
        let mut f = std::fs::File::create(&keyjey_path).unwrap();
        writeln!(f, "lines = [\"$keyjey.cost\"]").unwrap();

        let result = load_with_source(None, dir.path().to_str());
        assert!(
            matches!(result.source, ConfigSource::DedicatedFile(_)),
            "expected DedicatedFile source, got: {}",
            result.source
        );
        assert_eq!(result.config.lines.as_ref().unwrap()[0], "$keyjey.cost");
    }

    #[test]
    fn test_load_with_source_override_keyjey_toml_returns_dedicated_file_variant() {
        // Regression: --config override pointing to keyjey.toml must return DedicatedFile, not Override
        let dir = tempfile::tempdir().unwrap();

        let keyjey_path = dir.path().join("keyjey.toml");
        let mut f = std::fs::File::create(&keyjey_path).unwrap();
        writeln!(f, "lines = [\"$keyjey.cost\"]").unwrap();

        let result = load_with_source(Some(&keyjey_path), None);
        assert!(
            matches!(result.source, ConfigSource::DedicatedFile(_)),
            "expected DedicatedFile source for keyjey.toml override, got: {}",
            result.source
        );
        assert_eq!(result.config.lines.as_ref().unwrap()[0], "$keyjey.cost");
    }
}
