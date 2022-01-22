use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;

use johnny::{JohnnyDecimal, ID};

use super::JCommand;

#[derive(Parser)]
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

#[async_trait::async_trait]
impl JCommand for MoveCommand {
    async fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        self.validate_id()?;
        for f in self.files.iter() {
            let item = jd.mv(self.category, f, self.id.as_ref()).await?;
            println!("{}", item);
        }
        Ok(())
    }

    async fn run_json(&self, mut jd: JohnnyDecimal) -> Result<()> {
        self.validate_id()?;
        for f in self.files.iter() {
            jd.mv(self.category, f, self.id.as_ref()).await?;
        }
        Ok(())
    }
}
