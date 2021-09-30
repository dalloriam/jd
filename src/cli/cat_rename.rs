use anyhow::Result;
use clap::Clap;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Clap)]
pub struct CatRename {
    category: usize,
    name: String,
}

impl JCommand for CatRename {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rename_category(self.category, &self.name)
    }

    fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rename_category(self.category, &self.name)
    }
}
