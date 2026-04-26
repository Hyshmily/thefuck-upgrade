use crate::types::{Command, MatchResult};
use crate::util;

const CARGO_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("build", &["buid", "buld", "bluid"]),
    ("test", &["tset", "tets", "tst"]),
    ("run", &["rnu", "rn"]),
    ("check", &["chekc", "chec", "chcek"]),
    ("clippy", &["clipp", "clipy", "clippi"]),
    ("fmt", &["frmt", "ftm"]),
    ("clean", &["clea", "clen", "clena"]),
    ("install", &["isntall", "instal", "intsall"]),
    ("update", &["udpate", "updtae"]),
    ("doc", &["dco", "dc"]),
];

pub fn cargo_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "carg" | "cagro" | "carog" | "crgo" => "cargo",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "cargo_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn cargo_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "cargo" {
        return None;
    }

    for &(correct, typos) in CARGO_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "cargo_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
