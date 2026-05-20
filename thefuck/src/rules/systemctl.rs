use crate::types::{Command, MatchResult};
use crate::util;

const SYSTEMCTL_SUBCOMMANDS: &[&str] = &[
    "start",
    "stop",
    "restart",
    "reload",
    "enable",
    "disable",
    "status",
    "mask",
    "unmask",
    "is-active",
    "is-enabled",
    "is-failed",
    "show",
    "cat",
    "edit",
    "list-units",
    "list-unit-files",
    "list-dependencies",
    "list-sockets",
    "list-timers",
    "daemon-reload",
    "get-default",
    "set-default",
    "isolate",
    "kill",
    "freeze",
    "thaw",
    "clean",
    "reset-failed",
    "list-jobs",
    "cancel",
    "reenable",
    "preset",
    "link",
    "revert",
    "add-wants",
    "add-requires",
];

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

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(
        arg,
        SYSTEMCTL_SUBCOMMANDS,
        SYSTEMCTL_SUBCOMMAND_TYPOS,
        THRESHOLD,
    )
}

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

    let arg = &command.parts[1];
    if arg.starts_with('-') || SYSTEMCTL_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "systemctl_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
