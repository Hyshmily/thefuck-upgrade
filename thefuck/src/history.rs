use crate::types::Error;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

const MAX_HISTORY_LINES: usize = 1000;

pub async fn add_command(command: String) -> Result<()> {
    let path = history_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let existing = fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<&str> = existing.lines().collect();
    lines.push(&command);
    if lines.len() > MAX_HISTORY_LINES {
        lines = lines[lines.len() - MAX_HISTORY_LINES..].to_vec();
    }
    fs::write(&path, lines.join("\n") + "\n")?;

    Ok(())
}

pub fn get_history(limit: Option<usize>) -> Result<Vec<String>> {
    let path = history_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path)?;
    let mut history: Vec<String> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect();

    if let Some(limit) = limit {
        if history.len() > limit {
            let keep_from = history.len() - limit;
            history = history.split_off(keep_from);
        }
    }

    Ok(history)
}

fn history_path() -> Result<PathBuf> {
    let base = dirs::data_local_dir()
        .or_else(dirs::home_dir)
        .ok_or_else(|| Error::InvalidCommand("No history directory".to_string()))?;

    Ok(base.join("thefuck").join("history"))
}
