use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::config::KeyjeyConfig;
use crate::context::Context;

pub fn render(ctx: &Context, cfg: &KeyjeyConfig) -> Option<String> {
    let reasoning_cfg = cfg.reasoning.as_ref();
    if reasoning_cfg.and_then(|c| c.disabled) == Some(true) {
        return None;
    }

    let value = resolve_effort_level(ctx).unwrap_or_else(|| {
        reasoning_cfg
            .and_then(|c| c.unknown_label.clone())
            .unwrap_or_else(|| "?".to_string())
    });

    let symbol = reasoning_cfg.and_then(|c| c.symbol.as_deref());
    let style = reasoning_cfg.and_then(|c| c.style.as_deref());
    if let Some(fmt) = reasoning_cfg.and_then(|c| c.format.as_deref()) {
        return crate::format::apply_module_format(fmt, Some(&value), symbol, style);
    }

    let symbol_str = symbol.unwrap_or("");
    let content = format!("{symbol_str}{value}");
    Some(crate::ansi::apply_style(&content, style))
}

fn resolve_effort_level(ctx: &Context) -> Option<String> {
    resolve_effort_level_from_paths(settings_paths(ctx))
}

fn resolve_effort_level_from_paths(paths: Vec<PathBuf>) -> Option<String> {
    for settings_path in paths {
        if let Some(cached) = crate::cache::read_keyed_cache("reasoning", &settings_path) {
            return Some(cached);
        }
        let Ok(raw) = std::fs::read_to_string(&settings_path) else {
            continue;
        };
        let Some(effort) = parse_effort_level(&raw) else {
            continue;
        };
        crate::cache::write_keyed_cache("reasoning", &settings_path, &effort);
        return Some(effort);
    }
    None
}

fn settings_paths(ctx: &Context) -> Vec<PathBuf> {
    let mut seen = HashSet::new();
    let mut paths = Vec::new();

    let mut push_dir = |dir: Option<&str>| {
        let Some(dir) = dir else {
            return;
        };
        let path = Path::new(dir).join(".claude").join("settings.json");
        let key = path.to_string_lossy().to_string();
        if seen.insert(key) {
            paths.push(path);
        }
    };

    push_dir(
        ctx.workspace
            .as_ref()
            .and_then(|w| w.project_dir.as_deref()),
    );
    push_dir(
        ctx.workspace
            .as_ref()
            .and_then(|w| w.current_dir.as_deref()),
    );
    push_dir(ctx.cwd.as_deref());

    if let Some(home) = dirs::home_dir() {
        let path = home.join(".claude").join("settings.json");
        let key = path.to_string_lossy().to_string();
        if seen.insert(key) {
            paths.push(path);
        }
    }

    paths
}

fn parse_effort_level(raw: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(raw).ok()?;
    value
        .get("effortLevel")
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{KeyjeyConfig, ReasoningConfig};
    use crate::context::{Context, Workspace};

    fn write_settings(dir: &Path, effort: &str) {
        let settings_dir = dir.join(".claude");
        std::fs::create_dir_all(&settings_dir).unwrap();
        std::fs::write(
            settings_dir.join("settings.json"),
            format!(r#"{{"effortLevel":"{effort}"}}"#),
        )
        .unwrap();
    }

    #[test]
    fn test_render_disabled_returns_none() {
        let cfg = KeyjeyConfig {
            reasoning: Some(ReasoningConfig {
                disabled: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(render(&Context::default(), &cfg).is_none());
    }

    #[test]
    fn test_resolve_from_project_settings() {
        let dir = tempfile::tempdir().unwrap();
        write_settings(dir.path(), "high");
        let ctx = Context {
            workspace: Some(Workspace {
                project_dir: Some(dir.path().to_string_lossy().to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(resolve_effort_level(&ctx), Some("high".to_string()));
    }

    #[test]
    fn test_resolve_project_takes_priority_over_user() {
        let project = tempfile::tempdir().unwrap();
        let current = tempfile::tempdir().unwrap();
        write_settings(project.path(), "low");
        write_settings(current.path(), "high");
        let ctx = Context {
            workspace: Some(Workspace {
                project_dir: Some(project.path().to_string_lossy().to_string()),
                current_dir: Some(current.path().to_string_lossy().to_string()),
            }),
            ..Default::default()
        };
        assert_eq!(resolve_effort_level(&ctx), Some("low".to_string()));
    }

    #[test]
    fn test_resolve_unknown_returns_question_mark_default() {
        assert_eq!(resolve_effort_level_from_paths(Vec::new()), None);
    }

    #[test]
    fn test_resolve_unknown_uses_custom_unknown_label() {
        let value = resolve_effort_level_from_paths(Vec::new()).unwrap_or_else(|| {
            ReasoningConfig {
                unknown_label: Some("unknown".to_string()),
                ..Default::default()
            }
            .unknown_label
            .unwrap()
        });
        assert_eq!(value, "unknown");
    }

    #[test]
    fn test_invalid_json_falls_through_to_next_source() {
        let project = tempfile::tempdir().unwrap();
        let current = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(project.path().join(".claude")).unwrap();
        std::fs::write(project.path().join(".claude/settings.json"), "{").unwrap();
        write_settings(current.path(), "medium");
        let ctx = Context {
            workspace: Some(Workspace {
                project_dir: Some(project.path().to_string_lossy().to_string()),
                current_dir: Some(current.path().to_string_lossy().to_string()),
            }),
            ..Default::default()
        };
        let paths = settings_paths(&ctx);
        assert_eq!(
            resolve_effort_level_from_paths(paths.into_iter().take(2).collect()),
            Some("medium".to_string())
        );
    }

    #[test]
    fn test_settings_mtime_change_invalidates_cache() {
        let dir = tempfile::tempdir().unwrap();
        write_settings(dir.path(), "low");
        let settings = dir.path().join(".claude/settings.json");
        let paths = vec![settings.clone()];
        assert_eq!(
            resolve_effort_level_from_paths(paths.clone()),
            Some("low".to_string())
        );

        std::fs::write(&settings, r#"{"effortLevel":"high"}"#).unwrap();
        let newer = std::time::SystemTime::now() + std::time::Duration::from_secs(5);
        filetime::set_file_mtime(&settings, filetime::FileTime::from_system_time(newer)).unwrap();

        assert_eq!(
            resolve_effort_level_from_paths(paths),
            Some("high".to_string())
        );
    }

    #[test]
    fn test_symbol_and_style_applied() {
        let dir = tempfile::tempdir().unwrap();
        write_settings(dir.path(), "high");
        let ctx = Context {
            cwd: Some(dir.path().to_string_lossy().to_string()),
            ..Default::default()
        };
        let cfg = KeyjeyConfig {
            reasoning: Some(ReasoningConfig {
                symbol: Some("R ".to_string()),
                style: Some("bold blue".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let rendered = render(&ctx, &cfg).unwrap();
        assert!(rendered.contains("R high"));
        assert!(rendered.contains('\x1b'));
    }
}
