use anyhow::{anyhow, Result};
use clap::Clap;

use johnny::JohnnyDecimal;

#[derive(Clap)]
pub struct MkCatCommand {
    category: usize,
    name: String,
}

impl MkCatCommand {
    pub fn run(self, mut jd: JohnnyDecimal) -> Result<()> {
        let area = jd
            .index
            .get_area_from_category_mut(self.category)?
            .ok_or_else(|| anyhow!("area does not exist"))?;

        area.create_category(self.category, &self.name)?;

        jd.save()?;

        Ok(())
    }
}
