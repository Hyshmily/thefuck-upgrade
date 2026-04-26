use crate::types::{Command, MatchResult};
use crate::util;

const TERRAFORM_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("apply", &["aplpy", "appl", "app y"]),
    ("plan", &["plna", "plan", "pla"]),
    ("init", &["int", "ini"]),
    ("destroy", &["destory", "destoy", "destroi"]),
    ("output", &["outptu", "outpu", "outpt"]),
    ("validate", &["valdiate", "validte", "vaildate"]),
    ("refresh", &["refrsh", "refesh", "refrehs"]),
];

pub fn terraform_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "terrafrom" | "terrafor" | "terrform" => "terraform",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "terraform_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn terraform_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "terraform" {
        return None;
    }

    for &(correct, typos) in TERRAFORM_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "terraform_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
