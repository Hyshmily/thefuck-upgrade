use anyhow::Result;
use clap::Parser;
use thefuck::argument_parser::Cli;
use thefuck::entrypoints;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.alias {
        entrypoints::print_alias();
    } else if cli.first_use {
        entrypoints::firstuse::main()?;
    } else if cli.update {
        entrypoints::update::main()?;
    } else if cli.delete {
        entrypoints::delete::main()?;
    } else {
        entrypoints::fix_command::run(cli.fix).await?;
    }

    Ok(())
}
