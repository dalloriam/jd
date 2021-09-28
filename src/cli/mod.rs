mod categories;
mod init;
mod json;
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
    #[clap(long = "json")]
    json: bool,

    #[clap(subcommand)]
    command: Command,
}

pub trait JCommand {
    fn run(&self, jd: JohnnyDecimal) -> Result<()>;
    fn run_json(&self, jd: JohnnyDecimal) -> Result<()>;
}

impl Root {
    pub fn run(self) -> Result<()> {
        let cfg = Config::load()?;
        let client = JohnnyDecimal::new(cfg)?;

        let cmd: Box<dyn JCommand> = match self.command {
            Command::Categories(cmd) => Box::new(cmd),
            Command::Init(cmd) => Box::new(cmd),
            Command::Locate(cmd) => Box::new(cmd),
            Command::Ls(cmd) => Box::new(cmd),
            Command::MkCat(cmd) => Box::new(cmd),
            Command::Move(cmd) => Box::new(cmd),
            Command::Open(cmd) => Box::new(cmd),
            Command::Rm(cmd) => Box::new(cmd),
            Command::Search(cmd) => Box::new(cmd),
            //Command::Validate(cmd) => cmd.run(client),
        };

        if self.json {
            cmd.run_json(client)
        } else {
            cmd.run(client)
        }
    }
}

#[derive(Clap)]
pub enum Command {
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

    /// Open a directory using the system defaults.
    Open(open::OpenCommand),

    #[clap(name = "rm")]
    Rm(rm::RmCommand),

    /// Search in the index.
    Search(search::SearchCommand),
    //Validate(validate::ValidateCommand),
}
