use crate::types::{Command, MatchResult};
use crate::util;

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
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn git_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "git" {
        return None;
    }

    let replacement = match command.parts[1].as_str() {
        "statsu" | "stauts" => Some("status"),
        "comit" | "cmomit" => Some("commit"),
        "brnch" | "branck" => Some("branch"),
        "pul" => Some("pull"),
        "psuh" | "pus" => Some("push"),
        _ => None,
    }?;

    let mut corrected = command.parts.clone();
    corrected[1] = replacement.to_string();

    Some(MatchResult {
        rule: "git_subcommand_typo".to_string(),
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
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
        similarity: util::SIMILARITY_UPSTREAM,
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
        similarity: util::SIMILARITY_FORCE,
    })
}

pub fn git_checkout_to_switch_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 3 || command.parts[0] != "git" || command.parts[1] != "checkout" {
        return None;
    }

    if command.parts[2] == "-b" {
        if command.parts.len() < 4 {
            return None;
        }

        let mut corrected = vec!["git".to_string(), "switch".to_string(), "-c".to_string()];
        corrected.extend(command.parts.iter().skip(3).cloned());
        return Some(MatchResult {
            rule: "git_checkout_to_switch".to_string(),
            corrected_command: corrected.join(" "),
            similarity: 0.95,
        });
    }

    if command.parts[2].starts_with('-') {
        return None;
    }

    let mut corrected = vec!["git".to_string(), "switch".to_string()];
    corrected.extend(command.parts.iter().skip(2).cloned());

    Some(MatchResult {
        rule: "git_checkout_to_switch".to_string(),
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_BRANCH,
    })
}
