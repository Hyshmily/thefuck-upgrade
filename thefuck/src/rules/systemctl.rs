use crate::types::{Command, MatchResult};
use crate::util;

const SYSTEMCTL_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("start", &["satrt", "strt", "star"]),
    ("stop", &["stpo", "stp", "stopp"]),
    ("restart", &["restar", "restat", "restrt"]),
    ("enable", &["enabel", "enble", "enale"]),
    ("disable", &["dsiable", "disabe", "disble"]),
    ("status", &["statsu", "stauts", "stat"]),
    ("mask", &["maks", "msk"]),
    ("unmask", &["unmsk", "unmak", "unmaks"]),
];

pub fn systemctl_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "systemclt" | "sytemctl" | "sytemclt" | "systemtl" => "systemctl",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "systemctl_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn systemctl_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "systemctl" {
        return None;
    }

    for &(correct, typos) in SYSTEMCTL_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "systemctl_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
