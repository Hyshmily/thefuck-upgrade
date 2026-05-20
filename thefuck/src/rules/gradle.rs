use crate::rules::helpers;
use crate::types::{Command, MatchResult};
use crate::util;

const GRADLE_TASKS: &[&str] = &[
    "build",
    "test",
    "clean",
    "assemble",
    "check",
    "run",
    "bootRun",
    "dependencies",
    "tasks",
    "compileJava",
    "compileTestJava",
    "jar",
    "javadoc",
    "publish",
    "publishToMavenLocal",
    "install",
    "uploadArchives",
    "init",
    "wrapper",
    "projects",
    "properties",
];

const GRADLE_TYPOS: &[(&str, &[&str])] = &[
    ("build", &["buid", "buld", "bluid"]),
    ("test", &["tst", "tes", "tets"]),
    ("clean", &["clea", "clen", "clena"]),
    ("run", &["rnu", "rn"]),
    ("tasks", &["tsaks", "task"]),
];

const THRESHOLD: f64 = 0.75;

fn is_gradle(bin: &str) -> bool {
    matches!(bin, "gradle" | "gradlew")
}

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, GRADLE_TASKS, GRADLE_TYPOS, THRESHOLD)
}

pub fn gradle_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "gradel" | "grdle" | "grdale" => "gradle",
        "gardlew" | "gradlw" => "gradlew",
        _ => return None,
    };

    Some(MatchResult {
        rule: "gradle_command",
        corrected_command: helpers::replace_first(&command.parts, replacement),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn gradle_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || !is_gradle(&command.parts[0]) {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || GRADLE_TASKS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    Some(MatchResult {
        rule: "gradle_subcommand_typo",
        corrected_command: helpers::replace_part(&command.parts, 1, &corrected_sub),
        similarity,
    })
}
