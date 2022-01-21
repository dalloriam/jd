use anyhow::Result;
use clap::Parser;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Parser)]
pub struct AddURLCommand {
    #[clap(long = "category", short = 'c')]
    category: usize,

    name: String,

    url: String,
}

#[async_trait::async_trait]
impl JCommand for AddURLCommand {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let item = jd.alloc_url(self.category, &self.name, &self.url).await?;
        println!("{}", item);
        Ok(())
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.alloc_url(self.category, &self.name, &self.url).await?;
        Ok(())
    }
}
