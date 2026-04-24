use crate::types::Settings;
use anyhow::Result;

pub fn load_settings() -> Result<Settings> {
    let mut settings = Settings::default();

    if let Some(value) = env_bool("THEFUCK_REQUIRE_CONFIRMATION") {
        settings.require_confirmation = value;
    }

    if let Some(value) = env_bool("THEFUCK_ALTER_HISTORY") {
        settings.alter_history = value;
    }

    if let Some(value) = env_bool("THEFUCK_DEBUG") {
        settings.debug = value;
    }

    if let Ok(value) = std::env::var("THEFUCK_NUM_CLOSE_MATCHES") {
        if let Ok(parsed) = value.parse::<usize>() {
            settings.num_close_matches = parsed.max(1);
        }
    }

    if let Ok(value) = std::env::var("THEFUCK_RULES") {
        settings.rules = parse_list(&value);
    }

    if let Ok(value) = std::env::var("THEFUCK_EXCLUDE_RULES") {
        settings.exclude_rules = parse_list(&value);
    }

    Ok(settings)
}

fn parse_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn env_bool(key: &str) -> Option<bool> {
    std::env::var(key).ok().and_then(|value| {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        }
    })
}
