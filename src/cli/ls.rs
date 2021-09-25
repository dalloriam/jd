use anyhow::Result;
use clap::Clap;

use johnny::Index;

use super::Config;

#[derive(Clap)]
pub struct LsCommand {
    category: usize,
}

impl LsCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let index = Index::load(&config.index_path)?;
        for val in index.list_for_category(self.category)? {
            println!("{}", val);
        }

        Ok(())
    }
}
