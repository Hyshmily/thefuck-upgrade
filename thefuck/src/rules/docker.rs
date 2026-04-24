use crate::types::{Command, MatchResult};

pub fn docker_compose_v2_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() || command.parts[0] != "docker-compose" {
        return None;
    }

    let mut corrected = vec!["docker".to_string(), "compose".to_string()];
    corrected.extend(command.parts.iter().skip(1).cloned());

    Some(MatchResult {
        rule: "docker_compose_v2".to_string(),
        corrected_command: corrected.join(" "),
        similarity: 0.94,
    })
}

pub fn docker_legacy_management_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "docker" {
        return None;
    }

    let mut corrected = match command.parts[1].as_str() {
        "images" => vec!["docker".to_string(), "image".to_string(), "ls".to_string()],
        "ps" => vec![
            "docker".to_string(),
            "container".to_string(),
            "ls".to_string(),
        ],
        _ => return None,
    };

    corrected.extend(command.parts.iter().skip(2).cloned());

    Some(MatchResult {
        rule: "docker_legacy_management".to_string(),
        corrected_command: corrected.join(" "),
        similarity: 0.92,
    })
}
