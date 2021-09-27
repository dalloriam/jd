use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, bail, ensure, Result};

use regex::Regex;

use super::{Location, LocationResolver};

use crate::{index::Area, Index, Item};

pub struct DiskResolver {
    root_path: PathBuf,
}

impl LocationResolver for DiskResolver {
    fn get(&self, item: &Item, index: &Index) -> Result<Option<Location>> {
        let area = index
            .get_area_from_category(item.id.category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category(item.id.category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        let category_path = self
            .root_path
            .join(PathBuf::from(format!("{}", area)))
            .join(PathBuf::from(format!("{}", category)));

        if !category_path.exists() {
            return Ok(None);
        }

        let dst = category_path.join(PathBuf::from(format!("{}", item)));

        Ok(Some(Location::Path(dst)))
    }

    fn collect(&self, index: &mut Index) -> Result<()> {
        let area_re = Regex::new(r"(\d\d)-(\d\d) (.*)")?;

        for entry in fs::read_dir(&self.root_path)?.filter_map(|f| f.ok()) {
            if entry.file_name().to_string_lossy().starts_with('.') {
                // We skip hidden files
                continue;
            }

            ensure!(
                entry.path().is_dir(),
                "unexpected file in root: {:?}",
                entry.path()
            );

            let fname_str = entry.file_name().to_string_lossy().to_string();
            if !area_re.is_match(&fname_str) {
                bail!("invalid dir in Johnny Decimal Root: {:?}", entry.path());
            }

            if let Some(cap) = area_re.captures(&fname_str) {
                ensure!(cap.len() == 4, "invalid directory: {:?}", &fname_str,);
                let lower_bound = cap.get(1).unwrap().as_str().parse::<usize>()?;
                let upper_bound = cap.get(2).unwrap().as_str().parse::<usize>()?;
                let name = cap.get(3).unwrap().as_str();
                ensure!(
                    lower_bound + 9 == upper_bound,
                    "invalid directory bounds: {}-{}",
                    lower_bound,
                    upper_bound
                );

                let area = index.create_area_mut((lower_bound, upper_bound), name)?;
                // TODO: Load area contents here
                panic!("TODO: You were here")
            } else {
                bail!("invalid directory");
            }
        }

        Ok(Vec::default())
    }

    fn set(&self, item: &Item, src_location: Location, index: &Index) -> Result<()> {}

    fn remove(&self, id: &Item, index: &Index) -> Result<()> {}
}
