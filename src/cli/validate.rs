use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use johnny::Index;

use super::Config;

#[derive(Clap)]
pub struct ValidateCommand {
    /// The root to validate.
    root: PathBuf,
}

impl ValidateCommand {
    pub fn run(self, _config: Config) -> Result<()> {
        let mut index = Index::default();
        index.build_from(self.root)?;
        println!("OK");
        Ok(())
    }
}
