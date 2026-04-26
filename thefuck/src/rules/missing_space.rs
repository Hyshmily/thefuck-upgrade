use crate::types::{Command, MatchResult};
use crate::util;

/// Known command binaries that might be hyphen-joined with their subcommands.
const KNOWN_COMMANDS: &[&str] = &[
    "git",
    "docker",
    "npm",
    "npx",
    "yarn",
    "pnpm",
    "cargo",
    "go",
    "apt",
    "brew",
    "kubectl",
    "systemctl",
    "terraform",
    "conda",
    "pip",
    "python",
    "mvn",
    "gradle",
];

pub fn missing_space_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    // Case 1: "cd.." -> "cd .."
    if command.parts.len() == 1 {
        let raw = &command.parts[0];
        if let Some(rest) = raw.strip_prefix("cd") {
            if !rest.is_empty() && !rest.starts_with(' ') {
                return Some(MatchResult {
                    rule: "missing_space",
                    corrected_command: format!("cd {}", rest),
                    similarity: util::SIMILARITY_TYPO,
                });
            }
        }
    }

    None
}

pub fn wrong_hyphen_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() != 1 {
        return None;
    }

    let raw = &command.parts[0];

    // Find the last hyphen that could split a known command from its subcommand
    for cmd in KNOWN_COMMANDS {
        if let Some(rest) = raw.strip_prefix(&format!("{}-", cmd)) {
            if !rest.is_empty() && !rest.contains(' ') {
                return Some(MatchResult {
                    rule: "wrong_hyphen",
                    corrected_command: format!("{} {}", cmd, rest),
                    similarity: util::SIMILARITY_TYPO,
                });
            }
        }
    }

    None
}
