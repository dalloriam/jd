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

#[async_trait::async_trait]
impl JCommand for InitCommand {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rebuild().await?;
        Ok(())
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rebuild().await?;
        Ok(())
    }
}
