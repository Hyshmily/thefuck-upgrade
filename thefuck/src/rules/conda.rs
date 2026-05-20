use crate::rules::helpers;
use crate::types::{Command, MatchResult};
use crate::util;

const CONDA_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall", "insatll"]),
    ("remove", &["romve", "remov", "remoe"]),
    ("list", &["lits", "lst", "lis"]),
    ("create", &["cretate", "creat", "crate"]),
    ("activate", &["acitvate", "activte", "actiate"]),
    ("deactivate", &["decativate", "deactivte", "deactiate"]),
    ("update", &["udpate", "updtae", "updte"]),
    ("search", &["searc", "serch", "searh"]),
];

pub fn conda_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "cnda" | "cond a" | "codna" => "conda",
        _ => return None,
    };

    Some(MatchResult {
        rule: "conda_command",
        corrected_command: helpers::replace_first(&command.parts, replacement),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn conda_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "conda" {
        return None;
    }

    for &(correct, typos) in CONDA_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            return Some(MatchResult {
                rule: "conda_subcommand_typo",
                corrected_command: helpers::replace_part(&command.parts, 1, correct),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
