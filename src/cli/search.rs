use anyhow::Result;
use clap::Clap;

use johnny::{Destination, Index, Mapping};

use super::Config;

#[derive(Clap)]
pub struct SearchCommand {
    /// The string to search for.
    expr: String,

    #[clap(long = "locate", short = 'l')]
    locate: bool,
}

impl SearchCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let index = Index::load(config.index_path)?;
        let mapping = Mapping::load(config.mapping_path)?;

        for result in index.search(&self.expr) {
            if self.locate {
                if let Some(Destination::Path(p)) = index.locate(
                    &format!("{:02}.{:03}", result.category, result.id),
                    &mapping,
                ) {
                    println!("{}", p.to_string_lossy())
                }
            } else {
                println!("{}", result);
            }
        }
        Ok(())
    }
}
