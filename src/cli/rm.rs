use std::fs;

use anyhow::{anyhow, Result};
use clap::Clap;

use johnny::{Destination, Index, Mapping};

use super::Config;

#[derive(Clap)]
pub struct RmCommand {
    id: String,
}

impl RmCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let mut index = Index::load(&config.index_path)?;
        let mapping = Mapping::load(&config.mapping_path)?;

        let Destination::Path(path) = index
            .locate(&self.id, &mapping)
            .ok_or_else(|| anyhow!("missing destination"))?;

        index.rm(&self.id)?;

        fs::remove_dir_all(path)?;

        Ok(())
    }
}
