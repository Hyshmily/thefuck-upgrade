use crate::types::{Command, MatchResult};
use crate::util;

const CHOCO_SUBCOMMANDS: &[&str] = &[
    "install",
    "uninstall",
    "upgrade",
    "list",
    "search",
    "info",
    "outdated",
    "pin",
    "unpin",
    "pack",
    "push",
    "new",
    "download",
    "sources",
    "config",
    "feature",
    "apikey",
    "setapikey",
    "help",
];

const CHOCO_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall"]),
    ("uninstall", &["unistall", "uninstal"]),
    ("upgrade", &["upgarde", "upgade"]),
    ("search", &["serch", "searh"]),
    ("list", &["lits", "lis"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, CHOCO_SUBCOMMANDS, CHOCO_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn choco_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "choc" | "chco" | "chcoolatey" => "choco",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "choco_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn choco_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "choco" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || CHOCO_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "choco_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
