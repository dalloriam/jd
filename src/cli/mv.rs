use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Clap;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Clap)]
pub struct MoveCommand {
    #[clap(long = "category", short = 'c')]
    category: usize,

    #[clap(long = "id")]
    id: Option<ID>,

    files: Vec<PathBuf>,
}

impl MoveCommand {
    fn validate_id(&self) -> Result<()> {
        if self.files.len() > 1 && self.id.is_some() {
            bail!("cannot specify an ID when uploading more than one file")
        }
        Ok(())
    }
}

impl JCommand for MoveCommand {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        self.validate_id()?;
        for f in self.files.iter() {
            let item = jd.mv(self.category, f, self.id.as_ref())?;
            println!("{}", item);
        }
        Ok(())
    }

    fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        self.validate_id()?;
        for f in self.files.iter() {
            jd.mv(self.category, f, self.id.as_ref())?;
        }
        Ok(())
    }
}
