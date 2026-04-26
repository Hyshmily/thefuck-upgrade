use crate::types::{Command, MatchResult};
use crate::util;

const MAVEN_SUBCOMMANDS: &[(&str, &[&str])] = &[
    ("clean", &["clea", "clen", "cleaan"]),
    ("compile", &["complie", "comple", "compil"]),
    ("test", &["tst", "tes"]),
    ("package", &["pacakge", "pakage", "packge"]),
    ("install", &["instlal", "instal", "intsall"]),
    ("deploy", &["depoy", "deplo"]),
    ("validate", &["validae", "validae"]),
];

pub fn mvn_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "mvnm" | "mnv" => Some("mvn"),
        _ => None,
    }?;

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "mvn_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn mvn_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "mvn" {
        return None;
    }

    for &(correct, typos) in MAVEN_SUBCOMMANDS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "mvn_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
