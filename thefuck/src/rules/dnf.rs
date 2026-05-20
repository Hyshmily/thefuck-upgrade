use crate::types::{Command, MatchResult};
use crate::util;

const DNF_SUBCOMMANDS: &[&str] = &[
    "install",
    "remove",
    "update",
    "upgrade",
    "search",
    "info",
    "list",
    "clean",
    "autoremove",
    "check-update",
    "provides",
    "history",
    "group",
    "module",
    "repolist",
    "repoquery",
    "download",
    "builddep",
    "config-manager",
    "copr",
    "distro-sync",
    "downgrade",
    "reinstall",
    "swap",
    "system-upgrade",
    "versionlock",
    "help",
];

const DNF_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall"]),
    ("remove", &["remov", "remoe"]),
    ("update", &["udpate", "updtae"]),
    ("search", &["serch", "searh"]),
    ("upgrade", &["upgarde", "upgade"]),
    ("history", &["histroy", "histoy"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, DNF_SUBCOMMANDS, DNF_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn dnf_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "dn" | "dnff" | "dng" => "dnf",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "dnf_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn dnf_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "dnf" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || DNF_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "dnf_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
