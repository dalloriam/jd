use anyhow::{anyhow, Result};
use clap::Clap;

use johnny::JohnnyDecimal;

#[derive(Clap)]
pub struct LsCommand {
    category: usize,
}

impl LsCommand {
    pub fn run(self, jd: JohnnyDecimal) -> Result<()> {
        let area = jd
            .index
            .get_area_from_category(self.category)?
            .ok_or_else(|| anyhow!("area does not exist"))?;

        let category = area
            .get_category(self.category)?
            .ok_or_else(|| anyhow!("category does not exist"))?;

        for val in category.list_items() {
            println!("{}", val);
        }

        Ok(())
    }
}
