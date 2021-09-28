use anyhow::Result;
use clap::Clap;

use johnny::JohnnyDecimal;

use serde::{Deserialize, Serialize};

use super::{json, JCommand};

#[derive(Deserialize, Serialize)]
struct Category {
    id: usize,
    name: String,
}

#[derive(Clap)]
pub struct CategoriesCommand {}

impl JCommand for CategoriesCommand {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        for area in jd.index.list_areas() {
            println!("{:02}-{:02} {}", area.bounds.0, area.bounds.1, &area.name);
            for category in area.list_categories() {
                println!("  {:02} {}", category.id, category.name);
            }
        }

        Ok(())
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        let mut categories = Vec::new();
        let viewer = json::Viewer::new(&jd);
        for area in jd.index.list_areas() {
            for category in area.list_categories() {
                categories.push(viewer.category(category));
            }
        }
        println!("{}", serde_json::to_string(&categories)?);

        Ok(())
    }
}
