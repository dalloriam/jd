mod categories;
//mod init;
mod locate;
mod ls;
mod mkcat;
mod mv;
mod open;
mod rm;
mod search;
//mod validate;

use anyhow::Result;

use clap::Clap;

use johnny::{Config, JohnnyDecimal};

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
        let client = JohnnyDecimal::new(cfg)?;

        match self.command {
            Command::Categories(cmd) => cmd.run(client),
            //Command::Init(cmd) => cmd.run(client),
            Command::Locate(cmd) => cmd.run(client),
            Command::Ls(cmd) => cmd.run(client),
            Command::MkCat(cmd) => cmd.run(client),
            Command::Move(cmd) => cmd.run(client),
            Command::Open(cmd) => cmd.run(client),
            Command::Rm(cmd) => cmd.run(client),
            Command::Search(cmd) => cmd.run(client),
            //Command::Validate(cmd) => cmd.run(client),
        }
    }
}

#[derive(Clap)]
pub enum Command {
    /// List categories.
    Categories(categories::CategoriesCommand),

    /// Initialize an index.
    //Init(init::InitCommand),

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

    /// Open a directory using the system defaults.
    Open(open::OpenCommand),

    #[clap(name = "rm")]
    Rm(rm::RmCommand),

    /// Search in the index.
    Search(search::SearchCommand),
    //Validate(validate::ValidateCommand),
}
