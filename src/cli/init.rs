use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use johnny::Index;

use super::Config;

#[derive(Clap)]
pub struct InitCommand {
    /// The root to use to build the initial index.
    #[clap(long = "root", short = 'r')]
    root: Option<PathBuf>,
}

impl InitCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let mut index = Index::default();

        if let Some(root) = self.root {
            index.build_from(root)?;
        }

        let f = fs::File::create(&config.index_path)?;

        serde_json::to_writer(f, &index)?;

        Ok(())
    }
}
