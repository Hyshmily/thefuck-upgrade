#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub rules: Vec<String>,
    pub exclude_rules: Vec<String>,
    pub require_confirmation: bool,
    pub alter_history: bool,
    pub num_close_matches: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rules: vec!["All rules enabled".to_string()],
            exclude_rules: vec![],
            require_confirmation: true,
            alter_history: true,
            num_close_matches: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub rule: &'static str,
    pub corrected_command: String,
    pub similarity: f64,
}

#[derive(Debug)]
pub enum Error {
    InvalidCommand(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidCommand(msg) => write!(f, "Invalid command: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
