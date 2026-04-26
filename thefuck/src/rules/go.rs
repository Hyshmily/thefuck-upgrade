use crate::types::{Command, MatchResult};
use crate::util;

const GO_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("build", &["buid", "buld", "bluid"]),
    ("test", &["tset", "tets", "tst"]),
    ("run", &["rnu", "rn"]),
    ("fmt", &["frmt", "ftm"]),
    ("mod", &["mdo", "md"]),
    ("install", &["isntall", "instal", "intsall"]),
    ("clean", &["clena", "clea", "clen"]),
    ("vet", &["vte", "ve"]),
    ("get", &["gte", "ge"]),
];

pub fn go_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "og" | "goo" => "go",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "go_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn go_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "go" {
        return None;
    }

    for &(correct, typos) in GO_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "go_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
