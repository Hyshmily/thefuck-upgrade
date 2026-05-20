use crate::types::{Command, MatchResult};
use crate::util;

const TERRAFORM_SUBCOMMANDS: &[&str] = &[
    "apply",
    "console",
    "destroy",
    "fmt",
    "force-unlock",
    "get",
    "graph",
    "import",
    "init",
    "login",
    "logout",
    "metadata",
    "output",
    "plan",
    "providers",
    "refresh",
    "show",
    "state",
    "taint",
    "test",
    "untaint",
    "validate",
    "version",
    "workspace",
];

const TERRAFORM_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("apply", &["aplpy", "appl", "app y"]),
    ("plan", &["plna", "plan", "pla"]),
    ("init", &["int", "ini"]),
    ("destroy", &["destory", "destoy", "destroi"]),
    ("output", &["outptu", "outpu", "outpt"]),
    ("validate", &["valdiate", "validte", "vaildate"]),
    ("refresh", &["refrsh", "refesh", "refrehs"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(
        arg,
        TERRAFORM_SUBCOMMANDS,
        TERRAFORM_SUBCOMMAND_TYPOS,
        THRESHOLD,
    )
}

pub fn terraform_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "terrafrom" | "terrafor" | "terrform" => "terraform",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "terraform_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn terraform_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "terraform" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || TERRAFORM_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "terraform_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
