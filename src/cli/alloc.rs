use anyhow::Result;
use clap::Clap;

use johnny::Index;

use super::Config;

#[derive(Clap)]
pub struct AllocCommand {
    #[clap(long = "category", short = 'c')]
    category: usize,

    /// The name to allocate.
    #[clap(long = "name", short = 'n')]
    name: String,
}

impl AllocCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let mut index = Index::load(&config.index_path)?;
        let item = index.alloc_item(self.category, &self.name)?;
        println!("{}", item);
        index.save(config.index_path)?;
        Ok(())
    }
}
