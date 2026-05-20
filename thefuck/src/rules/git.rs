use crate::types::{Command, MatchResult};
use crate::util;

const GIT_SUBCOMMANDS: &[&str] = &[
    "add",
    "am",
    "apply",
    "archive",
    "bisect",
    "blame",
    "branch",
    "bundle",
    "checkout",
    "cherry-pick",
    "citool",
    "clean",
    "clone",
    "commit",
    "config",
    "describe",
    "diff",
    "difftool",
    "fetch",
    "format-patch",
    "fsck",
    "gc",
    "gitk",
    "grep",
    "gui",
    "init",
    "instaweb",
    "log",
    "maintenance",
    "merge",
    "mergetool",
    "mv",
    "notes",
    "pull",
    "push",
    "range-diff",
    "rebase",
    "reflog",
    "remote",
    "repack",
    "replace",
    "request-pull",
    "reset",
    "restore",
    "revert",
    "rm",
    "shortlog",
    "show",
    "show-branch",
    "sparse-checkout",
    "stash",
    "status",
    "submodule",
    "switch",
    "tag",
    "whatchanged",
    "worktree",
];

const GIT_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("status", &["statsu", "stauts", "sttus"]),
    ("commit", &["comit", "cmomit", "commti"]),
    ("branch", &["brnch", "branck", "branc"]),
    ("pull", &["pul", "pll"]),
    ("push", &["psuh", "pus", "puh"]),
    ("stash", &["stahs", "stsh", "stas"]),
    ("merge", &["mrege", "merg", "mege"]),
    ("diff", &["dff", "dif"]),
    ("log", &["lg"]),
    ("switch", &["swtich", "swich"]),
    ("restore", &["restroe", "restor"]),
    ("rebase", &["rebaes", "rebas"]),
    ("cherry-pick", &["cherr-pick", "cherypick", "cherrypick"]),
    ("fetch", &["fet ch", "feth", "ftch"]),
    ("add", &["ad", "addd"]),
    ("clone", &["cloen", "clne"]),
    ("init", &["inti", "int"]),
    ("reset", &["resat", "rest"]),
    ("remote", &["remtoe", "remot"]),
    ("checkout", &["chekcout", "checkut"]),
];

const THRESHOLD: f64 = 0.75;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, GIT_SUBCOMMANDS, GIT_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn git_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "gti" | "gitt" | "gut" => Some("git"),
        _ => None,
    }?;

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "git_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn git_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "git" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || GIT_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "git_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}

pub fn git_push_upstream_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() != 3 || command.parts[0] != "git" || command.parts[1] != "push" {
        return None;
    }

    let branch = &command.parts[2];
    if branch.starts_with('-') {
        return None;
    }

    Some(MatchResult {
        rule: "git_push_upstream",
        corrected_command: format!("git push --set-upstream origin {branch}"),
        similarity: util::SIMILARITY_UPSTREAM,
    })
}

pub fn git_force_with_lease_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 3 || command.parts[0] != "git" || command.parts[1] != "push" {
        return None;
    }

    let force_index = command
        .parts
        .iter()
        .position(|part| part == "--force" || part == "-f")?;
    let mut corrected = command.parts.clone();
    corrected[force_index] = "--force-with-lease".to_string();

    Some(MatchResult {
        rule: "git_force_with_lease",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_FORCE,
    })
}

pub fn git_checkout_to_switch_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 3 || command.parts[0] != "git" || command.parts[1] != "checkout" {
        return None;
    }

    if command.parts[2] == "-b" {
        if command.parts.len() < 4 {
            return None;
        }

        let mut corrected = vec!["git".to_string(), "switch".to_string(), "-c".to_string()];
        corrected.extend(command.parts.iter().skip(3).cloned());
        return Some(MatchResult {
            rule: "git_checkout_to_switch",
            corrected_command: corrected.join(" "),
            similarity: util::SIMILARITY_BRANCH,
        });
    }

    if command.parts[2].starts_with('-') {
        return None;
    }

    let mut corrected = vec!["git".to_string(), "switch".to_string()];
    corrected.extend(command.parts.iter().skip(2).cloned());

    Some(MatchResult {
        rule: "git_checkout_to_switch",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_BRANCH,
    })
}
