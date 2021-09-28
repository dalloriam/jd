use anyhow::{anyhow, Result};
use clap::Clap;

use johnny::JohnnyDecimal;

use super::{json, JCommand};

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

impl JCommand for SearchCommand {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        let mut last_area_name = String::default();
        let mut last_category_name = String::default();

        for result in jd.index.search(&self.expr) {
            // TODO: This is super slow since we still search the full tree,
            // implement it intelligently in the future please.
            if let Some(a) = self.area.as_ref() {
                if *a / 10 != result.id.category / 10 {
                    continue;
                }
            }
            if let Some(c) = self.category.as_ref() {
                if *c != result.id.category {
                    continue;
                }
            }

            let area = jd
                .index
                .get_area_from_category(result.id.category)?
                .ok_or_else(|| anyhow!("missing area"))?;

            let category = area
                .get_category(result.id.category)?
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
                if let Some(loc) = jd.locate(&result.id)? {
                    println!("{}", loc);
                }
            } else {
                println!("    - {}", result);
            }
        }
        Ok(())
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        let hits = jd.index.search(&self.expr);
        let viewer = json::Viewer::new(&jd);

        let views = hits
            .iter()
            .map(|x| viewer.item(x))
            .collect::<Result<Vec<_>>>()?;

        println!("{}", serde_json::to_string(&views)?);

        Ok(())
    }
}
