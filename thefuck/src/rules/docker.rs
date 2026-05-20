use crate::rules::helpers;
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

    Some(MatchResult {
        rule: "docker_command",
        corrected_command: helpers::replace_first(&command.parts, replacement),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn docker_compose_v2_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() || command.parts[0] != "docker-compose" {
        return None;
    }

    Some(MatchResult {
        rule: "docker_compose_v2",
        corrected_command: helpers::prepend(&command.parts[1..], &["docker", "compose"]),
        similarity: util::SIMILARITY_MIGRATION,
    })
}

pub fn docker_legacy_management_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "docker" {
        return None;
    }

    let corrected_command = match command.parts[1].as_str() {
        "images" => helpers::prepend(&command.parts[2..], &["docker", "image", "ls"]),
        "ps" => helpers::prepend(&command.parts[2..], &["docker", "container", "ls"]),
        _ => return None,
    };

    Some(MatchResult {
        rule: "docker_legacy_management",
        corrected_command,
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
    Some(MatchResult {
        rule: "docker_subcommand_typo",
        corrected_command: helpers::replace_part(&command.parts, 1, &corrected_sub),
        similarity,
    })
}
