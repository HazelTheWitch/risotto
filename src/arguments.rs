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
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum RisottoCommand {
    /// Initialize an empty risotto.toml.
    Init {
        /// The path of the risotto.toml file.
        #[arg(short, long, default_value = "./risotto.toml")]
        path: PathBuf,
    },
    /// Apply the risotto.toml file at the current working directory.
    Apply,
    /// Add a config file to the current risotto.toml file.
    Add {
        /// The "remote" config file to add.
        #[arg(short, long)]
        target: PathBuf,

        /// The local version of the config file to create.
        #[arg(short, long)]
        local: PathBuf,
    },
}
