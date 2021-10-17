use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Parser)]
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

    fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rebuild()?;
        Ok(())
    }
}
