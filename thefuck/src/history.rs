use crate::types::Error;
use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub async fn add_command(command: String) -> Result<()> {
    let path = history_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(file, "{}", command)?;

    Ok(())
}

pub fn get_history(limit: Option<usize>) -> Result<Vec<String>> {
    let path = history_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path)?;
    let mut history = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

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
