use crate::types::{Command, MatchResult};

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
        similarity: 0.96,
    })
}
