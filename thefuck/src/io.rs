use crate::types::MatchResult;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self, Write};

pub fn display_corrections(corrections: &[MatchResult]) {
    println!("Did you mean:");
    println!();

    for (i, correction) in corrections.iter().enumerate() {
        println!("  {}: {}", i + 1, correction.corrected_command);
        println!(
            "    ({}, {:.0}% match)",
            correction.rule,
            correction.similarity * 100.0
        );
        println!();
    }
}

pub fn wait_for_choice(corrections: &[MatchResult]) -> Result<Option<usize>> {
    if corrections.is_empty() {
        return Ok(None);
    }

    if corrections.len() == 1 {
        return Ok(Some(0));
    }

    println!("Select a correction:");
    io::stdout().flush()?;

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .items(
            &corrections
                .iter()
                .map(|c| c.corrected_command.as_str())
                .collect::<Vec<_>>(),
        )
        .default(0)
        .interact_opt()?;

    Ok(selection)
}

pub fn should_skip_confirmation() -> bool {
    std::env::var("THEFUCK_REQUIRE_CONFIRMATION")
        .map(|value| {
            let normalized = value.trim().to_ascii_lowercase();
            matches!(normalized.as_str(), "0" | "false" | "no" | "off")
        })
        .unwrap_or(false)
}
