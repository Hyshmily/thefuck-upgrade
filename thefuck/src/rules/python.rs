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

    let mut corrected = command.parts.clone();
    corrected[0] = corrected_bin.to_string();

    Some(MatchResult {
        rule: "python_command",
        corrected_command: corrected.join(" "),
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

    let mut corrected = vec!["uv".to_string(), pip_bin.to_string()];
    corrected.extend(command.parts.iter().skip(1).cloned());

    Some(MatchResult {
        rule: "python_pip_to_uv",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
    })
}

pub fn pip_to_python_module_rule(command: &Command) -> Option<MatchResult> {
    let pip_bin = is_pip_bin(command)?;

    let mut corrected = vec!["python".to_string(), "-m".to_string(), pip_bin.to_string()];
    corrected.extend(command.parts.iter().skip(1).cloned());

    Some(MatchResult {
        rule: "python_pip_module",
        corrected_command: corrected.join(" "),
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
            let mut corrected = command.parts.clone();
            corrected[1] = correct.to_string();

            return Some(MatchResult {
                rule: "pip_subcommand_typo",
                corrected_command: corrected.join(" "),
                similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
            });
        }
    }

    None
}
