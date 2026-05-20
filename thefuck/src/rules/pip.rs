use crate::rules::helpers;
use crate::types::{Command, MatchResult};
use crate::util;

pub fn pip_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "ppi" | "pip" => None,
        "pi" => Some("pip"),
        _ => None,
    }?;

    Some(MatchResult {
        rule: "pip_command",
        corrected_command: helpers::replace_first(&command.parts, replacement),
        similarity: util::SIMILARITY_TYPO,
    })
}
