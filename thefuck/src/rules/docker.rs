use crate::types::{Command, MatchResult};
use crate::util;

const DOCKER_SUBCOMMANDS: &[&str] = &[
    "attach",
    "build",
    "commit",
    "compose",
    "config",
    "container",
    "context",
    "cp",
    "create",
    "diff",
    "events",
    "exec",
    "export",
    "history",
    "image",
    "images",
    "import",
    "info",
    "inspect",
    "kill",
    "load",
    "login",
    "logout",
    "logs",
    "manifest",
    "network",
    "node",
    "pause",
    "plugin",
    "port",
    "ps",
    "pull",
    "push",
    "rename",
    "restart",
    "rm",
    "rmi",
    "run",
    "save",
    "search",
    "secret",
    "service",
    "stack",
    "start",
    "stats",
    "stop",
    "swarm",
    "system",
    "tag",
    "top",
    "trust",
    "unpause",
    "update",
    "version",
    "volume",
    "wait",
    "builder",
    "buildx",
    "scan",
    "scout",
];

const DOCKER_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("images", &["imags", "imges", "imagse"]),
    ("container", &["contianer", "containr", "conainer"]),
    ("volume", &["voluem", "volum", "volue"]),
    ("network", &["ntework", "networ", "netwok"]),
    ("compose", &["compos", "compse", "compoe"]),
    ("build", &["buid", "buld", "bluid"]),
    ("push", &["psuh", "pus"]),
    ("pull", &["pul", "pll"]),
    ("exec", &["exc", "exe"]),
    ("restart", &["restar", "restrt"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, DOCKER_SUBCOMMANDS, DOCKER_SUBCOMMAND_TYPOS, THRESHOLD)
}

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

    let arg = &command.parts[1];
    if arg.starts_with('-') || DOCKER_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "docker_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
