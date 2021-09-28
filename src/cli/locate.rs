use anyhow::Result;
use clap::Clap;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Clap)]
pub struct LocateCommand {
    /// The AC.ID code to search for.
    id: String,
}

impl JCommand for LocateCommand {
    fn run(&self, jd: JohnnyDecimal) -> Result<()> {
        let id = self.id.parse::<ID>()?;
        if let Some(loc) = jd.locate(&id)? {
            println!("{}", loc);
        }
        Ok(())
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        unimplemented!()
    }
}
