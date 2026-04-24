use crate::types::{Command, MatchResult};
use crate::util;

pub fn sudo_missing_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let suspicious = matches!(
        command.parts[0].as_str(),
        "apt-get" | "apt" | "dnf" | "pacman" | "systemctl"
    );
    if !suspicious {
        return None;
    }

    Some(MatchResult {
        rule: "sudo_missing".to_string(),
        corrected_command: format!("sudo {}", command.raw),
        similarity: util::SIMILARITY_SUDO,
    })
}
