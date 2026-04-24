use crate::types::{Command, MatchResult};

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
        similarity: 0.88,
    })
}
