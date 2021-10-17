use anyhow::Result;
use clap::Parser;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Parser)]
pub struct AddURLCommand {
    #[clap(long = "category", short = 'c')]
    category: usize,

    name: String,

    url: String,
}

impl JCommand for AddURLCommand {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let item = jd.alloc_url(self.category, &self.name, &self.url)?;
        println!("{}", item);
        Ok(())
    }

    fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.alloc_url(self.category, &self.name, &self.url)?;
        Ok(())
    }
}
