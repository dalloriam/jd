use anyhow::{anyhow, Result};
use clap::Parser;

use johnny::JohnnyDecimal;

use super::{json, JCommand};

#[derive(Parser)]
pub struct SearchCommand {
    /// The string to search for.
    expr: String,

    /// An optional area restriction.
    #[clap(long = "area", short = 'a')]
    area: Option<usize>,

    /// An optional category restriction.
    #[clap(long = "category", short = 'c')]
    category: Option<usize>,
}

#[async_trait::async_trait]
impl JCommand for SearchCommand {
    async fn run(&self, jd: JohnnyDecimal) -> Result<()> {
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

            if area.name != last_area_name {
                bunt::println!("[{[blue + bold]:}]", area);
                last_area_name = area.name.clone();
            }

            if category.name != last_category_name {
                bunt::println!("  {[green]:}", category);
                last_category_name = category.name.clone();
            }

            println!("    {}", result);
        }
        Ok(())
    }

    async fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
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
