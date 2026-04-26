use crate::types::{Command, MatchResult};
use crate::util;

const APT_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall", "insatll"]),
    ("update", &["udpate", "updtae", "updte"]),
    ("upgrade", &["upgarde", "upgard", "upgade"]),
    ("remove", &["romve", "remov", "remoe"]),
    ("search", &["searc", "serch", "searh"]),
    ("list", &["lits", "lst", "lis"]),
    ("autoremove", &["autormove", "autoremov", "autoremoe"]),
];

pub fn apt_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "atp" | "aptt" => Some("apt"),
        "aptget" | "atp-get" => Some("apt-get"),
        _ => None,
    }?;

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "apt_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn apt_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 {
        return None;
    }

    let is_apt = matches!(command.parts[0].as_str(), "apt" | "apt-get");
    if !is_apt {
        return None;
    }

    for &(correct, typos) in APT_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "apt_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}

pub fn apt_get_to_apt_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "apt-get" {
        return None;
    }

    let mut corrected = vec!["apt".to_string()];
    corrected.extend(command.parts.iter().skip(1).cloned());

    Some(MatchResult {
        rule: "apt_get_to_apt",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_MIGRATION,
    })
}
