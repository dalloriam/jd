use anyhow::Result;
use clap::Parser;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Parser)]
pub struct LocateCommand {
    /// The AC.ID code to search for.
    id: ID,
}

impl JCommand for LocateCommand {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        if let Some(loc) = jd.locate(&self.id)? {
            println!("{}", loc);
        }
        Ok(())
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        if let Some(loc) = jd.locate(&self.id)? {
            println!("{}", serde_json::to_string(&loc)?);
        }
        Ok(())
    }
}
