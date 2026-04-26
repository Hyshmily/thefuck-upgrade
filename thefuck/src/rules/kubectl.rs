use crate::types::{Command, MatchResult};
use crate::util;

const KUBECTL_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("apply", &["aplpy", "appl", "app y"]),
    ("describe", &["describ", "descrbe", "descrie"]),
    ("delete", &["delte", "delet", "delee"]),
    ("create", &["cretate", "creat", "crate"]),
    ("get", &["gett", "gt", "ge"]),
    ("exec", &["exc", "exe", "exce"]),
    ("logs", &["log", "lgs"]),
    ("edit", &["edt", "edi"]),
];

pub fn kubectl_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "kubecttl" | "kubctl" | "kuberctl" | "kubcntl" => "kubectl",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "kubectl_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn kubectl_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "kubectl" {
        return None;
    }

    for &(correct, typos) in KUBECTL_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "kubectl_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
