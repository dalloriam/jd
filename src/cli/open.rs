use anyhow::Result;
use clap::Clap;

use johnny::{JohnnyDecimal, Location, ID};

#[derive(Clap)]
pub struct OpenCommand {
    id: String,
}

impl OpenCommand {
    pub fn run(self, jd: JohnnyDecimal) -> Result<()> {
        let id = self.id.parse::<ID>()?;
        if let Some(location) = jd.locate(&id)? {
            match location {
                Location::Path(p) => open::that(p)?,
                Location::URL(url) => open::that(url)?,
            }
        }

        Ok(())
    }
}
