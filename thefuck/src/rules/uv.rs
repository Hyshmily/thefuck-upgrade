use crate::rules::helpers;
use crate::types::{Command, MatchResult};
use crate::util;

const UV_SUBCOMMANDS: &[&str] = &[
    "add",
    "auth",
    "build",
    "cache",
    "clean",
    "completions",
    "config",
    "help",
    "init",
    "install",
    "lock",
    "pip",
    "publish",
    "python",
    "remove",
    "run",
    "self",
    "sync",
    "tool",
    "toolchain",
    "tree",
    "venv",
    "version",
    "workspace",
];

const UV_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall"]),
    ("add", &["ad", "addd"]),
    ("remove", &["remov", "remoe"]),
    ("run", &["rnu", "rn"]),
    ("sync", &["snyc", "snc"]),
    ("lock", &["lck", "loc"]),
    ("init", &["iniit", "int"]),
    ("build", &["buid", "buld"]),
    ("venv", &["vevn", "vnev"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, UV_SUBCOMMANDS, UV_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn uv_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "vu" | "ub" => "uv",
        _ => return None,
    };

    Some(MatchResult {
        rule: "uv_command",
        corrected_command: helpers::replace_first(&command.parts, replacement),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn uv_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "uv" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || UV_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    Some(MatchResult {
        rule: "uv_subcommand_typo",
        corrected_command: helpers::replace_part(&command.parts, 1, &corrected_sub),
        similarity,
    })
}
