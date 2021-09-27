use anyhow::Result;
use clap::Clap;

use johnny::{JohnnyDecimal, ID};

#[derive(Clap)]
pub struct RmCommand {
    id: String,
}

impl RmCommand {
    pub fn run(self, mut jd: JohnnyDecimal) -> Result<()> {
        let id = self.id.parse::<ID>()?;
        jd.rm(&id)?;
        Ok(())
    }
}
