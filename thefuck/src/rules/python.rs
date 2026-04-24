use crate::types::{Command, MatchResult};
use crate::util;

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
        rule: "python_command".to_string(),
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn pip_to_uv_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let pip_bin = match command.parts[0].as_str() {
        "pip" | "pip3" => command.parts[0].as_str(),
        _ => return None,
    };

    let modernizable = command
        .parts
        .get(1)
        .map(|s| s.as_str())
        .map_or(false, |sub| {
            matches!(
                sub,
                "install" | "uninstall" | "list" | "freeze" | "show" | "check"
            )
        });

    if !modernizable {
        return None;
    }

    let mut corrected = vec!["uv".to_string(), pip_bin.to_string()];
    corrected.extend(command.parts.iter().skip(1).cloned());

    Some(MatchResult {
        rule: "python_pip_to_uv".to_string(),
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_SUBCOMMAND_TYPO,
    })
}

pub fn pip_to_python_module_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let pip_bin = match command.parts[0].as_str() {
        "pip" | "pip3" => command.parts[0].as_str(),
        _ => return None,
    };

    let mut corrected = vec!["python".to_string(), "-m".to_string(), pip_bin.to_string()];
    corrected.extend(command.parts.iter().skip(1).cloned());

    Some(MatchResult {
        rule: "python_pip_module".to_string(),
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_UPSTREAM,
    })
}
