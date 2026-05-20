use crate::rules::helpers;
use crate::types::{Command, MatchResult};
use crate::util;

pub fn common_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "sl" => Some("ls"),
        "gerp" | "grpe" => Some("grep"),
        "mkae" => Some("make"),
        "ehco" => Some("echo"),
        "chomd" => Some("chmod"),
        "chwon" => Some("chown"),
        "clera" | "claer" => Some("clear"),
        "hsitory" => Some("history"),
        "exir" | "eixt" => Some("exit"),
        "iex" => Some("eval"),
        "evla" | "veal" => Some("eval"),
        "tuch" | "touh" => Some("touch"),
        "mrdir" => Some("mkdir"),
        "vom" | "vmi" | "ivm" => Some("vim"),
        "fid" | "fnd" => Some("find"),
        "pc" => Some("cp"),
        "cta" => Some("cat"),
        "mvv" => Some("mv"),
        "rmr" => Some("rm"),
        "man1" | "ma n" => None, // handled elsewhere or ambiguous
        _ => None,
    }?;

    Some(MatchResult {
        rule: "common_typo",
        corrected_command: helpers::replace_first(&command.parts, replacement),
        similarity: util::SIMILARITY_TYPO,
    })
}
