use crate::types::{Command, MatchResult};
use crate::util;
use std::fs;

pub fn cd_correction_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() != 2 || command.parts[0] != "cd" {
        return None;
    }

    let target = &command.parts[1];
    if target == ".." || std::path::Path::new(target).exists() {
        return None;
    }

    let cwd = std::env::current_dir().ok()?;
    let best = fs::read_dir(cwd)
        .ok()?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            let distance = util::levenshtein(target, &name);
            (distance <= 2).then_some((name, distance))
        })
        .min_by_key(|(_, distance)| *distance)?;

    Some(MatchResult {
        rule: "cd_correction",
        corrected_command: format!("cd {}", best.0),
        similarity: 1.0 - (best.1 as f64 / target.len().max(best.0.len()) as f64),
    })
}
