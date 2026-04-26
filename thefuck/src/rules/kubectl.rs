use crate::types::{Command, MatchResult};
use crate::util;

const KUBECTL_SUBCOMMANDS: &[&str] = &[
    "annotate",
    "api-resources",
    "api-versions",
    "apply",
    "attach",
    "auth",
    "autoscale",
    "certificate",
    "cluster-info",
    "completion",
    "config",
    "cordon",
    "cp",
    "create",
    "delete",
    "describe",
    "diff",
    "drain",
    "edit",
    "exec",
    "explain",
    "expose",
    "get",
    "help",
    "kustomize",
    "label",
    "logs",
    "options",
    "patch",
    "plugin",
    "port-forward",
    "proxy",
    "replace",
    "rollout",
    "run",
    "scale",
    "secrets",
    "set",
    "taint",
    "top",
    "uncordon",
    "version",
    "wait",
];

const KUBECTL_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("apply", &["aplpy", "appl", "app y"]),
    ("describe", &["describ", "descrbe", "descrie"]),
    ("delete", &["delte", "delet", "delee"]),
    ("create", &["cretate", "creat", "crate"]),
    ("get", &["gett", "gt", "ge"]),
    ("exec", &["exc", "exe", "exce"]),
    ("logs", &["log", "lgs"]),
    ("edit", &["edt", "edi"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(
        arg,
        KUBECTL_SUBCOMMANDS,
        KUBECTL_SUBCOMMAND_TYPOS,
        THRESHOLD,
    )
}

pub fn kubectl_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "kubecttl" | "kubctl" | "kuberctl" | "kubcntl" => "kubectl",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "kubectl_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn kubectl_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "kubectl" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || KUBECTL_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "kubectl_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
