use anyhow::Result;
use clap::Clap;

use johnny::JohnnyDecimal;

#[derive(Clap)]
pub struct CategoriesCommand {}

impl CategoriesCommand {
    pub fn run(self, jd: JohnnyDecimal) -> Result<()> {
        for area in jd.index.list_areas() {
            println!("{:02}-{:02} {}", area.bounds.0, area.bounds.1, &area.name);
            for category in area.list_categories() {
                println!("  {:02} {}", category.id, category.name);
            }
        }

        Ok(())
    }
}
