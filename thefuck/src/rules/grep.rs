use crate::types::{Command, MatchResult};
use crate::util;

pub fn grep_recursive_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "grep" {
        return None;
    }

    // If grep is used without -r/-R and without a file path,
    // suggest adding -r for recursive search
    let has_recursive = command
        .parts
        .iter()
        .any(|p| p == "-r" || p == "-R" || p == "--recursive");
    if has_recursive {
        return None;
    }

    // If the last argument looks like a pattern (not a file path),
    // suggest recursive grep
    let last = command.parts.last()?;
    if last.starts_with('-') {
        return None;
    }

    let mut corrected = command.parts.clone();
    corrected.insert(1, "-r".to_string());

    Some(MatchResult {
        rule: "grep_recursive",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_BRANCH,
    })
}
