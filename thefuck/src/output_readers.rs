use crate::types::Command;
use anyhow::Result;

pub fn read_output(_command: &Command) -> Result<String> {
    Ok("".to_string())
}
