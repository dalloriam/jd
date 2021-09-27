use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use johnny::JohnnyDecimal;

#[derive(Clap)]
pub struct MoveCommand {
    #[clap(long = "category", short = 'c')]
    category: usize,

    files: Vec<PathBuf>,
}

impl MoveCommand {
    pub fn run(self, mut jd: JohnnyDecimal) -> Result<()> {
        for f in self.files {
            let item = jd.mv(self.category, &f)?;
            println!("{}", item);
        }
        Ok(())
    }
}
