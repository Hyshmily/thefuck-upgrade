use crate::types::{Command, MatchResult};
use crate::util;

const WINGET_SUBCOMMANDS: &[&str] = &[
    "install",
    "uninstall",
    "upgrade",
    "list",
    "search",
    "show",
    "settings",
    "source",
    "hash",
    "validate",
    "help",
    "export",
    "import",
    "pin",
    "download",
    "repair",
    "configure",
];

const WINGET_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall"]),
    ("uninstall", &["unistall", "uninstal"]),
    ("upgrade", &["upgarde", "upgade"]),
    ("search", &["serch", "searh"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, WINGET_SUBCOMMANDS, WINGET_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn winget_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "wingt" | "wnget" | "winge" => "winget",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "winget_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn winget_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "winget" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || WINGET_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "winget_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
