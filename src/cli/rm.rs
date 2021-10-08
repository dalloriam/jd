use anyhow::Result;
use clap::Clap;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Clap)]
pub struct RmCommand {
    id: ID,
}

impl JCommand for RmCommand {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rm(&self.id)?;
        Ok(())
    }

    fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        jd.rm(&self.id)?;
        Ok(())
    }
}
