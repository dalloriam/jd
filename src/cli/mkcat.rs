use anyhow::Result;
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
        index.add_category(self.category, &self.name)?;
        index.save(&config.index_path)?;

        Ok(())
    }
}
