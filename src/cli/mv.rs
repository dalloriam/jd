use std::path::PathBuf;

use anyhow::{bail, ensure, Result};
use clap::Clap;

use fs_extra::dir::CopyOptions;
use johnny::{Destination, Index, Mapping};

use super::Config;

#[derive(Clap)]
pub struct MoveCommand {
    #[clap(long = "category", short = 'c')]
    category: usize,

    files: Vec<PathBuf>,
}

impl MoveCommand {
    pub fn run(self, config: Config) -> Result<()> {
        let mut index = Index::load(&config.index_path)?;
        let mapping = Mapping::load(&config.mapping_path)?;

        for f in self.files {
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            let item = index.alloc_item(self.category, &name)?;
            println!("{}", item);

            // We need to move our item in the space we allocated for it.
            if let Some(Destination::Path(p)) =
                index.locate(&format!("{:02}.{:03}", item.category, item.id), &mapping)
            {
                ensure!(f.exists(), "source path {:?} does not exist", f);
                ensure!(f.is_dir(), "source path {:?} is not a directory", f);
                ensure!(!p.exists(), "destination path {:?} already exists", p);

                let options = CopyOptions {
                    copy_inside: true,
                    ..Default::default()
                };
                fs_extra::dir::move_dir(f, p, &options)?;
            } else {
                bail!("incoherent allocation");
            }

            index.save(&config.index_path)?;
        }

        Ok(())
    }
}
