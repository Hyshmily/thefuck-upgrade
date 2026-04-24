use crate::conf::load_settings;
use crate::rules::RuleRegistry;
use crate::types::{Command, MatchResult, Settings};
use crate::util;
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
                    util::levenshtein_ratio(&self.command.raw, &correction.corrected_command);
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
