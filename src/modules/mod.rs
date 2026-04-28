pub mod agent;
pub mod context_bar;
pub mod context_window;
pub mod cost;
pub mod model;
pub mod reasoning;
pub mod session;
pub mod session_name;
pub mod usage_limits;
pub mod vim;
pub mod workspace;

/// All native keyjey module names — used by `keyjey explain` to enumerate modules.
/// Names map to `keyjey.*` internals for backwards compatibility.
pub const ALL_NATIVE_MODULES: &[&str] = &[
    "keyjey.model",
    "keyjey.model.display_name",
    "keyjey.model.id",
    "keyjey.cost",
    "keyjey.cost.total_cost_usd",
    "keyjey.cost.total_duration_ms",
    "keyjey.cost.total_api_duration_ms",
    "keyjey.cost.total_lines_added",
    "keyjey.cost.total_lines_removed",
    "keyjey.context_bar",
    "keyjey.context_window.used_percentage",
    "keyjey.context_window.remaining_percentage",
    "keyjey.context_window.size",
    "keyjey.context_window.total_input_tokens",
    "keyjey.context_window.total_output_tokens",
    "keyjey.context_window.exceeds_200k",
    "keyjey.context_window.current_usage.input_tokens",
    "keyjey.context_window.current_usage.output_tokens",
    "keyjey.context_window.current_usage.cache_creation_input_tokens",
    "keyjey.context_window.current_usage.cache_read_input_tokens",
    "keyjey.vim",
    "keyjey.vim.mode",
    "keyjey.agent",
    "keyjey.agent.name",
    "keyjey.cwd",
    "keyjey.session_id",
    "keyjey.transcript_path",
    "keyjey.version",
    "keyjey.output_style",
    "keyjey.workspace.current_dir",
    "keyjey.workspace.project_dir",
    "keyjey.usage_limits",
    "keyjey.session_name",
    "keyjey.reasoning",
];

/// Static dispatch registry — the ONLY file modified when adding a new native module.
/// [Source: architecture.md#Module System Architecture]
pub fn render_module(
    name: &str,
    ctx: &crate::context::Context,
    cfg: &crate::config::KeyjeyConfig,
) -> Option<String> {
    // Accept both keyjey.* (new) and keyjey.* (legacy) token namespaces.
    let canonical = if let Some(rest) = name.strip_prefix("keyjey.") {
        format!("keyjey.{rest}")
    } else {
        name.to_string()
    };

    match canonical.as_str() {
        "keyjey.model" => model::render(ctx, cfg),
        "keyjey.model.display_name" => model::render_display_name(ctx, cfg),
        "keyjey.model.id" => model::render_id(ctx, cfg),
        // Cost module — main alias and sub-fields
        "keyjey.cost" => cost::render(ctx, cfg),
        "keyjey.cost.total_cost_usd" => cost::render_total_cost_usd(ctx, cfg),
        "keyjey.cost.total_duration_ms" => cost::render_total_duration_ms(ctx, cfg),
        "keyjey.cost.total_api_duration_ms" => cost::render_total_api_duration_ms(ctx, cfg),
        "keyjey.cost.total_lines_added" => cost::render_total_lines_added(ctx, cfg),
        "keyjey.cost.total_lines_removed" => cost::render_total_lines_removed(ctx, cfg),
        // Context bar — progress bar with threshold styling
        "keyjey.context_bar" => context_bar::render(ctx, cfg),
        // Context window sub-fields
        "keyjey.context_window.used_percentage" => context_window::render_used_percentage(ctx, cfg),
        "keyjey.context_window.remaining_percentage" => {
            context_window::render_remaining_percentage(ctx, cfg)
        }
        "keyjey.context_window.size" => context_window::render_size(ctx, cfg),
        "keyjey.context_window.total_input_tokens" => {
            context_window::render_total_input_tokens(ctx, cfg)
        }
        "keyjey.context_window.total_output_tokens" => {
            context_window::render_total_output_tokens(ctx, cfg)
        }
        "keyjey.context_window.exceeds_200k" => context_window::render_exceeds_200k(ctx, cfg),
        "keyjey.context_window.current_usage.input_tokens" => {
            context_window::render_current_usage_input_tokens(ctx, cfg)
        }
        "keyjey.context_window.current_usage.output_tokens" => {
            context_window::render_current_usage_output_tokens(ctx, cfg)
        }
        "keyjey.context_window.current_usage.cache_creation_input_tokens" => {
            context_window::render_current_usage_cache_creation_input_tokens(ctx, cfg)
        }
        "keyjey.context_window.current_usage.cache_read_input_tokens" => {
            context_window::render_current_usage_cache_read_input_tokens(ctx, cfg)
        }
        // Vim module — mode display
        "keyjey.vim" => vim::render(ctx, cfg),
        "keyjey.vim.mode" => vim::render_mode(ctx, cfg),
        // Agent module — agent name display
        "keyjey.agent" => agent::render(ctx, cfg),
        "keyjey.agent.name" => agent::render_name(ctx, cfg),
        // Session identity modules
        "keyjey.cwd" => session::render_cwd(ctx, cfg),
        "keyjey.session_id" => session::render_session_id(ctx, cfg),
        "keyjey.transcript_path" => session::render_transcript_path(ctx, cfg),
        "keyjey.version" => session::render_version(ctx, cfg),
        "keyjey.output_style" => session::render_output_style(ctx, cfg),
        // Workspace modules
        "keyjey.workspace.current_dir" => workspace::render_current_dir(ctx, cfg),
        "keyjey.workspace.project_dir" => workspace::render_project_dir(ctx, cfg),
        // Usage limits module — non-blocking thread dispatch for live API data
        "keyjey.usage_limits" => usage_limits::render(ctx, cfg),
        // Derived session metadata
        "keyjey.session_name" => session_name::render(ctx, cfg),
        "keyjey.reasoning" => reasoning::render(ctx, cfg),
        other => {
            tracing::warn!("keyjey: unknown native module '{other}' — skipping");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::KeyjeyConfig;
    use crate::context::{Context, Model};

    #[test]
    fn test_dispatch_to_model_module_with_display_name_returns_some() {
        let ctx = Context {
            model: Some(Model {
                display_name: Some("Sonnet".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let cfg = KeyjeyConfig::default();
        let result = render_module("keyjey.model", &ctx, &cfg);
        assert!(result.is_some());
        assert!(result.unwrap().contains("Sonnet"));
    }

    #[test]
    fn test_unknown_module_name_returns_none() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig::default();
        assert!(render_module("keyjey.unknown_future_module", &ctx, &cfg).is_none());
    }
}
