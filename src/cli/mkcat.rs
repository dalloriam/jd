use anyhow::{anyhow, Result};
use clap::Clap;

use johnny::Index;

use super::Config;

#[derive(Clap)]
pub struct MkCatCommand {
    category: usize,
    name: String,
}

impl MkCatCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let mut index = Index::load(&config.index_path)?;
        let mut area = index
            .get_area_from_category_mut(self.category)?
            .ok_or_else(|| anyhow!("area does not exist"))?;

        area.create_category(self.category, &self.name)?;
        index.save(&config.index_path)?;

        Ok(())
    }
}
