use std::path::PathBuf;
use std::{fs, path::Path};

use anyhow::{anyhow, bail, ensure, Result};

use fs_extra::dir::CopyOptions;
use lazy_static::lazy_static;

use regex::Regex;

use super::{Location, LocationResolver};

use crate::{
    index::{Area, Category},
    Index, Item, ID,
};

pub struct DiskResolver {
    root_path: PathBuf,
}

impl DiskResolver {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    fn collect_area(&self, path: &Path, area: &mut Area) -> Result<()> {
        lazy_static! {
            static ref CATEGORY_RE: Regex = Regex::new(r"(\d\d) (.*)").unwrap();
        }

        for entry in fs::read_dir(path)?.filter_map(|f| f.ok()) {
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            if !entry.path().is_dir() {
                bail!("file in area root: {:?}", entry.path());
            }

            let fname_str = entry.file_name().to_string_lossy().to_string();
            if !CATEGORY_RE.is_match(&fname_str) {
                bail!("invalid dir in area root: {:?}", entry.path());
            }

            if let Some(cap) = CATEGORY_RE.captures(&fname_str) {
                ensure!(cap.len() == 3, "invalid directory: {:?}", &fname_str,);
                let id = cap.get(1).unwrap().as_str().parse::<usize>()?;
                let name = cap.get(2).unwrap().as_str();

                let category = area.create_category_mut(id, String::from(name))?;
                self.collect_category(&entry.path(), category)?;
            }
        }

        Ok(())
    }

    fn collect_category(&self, path: &Path, category: &mut Category) -> Result<()> {
        lazy_static! {
            static ref ITEM_RE: Regex = Regex::new(r"(\d\d)\.(\d\d\d) (.*)").unwrap();
        }

        for entry in fs::read_dir(&path)?.filter_map(|f| f.ok()) {
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            if !entry.path().is_dir() {
                bail!("file in category root: {:?}", entry.path());
            }

            let fname_str = entry.file_name().to_string_lossy().to_string();
            if !ITEM_RE.is_match(&fname_str) {
                bail!("invalid dir in category root: {:?}", entry.path());
            }

            if let Some(cap) = ITEM_RE.captures(&fname_str) {
                ensure!(cap.len() == 4, "invalid directory: {:?}", &fname_str,);
                let cat = cap.get(1).unwrap().as_str().parse::<usize>()?;
                let id = cap.get(2).unwrap().as_str().parse::<usize>()?;
                let name = cap.get(3).unwrap().as_str();

                let item = Item {
                    id: ID { category: cat, id },
                    name: String::from(name),
                };
                category.import_item(item)?;
            }
        }
        Ok(())
    }

    fn get_category_path(&self, category: usize, index: &Index) -> Result<PathBuf> {
        let area = index
            .get_area_from_category(category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category(category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        let category_path = self
            .root_path
            .join(PathBuf::from(format!("{}", area)))
            .join(PathBuf::from(format!("{}", category)));

        Ok(category_path)
    }
}

impl LocationResolver for DiskResolver {
    fn get(&self, item: &Item, index: &Index) -> Result<Option<Location>> {
        let category_path = self.get_category_path(item.id.category, index)?;
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
                self.collect_area(&entry.path(), area)?;
            } else {
                bail!("invalid directory");
            }
        }

        Ok(())
    }

    fn set(&self, item: &Item, src_location: Location, index: &Index) -> Result<()> {
        let category_path = self.get_category_path(item.id.category, index)?;
        if !category_path.exists() {
            fs::create_dir_all(&category_path)?;
        }

        let dst = category_path.join(PathBuf::from(format!("{}", item)));

        match src_location {
            Location::Path(p) => {
                let options = CopyOptions {
                    copy_inside: true,
                    ..Default::default()
                };
                fs_extra::dir::move_dir(p, dst, &options)?;
            }
            Location::URL(_u) => {
                unimplemented!("TODO: Download & store");
            }
        }

        Ok(())
    }

    fn remove(&self, id: &Item, index: &Index) -> Result<()> {
        if let Some(loc) = self.get(id, index)? {
            match loc {
                Location::Path(p) => {
                    if p.exists() {
                        fs::remove_dir_all(p)?;
                    }
                }
                Location::URL(u) => {
                    bail!("incoherent location: {}", u);
                }
            }
        }

        Ok(())
    }

    fn rename_category(&self, category: usize, new_name: &str, index: &Index) -> Result<()> {
        let old_path = self.get_category_path(category, index)?;

        let area = index
            .get_area_from_category(category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let new_category = Category::new(category, String::from(new_name));

        let new_path = self
            .root_path
            .join(PathBuf::from(format!("{}", area)))
            .join(PathBuf::from(format!("{}", new_category)));

        let options = CopyOptions {
            copy_inside: true,
            ..Default::default()
        };
        fs_extra::dir::move_dir(old_path, new_path, &options)?;

        Ok(())
    }
}
