use crate::types::{Command, MatchResult};
use crate::util;

const PIP_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall", "insatll"]),
    ("uninstall", &["unistall", "uninstal", "unintsall"]),
    ("freeze", &["freze", "freee", "freese"]),
    ("list", &["lits", "lst", "lis"]),
    ("show", &["shwo", "shw", "sow"]),
    ("check", &["chekc", "chec", "chcek"]),
    ("download", &["dwonload", "downlaod", "dwnload"]),
];

fn is_pip_bin(bin: &str) -> bool {
    matches!(bin, "pip" | "pip3" | "pip2")
}

pub fn pip_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || !is_pip_bin(&command.parts[0]) {
        return None;
    }

    for &(correct, typos) in PIP_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "pip_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}

pub fn pip_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "ppi" | "pip" => None, // "pip" is already correct
        "pi" => Some("pip"),
        _ => None,
    }?;

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "pip_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}
