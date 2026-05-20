use crate::types::{Command, MatchResult};
use crate::util;

const PACMAN_SUBCOMMANDS: &[&str] = &[
    "-S", "-R", "-Q", "-D", "-U", "-T", "-F", "-V", "-h", "-Si", "-Ss", "-Su", "-Syu", "-Syyu",
    "-Syu", "-Syy", "-Rs", "-Rns", "-Rdd", "-Rn", "-Rsn", "-Qs", "-Qi", "-Ql", "-Qe", "-Qm", "-Qo",
    "-Qdt",
];

const PACMAN_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("-S", &["-s", "- s", "s-"]),
    ("-R", &["-r", "- r", "r-"]),
    ("-Q", &["-q", "- q", "q-"]),
    ("-Syu", &["-syu", "-Suy", "-Syu"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, PACMAN_SUBCOMMANDS, PACMAN_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn pacman_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "pacma" | "pacmn" | "pacamn" | "pacmna" => "pacman",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "pacman_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn pacman_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "pacman" {
        return None;
    }

    let arg = &command.parts[1];
    if PACMAN_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "pacman_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
