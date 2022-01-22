use anyhow::Result;
use clap::Parser;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Parser)]
pub struct MkAreaCommand {
    area: usize,
    name: String,
}

#[async_trait::async_trait]
impl JCommand for MkAreaCommand {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let lower_bound = (self.area / 10) * 10;
        let upper_bound = lower_bound + 9;

        {
            let area = jd
                .index
                .create_area((lower_bound, upper_bound), &self.name)?;

            println!("{}", area);
        }

        jd.save()?;

        Ok(())
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let lower_bound = (self.area / 10) * 10;
        let upper_bound = lower_bound + 9;

        jd.index
            .create_area((lower_bound, upper_bound), &self.name)?;

        jd.save()
    }
}
