use crate::conf::load_settings;
use crate::rules::RuleRegistry;
use crate::types::{Command, MatchResult, Settings};
use anyhow::Result;

pub struct Corrector {
    settings: Settings,
    command: Command,
    registry: RuleRegistry,
}

impl Corrector {
    pub fn from_command(command: Command) -> Result<Self> {
        let settings = load_settings()?;
        Ok(Self::new(command, settings))
    }

    pub fn new(command: Command, settings: Settings) -> Self {
        Self {
            settings,
            command,
            registry: RuleRegistry::new(),
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn find_corrections(&self) -> Vec<MatchResult> {
        let mut corrections = self.registry.match_command(
            &self.command,
            &self.settings.rules,
            &self.settings.exclude_rules,
        );

        for correction in &mut corrections {
            if correction.similarity <= 0.0 {
                correction.similarity =
                    levenshtein_ratio(&self.command.raw, &correction.corrected_command);
            }
        }

        corrections.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        corrections.truncate(self.settings.num_close_matches.max(1));
        corrections
    }
}

// Simple Levenshtein distance implementation
pub fn levenshtein(s1: &str, s2: &str) -> usize {
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 0..=s1.len() {
        dp[i][0] = i;
    }

    for j in 0..=s2.len() {
        dp[0][j] = j;
    }

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + 1);
            }
        }
    }

    dp[s1.len()][s2.len()]
}

pub fn levenshtein_ratio(s1: &str, s2: &str) -> f64 {
    let distance = levenshtein(s1, s2);
    let max_len = s1.len().max(s2.len());
    if max_len == 0 {
        1.0
    } else {
        1.0 - (distance as f64 / max_len as f64)
    }
}
