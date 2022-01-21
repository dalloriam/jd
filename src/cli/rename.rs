use anyhow::Result;
use clap::Parser;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Parser)]
pub struct RenameCommand {
    id: ID,
    name: String,
}

impl JCommand for RenameCommand {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let item = jd.rename(self.id.clone(), &self.name)?;
        println!("{}", item);
        Ok(())
    }

    fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rename(self.id.clone(), &self.name)?;
        Ok(())
    }
}
