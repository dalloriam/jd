use anyhow::Result;
use clap::Parser;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Parser)]
pub struct RmCommand {
    id: ID,
}

#[async_trait::async_trait]
impl JCommand for RmCommand {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rm(&self.id).await?;
        Ok(())
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rm(&self.id).await?;
        Ok(())
    }
}
