use anyhow::Result;
use clap::Clap;

use johnny::{Destination, Index, Mapping};

use super::Config;

#[derive(Clap)]
pub struct SearchCommand {
    /// The string to search for.
    expr: String,

    /// Whether to locate the search results.
    #[clap(long = "locate", short = 'l')]
    locate: bool,

    /// An optional area restriction.
    #[clap(long = "area", short = 'a')]
    area: Option<usize>,

    /// An optional category restriction.
    #[clap(long = "category", short = 'c')]
    category: Option<usize>,
}

impl SearchCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let index = Index::load(config.index_path)?;
        let mapping = Mapping::load(config.mapping_path)?;

        for result in index.search(&self.expr) {
            // TODO: This is super slow since we still search the full tree,
            // implement it intelligently in the future please.
            if let Some(a) = self.area.as_ref() {
                if *a / 10 != result.category / 10 {
                    continue;
                }
            }
            if let Some(c) = self.category.as_ref() {
                if *c != result.category {
                    continue;
                }
            }

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
