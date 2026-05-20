use crate::types::{Command, MatchResult};
use crate::util;

const BUN_SUBCOMMANDS: &[&str] = &[
    "add", "build", "create", "dev", "help", "init", "install", "link", "outdated", "pm", "remove",
    "run", "start", "test", "unlink", "update", "upgrade", "x", "patch", "publish", "version",
    "watch",
];

const BUN_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall"]),
    ("add", &["ad", "addd"]),
    ("remove", &["remov", "remoe"]),
    ("run", &["rnu", "rn"]),
    ("test", &["tst", "tes"]),
    ("build", &["buid", "buld"]),
    ("init", &["iniit", "int"]),
    ("create", &["creat", "cretate"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, BUN_SUBCOMMANDS, BUN_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn bun_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "bnu" | "bum" | "ubn" => "bun",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "bun_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn bun_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "bun" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || BUN_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "bun_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
