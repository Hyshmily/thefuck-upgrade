use crate::types::{Error, Settings};
use anyhow::Result;
use std::path::PathBuf;

const CONFIG_FILE: &str = "thefuck/settings.py";

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

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| Error::InvalidCommand("No config directory".to_string()))?;

    Ok(config_dir.join(CONFIG_FILE))
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

pub fn create_config_if_missing() -> Result<()> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        let default_settings = Settings::default();
        let python_config = generate_python_config(&default_settings);

        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&config_path, python_config)?;
    }
    Ok(())
}

fn generate_python_config(settings: &Settings) -> String {
    let mut config = String::new();
    config.push_str("# The Fuck settings file\n");
    config.push_str("#\n");
    config.push_str("# The rules are defined as in the example below:\n");
    config.push_str("#\n");
    config.push_str("# rules = ['cd_parent', 'git_push', 'python_command', 'sudo']\n");
    config.push_str("#\n");
    config
        .push_str("# The default values are as follows. Uncomment and change to fit your needs.\n");
    config.push_str("# See https://github.com/nvbn/thefuck#settings for more information.\n");
    config.push_str("#\n");
    config.push_str("\n");

    if !settings.rules.contains(&"All rules enabled".to_string()) {
        config.push_str(&format!("rules = {:?}\n", settings.rules));
        config.push_str("\n");
    }

    if !settings.exclude_rules.is_empty() {
        config.push_str(&format!("exclude_rules = {:?}\n", settings.exclude_rules));
        config.push_str("\n");
    }

    config.push_str(&format!("# wait_command = {}\n", settings.wait_command));
    config.push_str(&format!(
        "# require_confirmation = {}\n",
        settings.require_confirmation
    ));
    config.push_str(&format!("# no_colors = {}\n", settings.no_colors));
    config.push_str(&format!("# debug = {}\n", settings.debug));
    config.push_str(&format!("# alter_history = {}\n", settings.alter_history));
    config.push_str(&format!(
        "# wait_slow_command = {}\n",
        settings.wait_slow_command
    ));

    if !settings.slow_commands.is_empty() {
        config.push_str(&format!("# slow_commands = {:?}\n", settings.slow_commands));
    }

    config.push_str(&format!("# repeat = {}\n", settings.repeat));
    config.push_str(&format!("# instant_mode = {}\n", settings.instant_mode));
    config.push_str(&format!(
        "# num_close_matches = {}\n",
        settings.num_close_matches
    ));

    config
}
