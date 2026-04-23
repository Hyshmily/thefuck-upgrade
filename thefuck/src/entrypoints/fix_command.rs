use crate::argument_parser::FixOptions;
use crate::conf::create_config_if_missing;
use crate::conf::load_settings;
use crate::corrector::Corrector;
use crate::history;
use crate::io;
use anyhow::Result;

pub async fn run(options: FixOptions) -> Result<()> {
    let _ = create_config_if_missing();

    if options.resolved_command().is_none() {
        println!("No command found. Run a command first, then execute 'fuck'.");
        return Ok(());
    }

    let mut settings = load_settings()?;
    if let Some(num_matches) = options.num_matches {
        settings.num_close_matches = num_matches.max(1);
    }
    if options.no_alter_history {
        settings.alter_history = false;
    }
    if options.debug {
        settings.debug = true;
    }
    if options.yes {
        settings.require_confirmation = false;
    }

    let cmd = options.to_command()?;
    let corrector = Corrector::new(cmd.clone(), settings.clone());
    let corrections = corrector.find_corrections();

    if corrections.is_empty() {
        println!("No corrections found");
        return Ok(());
    }

    if options.only_command {
        println!("{}", corrections[0].corrected_command);
        return Ok(());
    }

    io::display_corrections(&corrections).await?;

    let should_skip_confirmation =
        options.yes || !settings.require_confirmation || io::should_skip_confirmation();
    let choice = if should_skip_confirmation {
        Some(0)
    } else {
        io::wait_for_choice(&corrections).await?
    };

    if let Some(index) = choice {
        let selected = &corrections[index];
        println!("Executing: {}", selected.corrected_command);
        if settings.alter_history {
            history::add_command(cmd.raw.clone()).await?;
        }
        execute_command(&selected.corrected_command).await?;
    }

    Ok(())
}

async fn execute_command(command: &str) -> Result<()> {
    use tokio::process::Command;

    // Split command into parts
    let parts = shell_words::split(command)?;
    if parts.is_empty() {
        return Ok(());
    }

    // Execute the command
    let mut child = Command::new(&parts[0]).args(&parts[1..]).spawn()?;

    // Wait for completion
    child.wait().await?;

    Ok(())
}
