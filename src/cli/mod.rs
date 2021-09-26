mod alloc;
mod categories;
mod config;
mod init;
mod locate;
mod ls;
mod mkcat;
mod mv;
mod rm;
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
            Command::Alloc(cmd) => cmd.run(cfg),
            Command::Categories(cmd) => cmd.run(cfg),
            Command::Init(cmd) => cmd.run(cfg),
            Command::Locate(cmd) => cmd.run(cfg),
            Command::Ls(cmd) => cmd.run(cfg),
            Command::MkCat(cmd) => cmd.run(cfg),
            Command::Move(cmd) => cmd.run(cfg),
            Command::Rm(cmd) => cmd.run(cfg),
            Command::Search(cmd) => cmd.run(cfg),
            Command::Validate(cmd) => cmd.run(cfg),
        }
    }
}

#[derive(Clap)]
pub enum Command {
    /// Allocate a name.
    Alloc(alloc::AllocCommand),

    /// List categories.
    Categories(categories::CategoriesCommand),

    /// Initialize an index.
    Init(init::InitCommand),

    /// Locate an ID.
    Locate(locate::LocateCommand),

    /// List entries in a category.
    Ls(ls::LsCommand),

    /// Make a category.
    #[clap(name = "mkcat")]
    MkCat(mkcat::MkCatCommand),

    /// Move a directory in the system.
    #[clap(name = "mv")]
    Move(mv::MoveCommand),

    #[clap(name = "rm")]
    Rm(rm::RmCommand),

    /// Search in the index.
    Search(search::SearchCommand),

    /// Validate a root.
    Validate(validate::ValidateCommand),
}
