use crate::types::{Command, MatchResult};
use crate::util;

const GO_SUBCOMMANDS: &[&str] = &[
    "build", "clean", "doc", "env", "fix", "fmt", "generate", "get", "install", "list", "mod",
    "work", "run", "test", "tool", "version", "vet", "bug", "cache", "help",
];

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

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, GO_SUBCOMMANDS, GO_SUBCOMMAND_TYPOS, THRESHOLD)
}

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

    let arg = &command.parts[1];
    if arg.starts_with('-') || GO_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "go_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
