use anyhow::Result;
use clap::Parser;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Parser)]
pub struct LocateCommand {
    /// The AC.ID code to search for.
    id: ID,
}

#[async_trait::async_trait]
impl JCommand for LocateCommand {
    async fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        if let Some(loc) = jd.locate(&self.id).await? {
            println!("{}", loc);
        }
        Ok(())
    }

    async fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        if let Some(loc) = jd.locate(&self.id).await? {
            println!("{}", serde_json::to_string(&loc)?);
        }
        Ok(())
    }
}
