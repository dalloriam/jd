use anyhow::{anyhow, Result};
use clap::Clap;

use johnny::JohnnyDecimal;

use super::{json, JCommand};

#[derive(Clap)]
pub struct LsCommand {
    category: Option<usize>,

    #[clap(long = "names")]
    show_names: bool,
}

impl JCommand for LsCommand {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        for area in jd.index.list_areas() {
            if let Some(cat_filter) = self.category {
                if cat_filter / 10 != area.bounds.0 / 10 {
                    continue;
                }
            }

            bunt::println!("[{[bold + blue]:}]", area);

            for category in area.list_categories() {
                if let Some(cat_filter) = self.category {
                    if cat_filter != category.id {
                        continue;
                    }
                }

                println!("  {}", category);

                if self.show_names || self.category.is_some() {
                    for item in category.list_items() {
                        println!("    {}", item);
                    }
                }
            }
        }

        Ok(())
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        let viewer = json::Viewer::new(&jd);

        let mut views = Vec::new();

        // TODO: Respect show-names parameter

        for area in jd.index.list_areas() {
            if let Some(cat_filter) = self.category {
                if cat_filter / 10 != area.bounds.0 / 10 {
                    continue;
                }
            }

            for category in area.list_categories() {
                if let Some(cat_filter) = self.category {
                    if cat_filter != category.id {
                        continue;
                    }
                }

                for item in category.list_items() {
                    views.push(viewer.item(&item)?);
                }
            }
        }

        println!("{}", serde_json::to_string(&views)?);

        Ok(())
    }
}
