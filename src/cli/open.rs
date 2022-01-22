use anyhow::Result;
use clap::Parser;

use johnny::{JohnnyDecimal, Location, ID};

use super::JCommand;

#[derive(Parser)]
pub struct OpenCommand {
    id: String,
}

impl OpenCommand {
    pub async fn open(&self, jd: JohnnyDecimal) -> Result<()> {
        let id = self.id.parse::<ID>()?;
        if let Some(location) = jd.locate(&id).await? {
            match location {
                Location::Path(p) => open::that(p)?,
                Location::URL(url) => open::that(url)?,
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl JCommand for OpenCommand {
    async fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        self.open(jd).await
    }

    async fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        self.open(jd).await
    }
}
