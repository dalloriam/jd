use anyhow::{anyhow, Result};
use clap::Parser;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Parser)]
pub struct MkCatCommand {
    category: usize,
    name: String,
}

#[async_trait::async_trait]
impl JCommand for MkCatCommand {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let area = jd
            .index
            .get_area_from_category_mut(self.category)?
            .ok_or_else(|| anyhow!("area does not exist"))?;

        area.create_category(self.category, self.name.clone())?;

        jd.save()
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let area = jd
            .index
            .get_area_from_category_mut(self.category)?
            .ok_or_else(|| anyhow!("area does not exist"))?;

        area.create_category(self.category, self.name.clone())?;

        jd.save()
    }
}
