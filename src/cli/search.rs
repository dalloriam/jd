use anyhow::{anyhow, Result};
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

        let mut last_area_name = String::default();
        let mut last_category_name = String::default();

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

            let area = index.areas[result.category / 10]
                .as_ref()
                .ok_or_else(|| anyhow!("missing area"))?;

            let category = area.categories[result.category % 10]
                .as_ref()
                .ok_or_else(|| anyhow!("missing category"))?;

            if !self.locate && area.name != last_area_name {
                println!("- {:02}-{:02} {}", area.bounds.0, area.bounds.1, area.name);
                last_area_name = area.name.clone();
            }

            if !self.locate && category.name != last_category_name {
                println!("  - {} {}", category.id, category.name);
                last_category_name = category.name.clone();
            }

            if self.locate {
                if let Some(Destination::Path(p)) = index.locate(
                    &format!("{:02}.{:03}", result.category, result.id),
                    &mapping,
                ) {
                    println!("{}", p.to_string_lossy())
                }
            } else {
                println!("    - {}", result);
            }
        }
        Ok(())
    }
}
