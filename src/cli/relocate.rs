use anyhow::Result;
use clap::Clap;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Clap)]
pub struct RelocateCommand {
    item: String,
    category: usize,
}

impl JCommand for RelocateCommand {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let id = self.item.parse::<ID>()?;
        let item = jd.relocate(&id, self.category)?;
        println!("{}", item);
        Ok(())
    }

    fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let id = self.item.parse::<ID>()?;
        jd.relocate(&id, self.category)?;
        Ok(())
    }
}
