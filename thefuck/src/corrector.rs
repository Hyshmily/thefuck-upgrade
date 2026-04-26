use crate::rules::RuleRegistry;
use crate::types::{Command, MatchResult};
use crate::util;

pub struct Corrector {
    command: Command,
    registry: RuleRegistry,
    rules: Vec<String>,
    exclude_rules: Vec<String>,
    num_close_matches: usize,
}

impl Corrector {
    pub fn new(command: Command, settings: crate::types::Settings) -> Self {
        Self {
            command,
            registry: RuleRegistry::new(),
            rules: settings.rules,
            exclude_rules: settings.exclude_rules,
            num_close_matches: settings.num_close_matches,
        }
    }

    pub fn find_corrections(&self) -> Vec<MatchResult> {
        let mut corrections =
            self.registry
                .match_command(&self.command, &self.rules, &self.exclude_rules);

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
        corrections.truncate(self.num_close_matches.max(1));
        corrections
    }
}
