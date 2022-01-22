use anyhow::Result;
use clap::Parser;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Parser)]
pub struct RenameCommand {
    id: ID,
    name: String,
}

#[async_trait::async_trait]
impl JCommand for RenameCommand {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let item = jd.rename(self.id.clone(), &self.name).await?;
        println!("{}", item);
        Ok(())
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rename(self.id.clone(), &self.name).await?;
        Ok(())
    }
}
