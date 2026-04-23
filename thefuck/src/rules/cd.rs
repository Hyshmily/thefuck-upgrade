use crate::types::{Command, MatchResult};
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
            let name = entry.file_name().to_string_lossy().to_string();
            let distance = levenshtein(target, &name);
            (distance <= 2).then_some((name, distance))
        })
        .min_by_key(|(_, distance)| *distance)?;

    Some(MatchResult {
        rule: "cd_correction".to_string(),
        corrected_command: format!("cd {}", best.0),
        similarity: 1.0 - (best.1 as f64 / target.len().max(best.0.len()) as f64),
    })
}

fn levenshtein(left: &str, right: &str) -> usize {
    let left_chars = left.chars().collect::<Vec<_>>();
    let right_chars = right.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; right_chars.len() + 1]; left_chars.len() + 1];

    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }

    for (j, cell) in dp[0].iter_mut().enumerate() {
        *cell = j;
    }

    for i in 1..=left_chars.len() {
        for j in 1..=right_chars.len() {
            if left_chars[i - 1] == right_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + 1);
            }
        }
    }

    dp[left_chars.len()][right_chars.len()]
}
