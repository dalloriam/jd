use anyhow::Result;
use clap::Clap;

use johnny::{JohnnyDecimal, ID};

#[derive(Clap)]
pub struct LocateCommand {
    /// The AC.ID code to search for.
    id: String,
}

impl LocateCommand {
    pub fn run(self, jd: JohnnyDecimal) -> Result<()> {
        let id = self.id.parse::<ID>()?;
        if let Some(loc) = jd.locate(&id)? {
            println!("{}", loc);
        }
        Ok(())
    }
}
