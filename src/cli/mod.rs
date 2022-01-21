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
mod rename;
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

#[async_trait::async_trait]
pub trait JCommand {
    async fn run(&self, jd: JohnnyDecimal) -> Result<()>;
    async fn run_json(&self, jd: JohnnyDecimal) -> Result<()>;
}

impl Root {
    pub async fn run(self) -> Result<()> {
        let cfg = Config::load()?;
        let client = JohnnyDecimal::new(cfg)?;

        if self.json {
            self.command.run_json(client).await
        } else {
            self.command.run(client).await
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

#[async_trait::async_trait]
impl JCommand for CategoryCmd {
    async fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            CategoryCmd::Create(cmd) => cmd.run(jd).await,
            CategoryCmd::Rename(cmd) => cmd.run(jd).await,
        }
    }

    async fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            CategoryCmd::Create(cmd) => cmd.run_json(jd).await,
            CategoryCmd::Rename(cmd) => cmd.run_json(jd).await,
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

    #[clap(name = "rename")]
    Rename(rename::RenameCommand),
}

#[async_trait::async_trait]
impl JCommand for ItemCmd {
    async fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            ItemCmd::AddFile(cmd) => cmd.run(jd).await,
            ItemCmd::AddURL(cmd) => cmd.run(jd).await,
            ItemCmd::Open(cmd) => cmd.run(jd).await,
            ItemCmd::Move(cmd) => cmd.run(jd).await,
            ItemCmd::Locate(cmd) => cmd.run(jd).await,
            ItemCmd::Remove(cmd) => cmd.run(jd).await,
            ItemCmd::Rename(cmd) => cmd.run(jd).await,
        }
    }

    async fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            ItemCmd::AddFile(cmd) => cmd.run_json(jd).await,
            ItemCmd::AddURL(cmd) => cmd.run_json(jd).await,
            ItemCmd::Open(cmd) => cmd.run_json(jd).await,
            ItemCmd::Move(cmd) => cmd.run_json(jd).await,
            ItemCmd::Locate(cmd) => cmd.run_json(jd).await,
            ItemCmd::Remove(cmd) => cmd.run_json(jd).await,
            ItemCmd::Rename(cmd) => cmd.run_json(jd).await,
        }
    }
}

#[derive(Parser)]
enum AreaCmd {
    New(mkarea::MkAreaCommand),
}

#[async_trait::async_trait]
impl JCommand for AreaCmd {
    async fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            AreaCmd::New(cmd) => cmd.run(jd).await,
        }
    }

    async fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            AreaCmd::New(cmd) => cmd.run_json(jd).await,
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

#[async_trait::async_trait]
impl JCommand for Cmd {
    async fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            Cmd::Init(cmd) => cmd.run(jd).await,
            Cmd::List(cmd) => cmd.run(jd).await,
            Cmd::Open(cmd) => cmd.run(jd).await,
            Cmd::Search(cmd) => cmd.run(jd).await,
            Cmd::Areas(cmd) => cmd.run(jd).await,
            Cmd::Categories(cmd) => cmd.run(jd).await,
            Cmd::Item(cmd) => cmd.run(jd).await,
        }
    }

    async fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        match self {
            Cmd::Init(cmd) => cmd.run_json(jd).await,
            Cmd::List(cmd) => cmd.run_json(jd).await,
            Cmd::Open(cmd) => cmd.run_json(jd).await,
            Cmd::Search(cmd) => cmd.run_json(jd).await,
            Cmd::Areas(cmd) => cmd.run_json(jd).await,
            Cmd::Categories(cmd) => cmd.run_json(jd).await,
            Cmd::Item(cmd) => cmd.run_json(jd).await,
        }
    }
}
