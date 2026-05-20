use crate::rules::helpers;
use crate::types::{Command, MatchResult};
use crate::util;

const BREW_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall", "insatll"]),
    ("uninstall", &["unistall", "uninstal", "unintsall"]),
    ("update", &["udpate", "updtae", "updte"]),
    ("upgrade", &["upgarde", "upgard", "upgade"]),
    ("doctor", &["docto", "doctr", "dctor"]),
    ("clean", &["clena", "clea", "clen"]),
    ("search", &["searc", "serch", "searh"]),
    ("info", &["inof", "inf", "ifo"]),
    ("list", &["lits", "lst", "lis"]),
];

pub fn brew_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "berw" | "brwe" | "bew" => "brew",
        _ => return None,
    };

    Some(MatchResult {
        rule: "brew_command",
        corrected_command: helpers::replace_first(&command.parts, replacement),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn brew_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "brew" {
        return None;
    }

    for &(correct, typos) in BREW_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            return Some(MatchResult {
                rule: "brew_subcommand_typo",
                corrected_command: helpers::replace_part(&command.parts, 1, correct),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
