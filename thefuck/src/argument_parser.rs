use crate::types::{Command, Error};
use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(about = "Magnificent app which corrects your previous console command")]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    #[command(flatten)]
    pub fix: FixOptions,
}

#[derive(Debug, Clone, Subcommand)]
pub enum SubCommand {
    #[command(about = "Print shell alias")]
    Alias,
    #[command(about = "Print first use guide")]
    FirstUse,
}

#[derive(Debug, Clone, Default, Args)]
pub struct FixOptions {
    #[arg(long, help = "Original command to fix")]
    pub command: Option<String>,

    #[arg(long = "num-matches", help = "Number of matches to show")]
    pub num_matches: Option<usize>,

    #[arg(long, help = "Skip confirmation")]
    pub yes: bool,

    #[arg(long = "only-command", help = "Show only corrected command")]
    pub only_command: bool,

    #[arg(long, help = "Don't alter history")]
    pub no_alter_history: bool,

    #[arg(long, help = "Debug mode")]
    pub debug: bool,
}

impl FixOptions {
    pub fn resolved_command(&self) -> Option<String> {
        self.command
            .clone()
            .or_else(|| std::env::var("TF_HISTORY").ok())
            .or_else(|| std::env::var("THEFUCK_COMMAND").ok())
    }

    pub fn to_command(&self) -> Result<Command> {
        self.resolved_command()
            .map(Command::new)
            .ok_or_else(|| Error::InvalidCommand("No command provided".to_string()).into())
    }
}
