use anyhow::Result;
use clap::Clap;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Clap)]
pub struct RmCommand {
    id: String,
}

impl JCommand for RmCommand {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        let id = self.id.parse::<ID>()?;
        jd.rm(&id)?;
        Ok(())
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        unimplemented!()
    }
}
