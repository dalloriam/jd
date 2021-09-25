mod config;
mod init;
mod locate;
mod search;
mod validate;

use config::Config;

use anyhow::Result;
use clap::Clap;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clap)]
#[clap(version = VERSION, author = "William Dussault")]
pub struct Root {
    #[clap(subcommand)]
    command: Command,
}

impl Root {
    pub fn run(self) -> Result<()> {
        let cfg = Config::load()?;

        match self.command {
            Command::Init(cmd) => cmd.run(cfg),
            Command::Locate(cmd) => cmd.run(cfg),
            Command::Search(cmd) => cmd.run(cfg),
            Command::Validate(cmd) => cmd.run(cfg),
        }
    }
}

#[derive(Clap)]
pub enum Command {
    /// Initialize an index.
    Init(init::InitCommand),

    /// Locate an ID.
    Locate(locate::LocateCommand),

    // Search in the index.
    Search(search::SearchCommand),

    /// Validate a root.
    Validate(validate::ValidateCommand),
}
