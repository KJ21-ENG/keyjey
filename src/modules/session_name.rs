use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use crate::config::KeyjeyConfig;
use crate::context::Context;

const DEFAULT_MAX_LENGTH: usize = 40;
const MAX_FIRST_LINE_BYTES: u64 = 64 * 1024;

pub fn render(ctx: &Context, cfg: &KeyjeyConfig) -> Option<String> {
    let sn_cfg = cfg.session_name.as_ref();
    if sn_cfg.and_then(|c| c.disabled) == Some(true) {
        return None;
    }

    let transcript_str = match ctx.transcript_path.as_deref() {
        Some(path) => path,
        None => {
            tracing::warn!("keyjey.session_name: transcript_path is absent - skipping");
            return None;
        }
    };
    let transcript_path = Path::new(transcript_str);
    let max_length = sn_cfg
        .and_then(|c| c.max_length)
        .unwrap_or(DEFAULT_MAX_LENGTH);

    let value = if let Some(session_id) = ctx.session_id.as_deref()
        && let Some(cached) = read_session_cache(session_id)
    {
        cached
    } else {
        let extracted = extract_first_user_message(transcript_path, max_length)?;
        if let Some(session_id) = ctx.session_id.as_deref() {
            write_session_cache(session_id, &extracted);
        }
        extracted
    };

    let symbol = sn_cfg.and_then(|c| c.symbol.as_deref());
    let style = sn_cfg.and_then(|c| c.style.as_deref());
    if let Some(fmt) = sn_cfg.and_then(|c| c.format.as_deref()) {
        return crate::format::apply_module_format(fmt, Some(&value), symbol, style);
    }

    let symbol_str = symbol.unwrap_or("");
    let content = format!("{symbol_str}{value}");
    Some(crate::ansi::apply_style(&content, style))
}

fn session_cache_path(session_id: &str) -> Option<PathBuf> {
    Some(
        crate::cache::global_cache_dir()?
            .join("session_name")
            .join(sanitize_filename(session_id)),
    )
}

fn read_session_cache(session_id: &str) -> Option<String> {
    std::fs::read_to_string(session_cache_path(session_id)?).ok()
}

fn write_session_cache(session_id: &str, content: &str) {
    if let Some(path) = session_cache_path(session_id) {
        crate::cache::atomic_write(&path, content.as_bytes());
    }
}

fn sanitize_filename(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "session".to_string()
    } else {
        out
    }
}

fn extract_first_user_message(path: &Path, max_length: usize) -> Option<String> {
    let file = std::fs::File::open(path).ok()?;
    let limited = file.take(MAX_FIRST_LINE_BYTES);
    let mut reader = BufReader::new(limited);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    if line.is_empty() {
        return None;
    }

    let value: serde_json::Value = serde_json::from_str(line.trim_end()).ok()?;
    if value.get("type").and_then(|v| v.as_str()) != Some("user") {
        return None;
    }
    let content = value
        .get("message")
        .and_then(|m| m.get("content"))
        .and_then(content_to_text)?;
    let cleaned = content.replace(['\n', '\r'], " ").trim().to_string();
    if cleaned.is_empty() {
        return None;
    }
    Some(truncate_chars(&cleaned, max_length))
}

fn content_to_text(value: &serde_json::Value) -> Option<String> {
    if let Some(s) = value.as_str() {
        return Some(s.to_string());
    }
    let arr = value.as_array()?;
    let parts = arr
        .iter()
        .filter(|item| item.get("type").and_then(|v| v.as_str()) == Some("text"))
        .filter_map(|item| item.get("text").and_then(|v| v.as_str()))
        .collect::<Vec<_>>();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn truncate_chars(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{KeyjeyConfig, SessionNameConfig};

    fn write_transcript(line: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("transcript.jsonl");
        std::fs::write(&path, line).unwrap();
        (dir, path)
    }

    fn ctx_for(path: &Path) -> Context {
        Context {
            session_id: Some(format!("test-{}", std::process::id())),
            transcript_path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_render_disabled_returns_none() {
        let (_dir, path) = write_transcript("{}\n");
        let cfg = KeyjeyConfig {
            session_name: Some(SessionNameConfig {
                disabled: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(render(&ctx_for(&path), &cfg).is_none());
    }

    #[test]
    fn test_render_no_transcript_path_returns_none() {
        assert!(render(&Context::default(), &KeyjeyConfig::default()).is_none());
    }

    #[test]
    fn test_extract_first_user_message_string_content() {
        let (_dir, path) =
            write_transcript(r#"{"type":"user","message":{"content":"Build me a TODO app"}}"#);
        assert_eq!(
            extract_first_user_message(&path, 40),
            Some("Build me a TODO app".to_string())
        );
    }

    #[test]
    fn test_extract_first_user_message_array_content() {
        let (_dir, path) = write_transcript(
            r#"{"type":"user","message":{"content":[{"type":"text","text":"Build"},{"type":"text","text":"TODO"}]}}"#,
        );
        assert_eq!(
            extract_first_user_message(&path, 40),
            Some("Build TODO".to_string())
        );
    }

    #[test]
    fn test_extract_first_user_message_truncates_to_max_length() {
        let (_dir, path) = write_transcript(
            r#"{"type":"user","message":{"content":"abcdefghijklmnopqrstuvwxyz"}}"#,
        );
        assert_eq!(
            extract_first_user_message(&path, 10),
            Some("abcdefghij".into())
        );
    }

    #[test]
    fn test_extract_first_user_message_custom_max_length() {
        let (_dir, path) =
            write_transcript(r#"{"type":"user","message":{"content":"custom length"}}"#);
        assert_eq!(extract_first_user_message(&path, 6), Some("custom".into()));
    }

    #[test]
    fn test_extract_first_user_message_replaces_newlines() {
        let (_dir, path) =
            write_transcript("{\"type\":\"user\",\"message\":{\"content\":\"hello\\nworld\"}}");
        assert_eq!(
            extract_first_user_message(&path, 40),
            Some("hello world".into())
        );
    }

    #[test]
    fn test_extract_first_user_message_skips_non_user_first_line() {
        let (_dir, path) = write_transcript(
            "{\"type\":\"assistant\",\"message\":{\"content\":\"no\"}}\n{\"type\":\"user\",\"message\":{\"content\":\"yes\"}}\n",
        );
        assert!(extract_first_user_message(&path, 40).is_none());
    }

    #[test]
    fn test_symbol_and_style_applied() {
        let (_dir, path) = write_transcript(r#"{"type":"user","message":{"content":"Build"}}"#);
        let ctx = Context {
            session_id: Some(format!("styled-{}", std::process::id())),
            transcript_path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let cfg = KeyjeyConfig {
            session_name: Some(SessionNameConfig {
                symbol: Some("S ".to_string()),
                style: Some("bold green".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let rendered = render(&ctx, &cfg).unwrap();
        assert!(rendered.contains("S Build"));
        assert!(rendered.contains('\x1b'));
    }

    #[test]
    fn test_render_uses_cache_on_second_call() {
        let (_dir, path) = write_transcript(r#"{"type":"user","message":{"content":"First"}}"#);
        let ctx = Context {
            session_id: Some(format!("cached-{}", std::process::id())),
            transcript_path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        };
        assert_eq!(render(&ctx, &KeyjeyConfig::default()), Some("First".into()));
        std::fs::write(&path, r#"{"type":"user","message":{"content":"Second"}}"#).unwrap();
        assert_eq!(render(&ctx, &KeyjeyConfig::default()), Some("First".into()));
    }
}
