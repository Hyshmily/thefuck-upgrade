use crate::types::{Command, MatchResult};
use crate::util;

const AZ_SUBCOMMANDS: &[&str] = &[
    "login",
    "logout",
    "account",
    "group",
    "vm",
    "aks",
    "acr",
    "webapp",
    "functionapp",
    "sql",
    "keyvault",
    "storage",
    "network",
    "cosmosdb",
    "container",
    "appservice",
    "monitor",
    "policy",
    "role",
    "ad",
    "batch",
    "cdn",
    "cognitive",
    "deployment",
    "disk",
    "dns",
    "eventgrid",
    "eventhubs",
    "extension",
    "feature",
    "feedback",
    "find",
    "function",
    "hdinsight",
    "identity",
    "iot",
    "key",
    "kusto",
    "lab",
    "lock",
    "logicapp",
    "managedapp",
    "maps",
    "mysql",
    "postgres",
    "redis",
    "reservations",
    "resource",
    "rest",
    "search",
    "servicebus",
    "sf",
    "sig",
    "signalr",
    "snapshot",
    "spring",
    "sshkey",
    "staticwebapp",
    "synapse",
    "tag",
    "vmss",
    "help",
];

const AZ_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("login", &["lgoin", "logi", "logn"]),
    ("group", &["gropu", "gruop"]),
    ("vm", &["mv", "vmm"]),
    ("aks", &["ask", "akss"]),
    ("acr", &["arc", "accr"]),
    ("storage", &["storge", "storag"]),
];

const THRESHOLD: f64 = 0.70;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, AZ_SUBCOMMANDS, AZ_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn az_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "za" | "azz" => "az",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "az_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn az_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "az" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || AZ_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "az_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
