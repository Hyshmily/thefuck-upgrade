use crate::rules::helpers;
use crate::types::{Command, MatchResult};
use crate::util;

const CARGO_SUBCOMMANDS: &[&str] = &[
    "build",
    "check",
    "clean",
    "clippy",
    "doc",
    "new",
    "init",
    "add",
    "remove",
    "run",
    "test",
    "bench",
    "update",
    "search",
    "publish",
    "install",
    "uninstall",
    "fmt",
    "fix",
    "generate",
    "package",
    "rustc",
    "rustdoc",
    "vendor",
    "verify",
    "version",
    "metadata",
    "config",
    "fetch",
    "help",
];

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

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, CARGO_SUBCOMMANDS, CARGO_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn cargo_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "carg" | "cagro" | "carog" | "crgo" => "cargo",
        _ => return None,
    };

    Some(MatchResult {
        rule: "cargo_command",
        corrected_command: helpers::replace_first(&command.parts, replacement),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn cargo_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "cargo" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || CARGO_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    Some(MatchResult {
        rule: "cargo_subcommand_typo",
        corrected_command: helpers::replace_part(&command.parts, 1, &corrected_sub),
        similarity,
    })
}
