use crate::types::{Command, MatchResult};
use crate::util;

const NPM_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall", "insatll"]),
    ("uninstall", &["unistall", "uninstal", "unintsall"]),
    ("update", &["udpate", "updtae", "updte"]),
    ("init", &["iniit", "int", "inti"]),
    ("start", &["satrt", "strt", "star"]),
    ("test", &["tets", "tst", "tes"]),
    ("run", &["rnu", "rn", "ru"]),
    ("build", &["bulid", "buid", "bluid"]),
    ("publish", &["publis", "publsh", "pubish"]),
];

fn is_npm_like(bin: &str) -> bool {
    matches!(bin, "npm" | "yarn" | "pnpm" | "npx")
}

fn npm_like_typo(bin: &str) -> Option<&str> {
    match bin {
        "npn" | "pnm" => Some("npm"),
        "yrn" | "yar" => Some("yarn"),
        "ppnm" | "pnmp" => Some("pnpm"),
        _ => None,
    }
}

pub fn npm_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = npm_like_typo(&command.parts[0])?;

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "npm_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn npm_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || !is_npm_like(&command.parts[0]) {
        return None;
    }

    for &(correct, typos) in NPM_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "npm_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
