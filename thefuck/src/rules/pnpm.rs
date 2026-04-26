use crate::types::{Command, MatchResult};
use crate::util;

const PNPM_SUBCOMMANDS: &[&str] = &[
    "install",
    "add",
    "remove",
    "update",
    "run",
    "test",
    "build",
    "start",
    "dev",
    "list",
    "outdated",
    "why",
    "dlx",
    "exec",
    "init",
    "link",
    "unlink",
    "patch",
    "patch-remove",
    "publish",
    "audit",
    "approve-builds",
    "config",
    "root",
    "bin",
    "store",
    "server",
    "recursive",
    "rebuild",
    "prune",
    "fetch",
    "setup",
    "doctor",
    "help",
];

const PNPM_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall", "insatll"]),
    ("add", &["ad", "addd"]),
    ("remove", &["remov", "remoe", "rmove"]),
    ("update", &["udpate", "updtae"]),
    ("run", &["rnu", "rn"]),
    ("test", &["tst", "tes", "tets"]),
    ("build", &["buid", "buld"]),
    ("init", &["iniit", "int"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, PNPM_SUBCOMMANDS, PNPM_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn pnpm_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "pnmp" | "ppnm" | "pnp" | "pmnp" => "pnpm",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "pnpm_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn pnpm_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "pnpm" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || PNPM_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "pnpm_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
