use crate::rules::helpers;
use crate::types::{Command, MatchResult};
use crate::util;

const PIP_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall", "insatll"]),
    ("uninstall", &["unistall", "uninstal", "unintsall"]),
    ("freeze", &["freze", "freee", "freese"]),
    ("list", &["lits", "lst", "lis"]),
    ("show", &["shwo", "shw", "sow"]),
    ("check", &["chekc", "chec", "chcek"]),
    ("download", &["dwonload", "downlaod", "dwnload"]),
];

fn is_pip_bin(command: &Command) -> Option<&str> {
    match command.parts.first()?.as_str() {
        "pip" | "pip3" => Some(command.parts[0].as_str()),
        _ => None,
    }
}

pub fn python_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let corrected_bin = match command.parts[0].as_str() {
        "pyhton" | "pyton" | "puthon" => "python",
        _ => return None,
    };

    Some(MatchResult {
        rule: "python_command",
        corrected_command: helpers::replace_first(&command.parts, corrected_bin),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn pip_to_uv_rule(command: &Command) -> Option<MatchResult> {
    let pip_bin = is_pip_bin(command)?;

    let modernizable = command.parts.get(1).is_some_and(|s| {
        matches!(
            s.as_str(),
            "install" | "uninstall" | "list" | "freeze" | "show" | "check"
        )
    });

    if !modernizable {
        return None;
    }

    Some(MatchResult {
        rule: "python_pip_to_uv",
        corrected_command: helpers::prepend(&command.parts[1..], &["uv", pip_bin]),
        similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
    })
}

pub fn pip_to_python_module_rule(command: &Command) -> Option<MatchResult> {
    let pip_bin = is_pip_bin(command)?;

    Some(MatchResult {
        rule: "python_pip_module",
        corrected_command: helpers::prepend(&command.parts[1..], &["python", "-m", pip_bin]),
        similarity: util::SIMILARITY_UPSTREAM,
    })
}

pub fn pip_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 {
        return None;
    }

    let _pip_bin = is_pip_bin(command)?;

    for &(correct, typos) in PIP_SUBCOMMAND_TYPOS {
        if typos.contains(&command.parts[1].as_str()) {
            return Some(MatchResult {
                rule: "pip_subcommand_typo",
                corrected_command: helpers::replace_part(&command.parts, 1, correct),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
