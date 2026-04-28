use crate::config::KeyjeyConfig;
use crate::context::Context;

enum Token {
    Native(String),
    Passthrough(String),
    Literal(String), // bare text preserved verbatim
    StyledSpan { content: String, style: String },
}

fn parse_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < line.len() {
        let remaining = &line[pos..];

        // Check for styled span: [content](style) — only when '[' is at current position
        if let Some(after_bracket) = remaining.strip_prefix('[') {
            if let Some(close_bracket_offset) = after_bracket.find("](") {
                let content = &after_bracket[..close_bracket_offset];
                let after_open_paren = &after_bracket[close_bracket_offset + 2..]; // skip "]("
                if let Some(close_paren) = after_open_paren.find(')') {
                    let style = &after_open_paren[..close_paren];
                    tokens.push(Token::StyledSpan {
                        content: content.to_string(),
                        style: style.to_string(),
                    });
                    // advance: 1 ('[') + close_bracket_offset + 2 ('](') + close_paren + 1 (')')
                    pos += 1 + close_bracket_offset + 2 + close_paren + 1;
                    continue;
                }
            }
            // No matching ](...) found — emit literal "[" and advance one char
            tokens.push(Token::Literal("[".to_string()));
            pos += 1;
            continue;
        }

        // Find next special character: '$' or '[' — process whichever comes first
        let next_dollar = remaining.find('$');
        let next_bracket = remaining.find('[');
        let next_pos = match (next_dollar, next_bracket) {
            (Some(d), Some(b)) => Some(d.min(b)),
            (Some(d), None) => Some(d),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        match next_pos {
            Some(special_pos) if special_pos > 0 => {
                // Literal text before the next special character
                tokens.push(Token::Literal(remaining[..special_pos].to_string()));
                pos += special_pos;
                // Re-enter loop — next iteration handles '[' or '$' at position 0
            }
            Some(_) => {
                // special_pos == 0 and not '[' (handled above) → must be '$'
                let after_dollar = &remaining[1..];
                let name_end = after_dollar
                    .find(|c: char| c.is_whitespace() || c == '[' || c == '$')
                    .unwrap_or(after_dollar.len());
                let name = &after_dollar[..name_end];
                if !name.is_empty() {
                    if name == "fill" {
                        // $fill is deferred (future $keyjey.flex feature) — emit empty, warn once
                        static FILL_WARNED: std::sync::atomic::AtomicBool =
                            std::sync::atomic::AtomicBool::new(false);
                        if !FILL_WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                            tracing::warn!(
                                "keyjey: $fill is not yet supported (deferred to $keyjey.flex); rendering as empty"
                            );
                        }
                        tokens.push(Token::Literal(String::new()));
                    } else if name.starts_with("keyjey.") {
                        tokens.push(Token::Native(name.to_string()));
                    } else {
                        tokens.push(Token::Passthrough(name.to_string()));
                    }
                }
                pos += 1 + name_end;
            }
            None => {
                // No more special characters — remainder is all literal text
                if !remaining.is_empty() {
                    tokens.push(Token::Literal(remaining.to_string()));
                }
                break;
            }
        }
    }

    tokens
}

fn render_line(line: &str, ctx: &Context, cfg: &KeyjeyConfig) -> String {
    let mut parts: Vec<String> = Vec::new();

    for token in parse_line(line) {
        match token {
            Token::Native(name) => {
                if let Some(rendered) = crate::modules::render_module(&name, ctx, cfg) {
                    parts.push(rendered);
                }
            }
            Token::Passthrough(name) => {
                if let Some(rendered) = crate::passthrough::render_passthrough(&name, ctx) {
                    parts.push(rendered);
                }
            }
            Token::Literal(text) => {
                parts.push(text);
            }
            Token::StyledSpan { content, style } => {
                parts.push(crate::ansi::apply_style(&content, Some(&style)));
            }
        }
    }

    parts.join("") // No separator — spacing is encoded in Literal tokens
}

pub fn render(lines: &[String], ctx: &Context, cfg: &KeyjeyConfig) -> String {
    // cfg.format takes priority over lines; split on "$line_break" to produce rows
    let owned_lines: Vec<String>;
    let effective_lines: &[String] = if let Some(format_str) = &cfg.format {
        if !lines.is_empty() {
            tracing::warn!("keyjey: format field is set — ignoring lines config");
        }
        owned_lines = format_str
            .split("$line_break")
            .map(|s| s.to_string())
            .collect();
        &owned_lines
    } else {
        lines
    };

    let mut rendered_lines = effective_lines
        .iter()
        .map(|line| render_line(line, ctx, cfg))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    if cfg.truncate.unwrap_or(true)
        && let Some(width) = detect_terminal_width(cfg)
    {
        rendered_lines = rendered_lines
            .into_iter()
            .map(|line| crate::ansi::truncate_to_visible_width(&line, width, true))
            .collect();
    }

    rendered_lines.join("\n")
}

