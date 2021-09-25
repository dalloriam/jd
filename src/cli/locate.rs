use anyhow::Result;
use clap::Clap;

use johnny::{Destination, Index, Mapping};

use super::Config;

#[derive(Clap)]
pub struct LocateCommand {
    /// The AC.ID code to search for.
    id: String,
}

impl LocateCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let index = Index::load(config.index_path)?;
        let mapping = Mapping::load(config.mapping_path)?;
        if let Some(dest) = index.locate(&self.id, &mapping) {
            let Destination::Path(p) = dest;
            println!("{}", p.to_string_lossy())
        }
        Ok(())
    }
}
