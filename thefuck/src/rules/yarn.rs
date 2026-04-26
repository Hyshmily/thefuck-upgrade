use crate::types::{Command, MatchResult};
use crate::util;

const YARN_SUBCOMMANDS: &[&str] = &[
    "add",
    "bin",
    "cache",
    "config",
    "constraints",
    "dedupe",
    "dlx",
    "exec",
    "explain",
    "info",
    "init",
    "install",
    "link",
    "login",
    "logout",
    "node",
    "npm",
    "pack",
    "patch",
    "patch-commit",
    "plugin",
    "rebuild",
    "remove",
    "run",
    "search",
    "set",
    "stage",
    "start",
    "test",
    "unlink",
    "unplug",
    "up",
    "upgrade-interactive",
    "version",
    "why",
    "workspace",
    "workspaces",
    "help",
];

const YARN_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall"]),
    ("add", &["ad", "addd"]),
    ("remove", &["remov", "remoe"]),
    ("run", &["rnu", "rn"]),
    ("test", &["tst", "tes"]),
    ("build", &["buid", "buld"]),
    ("init", &["iniit", "int"]),
    ("upgrade", &["upgarde", "upgade"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, YARN_SUBCOMMANDS, YARN_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn yarn_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "yrn" | "yar" | "yarnn" => "yarn",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "yarn_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn yarn_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "yarn" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || YARN_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "yarn_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
