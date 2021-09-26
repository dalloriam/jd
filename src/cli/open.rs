use anyhow::{anyhow, Result};
use clap::Clap;

use johnny::{Destination, Index, Mapping};

use super::Config;

#[derive(Clap)]
pub struct OpenCommand {
    id: String,
}

impl OpenCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let index = Index::load(&config.index_path)?;
        let mapping = Mapping::load(&config.mapping_path)?;

        let Destination::Path(p) = index
            .locate(&self.id, &mapping)
            .ok_or_else(|| anyhow!("{} not found", &self.id))?;

        open::that(p)?;

        Ok(())
    }
}
