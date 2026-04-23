use crate::types::{Command, MatchResult};

pub fn git_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "gti" | "gitt" | "gut" => Some("git"),
        _ => None,
    }?;

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "git_command".to_string(),
        corrected_command: corrected.join(" "),
        similarity: 0.97,
    })
}

pub fn git_push_upstream_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() != 3 || command.parts[0] != "git" || command.parts[1] != "push" {
        return None;
    }

    let branch = &command.parts[2];
    if branch.starts_with('-') {
        return None;
    }

    Some(MatchResult {
        rule: "git_push_upstream".to_string(),
        corrected_command: format!("git push --set-upstream origin {branch}"),
        similarity: 0.9,
    })
}

pub fn git_force_with_lease_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 3 || command.parts[0] != "git" || command.parts[1] != "push" {
        return None;
    }

    let force_index = command.parts.iter().position(|part| part == "--force")?;
    let mut corrected = command.parts.clone();
    corrected[force_index] = "--force-with-lease".to_string();

    Some(MatchResult {
        rule: "git_force_with_lease".to_string(),
        corrected_command: corrected.join(" "),
        similarity: 0.93,
    })
}
