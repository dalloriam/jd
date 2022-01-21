use anyhow::Result;
use clap::Parser;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Parser)]
pub struct CatRename {
    category: usize,
    name: String,
}

#[async_trait::async_trait]
impl JCommand for CatRename {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rename_category(self.category, &self.name).await
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rename_category(self.category, &self.name).await
    }
}
