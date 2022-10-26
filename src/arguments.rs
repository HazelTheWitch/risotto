use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Interact with the Risotto config system.
#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Arguments {
    #[clap(subcommand)]
    pub subcommand: RisottoCommand,
    
    /// Verbose output of operations.
    #[arg(short, long)]
    pub verbose: bool
}

#[derive(Subcommand, Debug)]
pub enum RisottoCommand {
    /// Initialize an empty risotto.toml.
    Init {
        /// The path of the risotto.toml file.
        #[arg(short, long, default_value="./risotto.toml")]
        path: PathBuf,
    },
    /// Apply the risotto.toml file at the current working directory.
    Apply {
        /// Do not back up previous config files before applying the new ones
        #[arg(long)]
        no_backup: bool,
    },
}