fn detect_terminal_width(cfg: &KeyjeyConfig) -> Option<usize> {
    if let Some(width) = cfg.max_width
        && width > 0
    {
        return Some(width as usize);
    }
    if let Ok(columns) = std::env::var("COLUMNS")
        && let Ok(width) = columns.parse::<usize>()
        && width > 0
    {
        return Some(width);
    }

    #[cfg(unix)]
    {
        let tty = std::fs::File::open("/dev/tty").ok()?;
        let (terminal_size::Width(width), _) = terminal_size::terminal_size_of(tty)?;
        return Some(width as usize);
    }

    #[cfg(windows)]
    {
        let (terminal_size::Width(width), _) = terminal_size::terminal_size()?;
        return Some(width as usize);
    }

    #[allow(unreachable_code)]
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::KeyjeyConfig;
    use crate::context::Context;

    #[test]
    fn test_parse_line_classifies_keyjey_as_native() {
        let tokens = parse_line("$keyjey.model");
        assert!(matches!(tokens[0], Token::Native(ref n) if n == "keyjey.model"));
    }

    #[test]
    fn test_parse_line_classifies_legacy_keyjey_as_native() {
        let tokens = parse_line("$keyjey.model");
        assert!(matches!(tokens[0], Token::Native(ref n) if n == "keyjey.model"));
    }

    #[test]
    fn test_parse_line_classifies_other_as_passthrough() {
        let tokens = parse_line("$git_branch");
        assert!(matches!(tokens[0], Token::Passthrough(ref n) if n == "git_branch"));
    }

    #[test]
    fn test_parse_line_literal_text_produces_literal_token() {
        // REPLACES test_parse_line_ignores_non_dollar_words
        let tokens = parse_line("literal text without dollar");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Literal(ref t) if t == "literal text without dollar"));
    }

    #[test]
    fn test_parse_line_preserves_prefix_literal() {
        let tokens = parse_line("in: $keyjey.context_window.total_input_tokens");
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0], Token::Literal(ref t) if t == "in: "));
        assert!(
            matches!(tokens[1], Token::Native(ref n) if n == "keyjey.context_window.total_input_tokens")
        );
    }

    #[test]
    fn test_parse_line_styled_span_with_content() {
        let tokens = parse_line("[text](bold green)");
        assert_eq!(tokens.len(), 1);
        assert!(
            matches!(&tokens[0], Token::StyledSpan { content, style } if content == "text" && style == "bold green")
        );
    }

    #[test]
    fn test_parse_line_empty_styled_span() {
        let tokens = parse_line("[](fg:#3d414a bg:#5f6366)");
        assert_eq!(tokens.len(), 1);
        assert!(
            matches!(&tokens[0], Token::StyledSpan { content, style } if content.is_empty() && style == "fg:#3d414a bg:#5f6366")
        );
    }

    #[test]
    fn test_parse_line_unclosed_bracket_literal() {
        let tokens = parse_line("[note:");
        // "[" emitted as Literal, then "note:" as Literal
        assert_eq!(tokens.len(), 2);
        assert!(matches!(&tokens[0], Token::Literal(t) if t == "["));
        assert!(matches!(&tokens[1], Token::Literal(t) if t == "note:"));
    }

    #[test]
    fn test_parse_line_mixed_span_and_native() {
        let tokens = parse_line("[bold](bold) $keyjey.model");
        assert_eq!(tokens.len(), 3);
        assert!(
            matches!(&tokens[0], Token::StyledSpan { content, style } if content == "bold" && style == "bold")
        );
        assert!(matches!(&tokens[1], Token::Literal(t) if t == " "));
        assert!(matches!(&tokens[2], Token::Native(n) if n == "keyjey.model"));
    }

    #[test]
    fn test_parse_line_spaces_styled_span() {
        // AC6: two spaces with foreground color
        let tokens = parse_line("[  ](fg:#ffc878)");
        assert_eq!(tokens.len(), 1);
        assert!(
            matches!(&tokens[0], Token::StyledSpan { content, style } if content == "  " && style == "fg:#ffc878")
        );
    }

    #[test]
    fn test_parse_line_styled_span_after_literal_text() {
        // Regression: styled spans preceded by literal text must still be parsed
        let tokens = parse_line("prefix [text](bold green)");
        assert_eq!(tokens.len(), 2);
        assert!(matches!(&tokens[0], Token::Literal(t) if t == "prefix "));
        assert!(
            matches!(&tokens[1], Token::StyledSpan { content, style } if content == "text" && style == "bold green")
        );
    }

    #[test]
    fn test_parse_line_styled_span_adjacent_to_token_no_space() {
        // Regression: $token[span](style) without space must not absorb '[' into token name
        let tokens = parse_line("$keyjey.model[sep](fg:red)");
        assert_eq!(tokens.len(), 2);
        assert!(matches!(&tokens[0], Token::Native(n) if n == "keyjey.model"));
        assert!(
            matches!(&tokens[1], Token::StyledSpan { content, style } if content == "sep" && style == "fg:red")
        );
    }

    #[test]
    fn test_parse_line_styled_span_after_native_token() {
        // Regression: styled span after $token must be parsed (not eaten as literal)
        let tokens = parse_line("$keyjey.model [sep](fg:red) $keyjey.cost");
        assert_eq!(tokens.len(), 5);
        assert!(matches!(&tokens[0], Token::Native(n) if n == "keyjey.model"));
        assert!(matches!(&tokens[1], Token::Literal(t) if t == " "));
        assert!(
            matches!(&tokens[2], Token::StyledSpan { content, style } if content == "sep" && style == "fg:red")
        );
        assert!(matches!(&tokens[3], Token::Literal(t) if t == " "));
        assert!(matches!(&tokens[4], Token::Native(n) if n == "keyjey.cost"));
    }

    #[test]
    fn test_render_empty_lines_is_empty() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig::default();
        let result = render(&[], &ctx, &cfg);
        assert_eq!(result, "");
    }

    #[test]
    fn test_render_two_empty_lines_filtered_to_empty() {
        // With default context (no model), both lines render empty and are filtered out
        let ctx = Context::default();
        let cfg = KeyjeyConfig::default();
        let lines = vec!["$keyjey.model".to_string(), "$keyjey.model".to_string()];
        let result = render(&lines, &ctx, &cfg);
        // Both tokens render to None → empty strings filtered out → empty result
        assert_eq!(result, "");
    }

    #[test]
    fn test_render_line_literal_and_native_concatenated_without_extra_space() {
        // Verifies AC6 behavior: "in: " + "15234" = "in: 15234" (no double space)
        use crate::context::{Context, ContextWindow};
        let ctx = Context {
            context_window: Some(ContextWindow {
                total_input_tokens: Some(15234),
                ..Default::default()
            }),
            ..Default::default()
        };
        let cfg = KeyjeyConfig::default();
        let result = render_line("in: $keyjey.context_window.total_input_tokens", &ctx, &cfg);
        assert_eq!(result, "in: 15234");
    }

    #[test]
    fn test_render_format_field_line_break() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig {
            format: Some("line1$line_breakline2".to_string()),
            ..Default::default()
        };
        let result = render(&[], &ctx, &cfg);
        assert_eq!(result, "line1\nline2");
    }

    #[test]
    fn test_render_format_takes_priority_over_lines() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig {
            format: Some("from_format".to_string()),
            lines: Some(vec!["from_lines".to_string()]),
            ..Default::default()
        };
        let lines = cfg.lines.as_deref().unwrap_or(&[]);
        let result = render(lines, &ctx, &cfg);
        assert_eq!(result, "from_format");
    }

    #[test]
    fn test_render_lines_unchanged_when_no_format() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig {
            lines: Some(vec!["hello".to_string()]),
            ..Default::default()
        };
        let lines = cfg.lines.as_deref().unwrap_or(&[]);
        let result = render(lines, &ctx, &cfg);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_render_truncates_when_max_width_set() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig {
            max_width: Some(5),
            ..Default::default()
        };
        let rendered = render(&["abcdef".to_string()], &ctx, &cfg);
        assert_eq!(rendered, "abcd…");
    }

    #[test]
    fn test_render_does_not_truncate_when_truncate_false() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig {
            truncate: Some(false),
            max_width: Some(5),
            ..Default::default()
        };
        let rendered = render(&["abcdef".to_string()], &ctx, &cfg);
        assert_eq!(rendered, "abcdef");
    }

    #[test]
    fn test_render_truncates_each_line_independently() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig {
            max_width: Some(4),
            ..Default::default()
        };
        let rendered = render(&["abcdef".to_string(), "xy".to_string()], &ctx, &cfg);
        assert_eq!(rendered, "abc…\nxy");
    }

    #[test]
    fn test_render_fill_token_renders_empty() {
        let ctx = Context::default();
        let cfg = KeyjeyConfig::default();
        let result = render_line("$fill", &ctx, &cfg);
        assert_eq!(result, "");
    }
}
