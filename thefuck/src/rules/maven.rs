use crate::types::{Command, MatchResult};
use crate::util;

const MAVEN_PHASES: &[&str] = &[
    // Default lifecycle
    "validate",
    "initialize",
    "generate-sources",
    "process-sources",
    "generate-resources",
    "process-resources",
    "compile",
    "process-classes",
    "generate-test-sources",
    "process-test-sources",
    "generate-test-resources",
    "process-test-resources",
    "test-compile",
    "process-test-classes",
    "test",
    "prepare-package",
    "package",
    "pre-integration-test",
    "integration-test",
    "post-integration-test",
    "verify",
    "install",
    "deploy",
    // Clean lifecycle
    "pre-clean",
    "clean",
    "post-clean",
    // Site lifecycle
    "pre-site",
    "site",
    "post-site",
    "site-deploy",
];

const MAVEN_TYPO_DICT: &[(&str, &[&str])] = &[
    ("clean", &["clea", "clen", "cleaan"]),
    ("compile", &["complie", "comple", "compil"]),
    ("test", &["tst", "tes", "tets"]),
    ("install", &["instlal", "instal", "intsall"]),
    ("deploy", &["depoy", "deplo"]),
    ("package", &["pacakge", "pakage", "packge"]),
    ("verify", &["verfiy", "verfy", "veriy"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, MAVEN_PHASES, MAVEN_TYPO_DICT, THRESHOLD)
}

pub fn mvn_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "mvnm" | "mnv" => Some("mvn"),
        _ => None,
    }?;

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "mvn_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn mvn_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "mvn" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || MAVEN_PHASES.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_phase, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_phase;

    Some(MatchResult {
        rule: "mvn_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}

pub fn mvn_multiphase_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "mvn" {
        return None;
    }

    for i in 1..command.parts.len() {
        let arg = &command.parts[i];
        if arg.starts_with('-') || MAVEN_PHASES.contains(&arg.as_str()) {
            continue;
        }

        if let Some((corrected_phase, similarity)) = find_match(arg) {
            let mut corrected = command.parts.clone();
            corrected[i] = corrected_phase;
            return Some(MatchResult {
                rule: "mvn_multiphase_typo",
                corrected_command: corrected.join(" "),
                similarity,
            });
        }
    }

    None
}
