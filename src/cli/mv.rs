use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use johnny::JohnnyDecimal;

use super::JCommand;

#[derive(Clap)]
pub struct MoveCommand {
    #[clap(long = "category", short = 'c')]
    category: usize,

    files: Vec<PathBuf>,
}

impl JCommand for MoveCommand {
    fn run(&self, mut jd: JohnnyDecimal) -> Result<()> {
        for f in self.files.iter() {
            let item = jd.mv(self.category, &f)?;
            println!("{}", item);
        }
        Ok(())
    }

    fn run_json(&self, jd: JohnnyDecimal) -> Result<()> {
        unimplemented!()
    }
}
