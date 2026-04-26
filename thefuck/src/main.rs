use anyhow::Result;
use clap::Parser;
use thefuck::argument_parser::{Cli, SubCommand};
use thefuck::entrypoints;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        Some(SubCommand::Alias) => {
            entrypoints::print_alias();
        }
        Some(SubCommand::FirstUse) => {
            entrypoints::firstuse::main()?;
        }
        Some(SubCommand::Update) => {
            entrypoints::update::main()?;
        }
        Some(SubCommand::Delete) => {
            entrypoints::delete::main()?;
        }
        None => {
            entrypoints::fix_command::run(cli.fix).await?;
        }
    }

    Ok(())
}
