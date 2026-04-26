use crate::types::{Command, MatchResult};
use crate::util;

const DOCKER_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("images", &["imags", "imges", "imagse"]),
    ("container", &["contianer", "containr", "conainer"]),
    ("volume", &["voluem", "volum", "volue"]),
    ("network", &["ntework", "networ", "netwok"]),
    ("compose", &["compos", "compse", "compoe"]),
    ("build", &["buid", "buld", "bluid"]),
    ("push", &["psuh", "pus"]),
    ("pull", &["pul", "pll"]),
];

pub fn docker_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "dcoker" | "dokcer" | "dockr" | "doker" => "docker",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "docker_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn docker_compose_v2_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() || command.parts[0] != "docker-compose" {
        return None;
    }

    let mut corrected = vec!["docker".to_string(), "compose".to_string()];
    corrected.extend(command.parts.iter().skip(1).cloned());

    Some(MatchResult {
        rule: "docker_compose_v2",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_MIGRATION,
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
        rule: "docker_legacy_management",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_LEGACY,
    })
}

pub fn docker_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "docker" {
        return None;
    }

    for &(correct, typos) in DOCKER_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "docker_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
