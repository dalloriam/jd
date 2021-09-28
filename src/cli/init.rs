use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Clap)]
pub struct InitCommand {
    /// The root to use to build the initial index.
    #[clap(long = "root", short = 'r')]
    root: Option<PathBuf>,
}

impl JCommand for InitCommand {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rebuild()?;
        Ok(())
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        unimplemented!()
    }
}
