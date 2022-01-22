use anyhow::Result;
use clap::Parser;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Parser)]
pub struct RelocateCommand {
    item: String,
    category: usize,
}

#[async_trait::async_trait]
impl JCommand for RelocateCommand {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let id = self.item.parse::<ID>()?;
        let item = jd.relocate(&id, self.category).await?;
        println!("{}", item);
        Ok(())
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let id = self.item.parse::<ID>()?;
        jd.relocate(&id, self.category).await?;
        Ok(())
    }
}
