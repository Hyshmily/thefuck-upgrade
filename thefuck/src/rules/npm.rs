use crate::types::{Command, MatchResult};
use crate::util;

const NPM_SUBCOMMANDS: &[&str] = &[
    "install",
    "uninstall",
    "update",
    "run",
    "test",
    "build",
    "start",
    "dev",
    "init",
    "publish",
    "add",
    "remove",
    "list",
    "link",
    "unlink",
    "audit",
    "fund",
    "exec",
    "config",
    "cache",
    "doctor",
    "help",
    "login",
    "logout",
    "outdated",
    "pack",
    "prefix",
    "prune",
    "rebuild",
    "restart",
    "search",
    "set",
    "shrinkwrap",
    "star",
    "stars",
    "stop",
    "team",
    "token",
    "version",
    "view",
    "whoami",
    "explore",
    "diff",
    "dist-tag",
    "docs",
    "edit",
    "find",
    "repo",
    "access",
    "dedupe",
    "owner",
    "ping",
    "pkg",
    "profile",
    "query",
];

const NPM_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("install", &["isntall", "instal", "intsall", "insatll"]),
    ("uninstall", &["unistall", "uninstal", "unintsall"]),
    ("update", &["udpate", "updtae", "updte"]),
    ("init", &["iniit", "int", "inti"]),
    ("start", &["satrt", "strt", "star"]),
    ("test", &["tets", "tst", "tes"]),
    ("run", &["rnu", "rn", "ru"]),
    ("build", &["bulid", "buid", "bluid"]),
    ("publish", &["publis", "publsh", "pubish"]),
];

const THRESHOLD: f64 = 0.75;

fn is_npm_like(bin: &str) -> bool {
    matches!(bin, "npm" | "yarn" | "pnpm" | "npx")
}

fn npm_like_typo(bin: &str) -> Option<&str> {
    match bin {
        "npn" | "pnm" => Some("npm"),
        "yrn" | "yar" => Some("yarn"),
        "ppnm" | "pnmp" => Some("pnpm"),
        _ => None,
    }
}

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, NPM_SUBCOMMANDS, NPM_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn npm_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = npm_like_typo(&command.parts[0])?;

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "npm_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn npm_subcommand_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || !is_npm_like(&command.parts[0]) {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || NPM_SUBCOMMANDS.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "npm_subcommand_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
