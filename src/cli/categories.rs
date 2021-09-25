use anyhow::Result;
use clap::Clap;

use johnny::Index;

use super::Config;

#[derive(Clap)]
pub struct CategoriesCommand {}

impl CategoriesCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let index = Index::load(config.index_path)?;
        for area in index.areas.iter().filter_map(|a| a.as_ref()) {
            println!("{:02}-{:02} {}", area.bounds.0, area.bounds.1, &area.name);
            for category in area.categories.iter().filter_map(|a| a.as_ref()) {
                println!("  {:02} {}", category.id, category.name);
            }
        }

        Ok(())
    }
}
