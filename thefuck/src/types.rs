use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub raw: String,
    pub parts: Vec<String>,
}

impl Command {
    pub fn new(raw: String) -> Self {
        let parts = shell_words::split(&raw)
            .unwrap_or(vec![raw.clone()])
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();

        Self { raw, parts }
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rules: Vec<String>,
    pub exclude_rules: Vec<String>,
    pub wait_command: u64,
    pub require_confirmation: bool,
    pub no_colors: bool,
    pub debug: bool,
    pub priority: std::collections::HashMap<String, u32>,
    pub history_limit: Option<usize>,
    pub alter_history: bool,
    pub wait_slow_command: u64,
    pub slow_commands: Vec<String>,
    pub repeat: bool,
    pub instant_mode: bool,
    pub num_close_matches: usize,
    pub env: std::collections::HashMap<String, String>,
    pub excluded_search_path_prefixes: Vec<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rules: vec!["All rules enabled".to_string()],
            exclude_rules: vec![],
            wait_command: 3,
            require_confirmation: true,
            no_colors: false,
            debug: false,
            priority: std::collections::HashMap::new(),
            history_limit: None,
            alter_history: true,
            wait_slow_command: 15,
            slow_commands: vec![
                "lein".to_string(),
                "react-native".to_string(),
                "gradle".to_string(),
                "./gradlew".to_string(),
                "vagrant".to_string(),
            ],
            repeat: false,
            instant_mode: false,
            num_close_matches: 3,
            env: {
                let mut m = std::collections::HashMap::new();
                m.insert("LC_ALL".to_string(), "C".to_string());
                m.insert("LANG".to_string(), "C".to_string());
                m.insert("GIT_TRACE".to_string(), "1".to_string());
                m
            },
            excluded_search_path_prefixes: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub rule: String,
    pub corrected_command: String,
    pub similarity: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No rules found")]
    NoRulesFound,
    #[error("No correction found")]
    NoCorrectionFound,
    #[error("Rule not found: {0}")]
    RuleNotFound(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
}
