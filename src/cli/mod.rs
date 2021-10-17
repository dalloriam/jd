mod addurl;
mod cat_rename;
mod init;
mod json;
mod locate;
mod ls;
mod mkarea;
mod mkcat;
mod mv;
mod open;
mod relocate;
mod rm;
mod search;

use anyhow::Result;

use clap::Parser;

use johnny::{Config, JohnnyDecimal};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(version = VERSION, author = "William Dussault")]
pub struct Root {
    #[clap(long = "json")]
    json: bool,

    #[clap(subcommand)]
    command: Cmd,
}

pub trait JCommand {
    fn run(&self, jd: JohnnyDecimal) -> Result<()>;
    fn run_json(&self, jd: JohnnyDecimal) -> Result<()>;
}

impl Root {
    pub fn run(self) -> Result<()> {
        let cfg = Config::load()?;
        let client = JohnnyDecimal::new(cfg)?;

        if self.json {
            self.command.run_json(client)
        } else {
            self.command.run(client)
        }
    }
}

#[derive(Parser)]
pub enum CategoryCmd {
    #[clap(name = "new")]
    Create(mkcat::MkCatCommand),

    #[clap(name = "rename")]
    Rename(cat_rename::CatRename),
}

impl JCommand for CategoryCmd {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            CategoryCmd::Create(cmd) => cmd.run(jd),
            CategoryCmd::Rename(cmd) => cmd.run(jd),
        }
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            CategoryCmd::Create(cmd) => cmd.run_json(jd),
            CategoryCmd::Rename(cmd) => cmd.run_json(jd),
        }
    }
}

#[derive(Parser)]
pub enum ItemCmd {
    #[clap(name = "add_file")]
    AddFile(mv::MoveCommand),

    #[clap(name = "add_url")]
    AddURL(addurl::AddURLCommand),

    #[clap(name = "open")]
    Open(open::OpenCommand),

    #[clap(name = "mv")]
    Move(relocate::RelocateCommand),

    #[clap(name = "find")]
    Locate(locate::LocateCommand),

    #[clap(name = "rm")]
    Remove(rm::RmCommand),
}

impl JCommand for ItemCmd {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            ItemCmd::AddFile(cmd) => cmd.run(jd),
            ItemCmd::AddURL(cmd) => cmd.run(jd),
            ItemCmd::Open(cmd) => cmd.run(jd),
            ItemCmd::Move(cmd) => cmd.run(jd),
            ItemCmd::Locate(cmd) => cmd.run(jd),
            ItemCmd::Remove(cmd) => cmd.run(jd),
        }
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            ItemCmd::AddFile(cmd) => cmd.run_json(jd),
            ItemCmd::AddURL(cmd) => cmd.run_json(jd),
            ItemCmd::Open(cmd) => cmd.run_json(jd),
            ItemCmd::Move(cmd) => cmd.run_json(jd),
            ItemCmd::Locate(cmd) => cmd.run_json(jd),
            ItemCmd::Remove(cmd) => cmd.run_json(jd),
        }
    }
}

#[derive(Parser)]
enum AreaCmd {
    New(mkarea::MkAreaCommand),
}

impl JCommand for AreaCmd {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            AreaCmd::New(cmd) => cmd.run(jd),
        }
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            AreaCmd::New(cmd) => cmd.run_json(jd),
        }
    }
}

#[derive(Parser)]
enum Cmd {
    #[clap(name = "init")]
    Init(init::InitCommand),

    #[clap(name = "ls")]
    List(ls::LsCommand),

    #[clap(name = "search")]
    Search(search::SearchCommand),

    /// Open an ID.
    Open(open::OpenCommand),

    #[clap(subcommand)]
    #[clap(name = "area")]
    Areas(AreaCmd),

    #[clap(subcommand)]
    #[clap(name = "cat")]
    Categories(CategoryCmd),

    #[clap(subcommand)]
    #[clap(name = "item")]
    Item(ItemCmd),
}

impl JCommand for Cmd {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            Cmd::Init(cmd) => cmd.run(jd),
            Cmd::List(cmd) => cmd.run(jd),
            Cmd::Open(cmd) => cmd.run(jd),
            Cmd::Search(cmd) => cmd.run(jd),
            Cmd::Areas(cmd) => cmd.run(jd),
            Cmd::Categories(cmd) => cmd.run(jd),
            Cmd::Item(cmd) => cmd.run(jd),
        }
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            Cmd::Init(cmd) => cmd.run_json(jd),
            Cmd::List(cmd) => cmd.run_json(jd),
            Cmd::Open(cmd) => cmd.run_json(jd),
            Cmd::Search(cmd) => cmd.run_json(jd),
            Cmd::Areas(cmd) => cmd.run_json(jd),
            Cmd::Categories(cmd) => cmd.run_json(jd),
            Cmd::Item(cmd) => cmd.run_json(jd),
        }
    }
}
