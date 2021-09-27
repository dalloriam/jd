use std::collections::HashMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

pub enum Location {
    Path(PathBuf),
    URL(String),
}

pub struct ID {
    pub category: usize,
    pub id: usize,
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:03}", self.category, self.id)
    }
}

pub trait LocationResolver {
    fn get(&self, item: &Item) -> Result<Option<Location>>;
    fn list(&self) -> Result<Vec<Item>>;
    fn set(&self, item: &Item, src_location: Location) -> Result<()>;
    fn remove(&self, id: &Item) -> Result<()>;
}

pub struct Item {
    id: ID,
    name: String,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.name)
    }
}

pub struct Category {
    pub id: u16,
    pub name: String,
    items: [Box<Option<Item>>; 1000],
}

impl Category {
    pub fn add_item(&mut self, name: &str) -> Result<Item> {
        unimplemented!()
    }

    pub fn get_item(&self, id: usize) -> Result<Option<Item>> {
        unimplemented!()
    }

    pub fn list_items(&self) -> &[Item] {
        unimplemented!()
    }

    pub fn remove_item(&self, id: &ID) -> Result<()> {
        unimplemented!()
    }
}

pub struct Area {
    pub bounds: (usize, usize),
    pub name: String,
    categories: [Box<Option<Category>>; 10],
}

impl Area {
    pub fn create_category(&self, category_id: usize, name: &str) -> Result<&Category> {
        unimplemented!()
    }

    pub fn get_category(&self, category_id: usize) -> Result<Option<&Category>> {
        unimplemented!()
    }

    pub fn get_category_mut(&mut self, category_id: usize) -> Result<Option<&mut Category>> {
        unimplemented!()
    }

    pub fn list_categories(&self) -> &[Category] {
        unimplemented!()
    }
}

pub struct Index {
    areas: [Box<Option<Area>>; 10],
}

impl Index {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        unimplemented!()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        unimplemented!()
    }

    pub fn create_area(&self, bounds: (usize, usize), name: &str) -> Result<&Area> {
        unimplemented!()
    }

    pub fn get_area(&self, bounds: (usize, usize)) -> Result<Option<&Area>> {
        unimplemented!()
    }

    pub fn get_area_mut(&mut self, bounds: (usize, usize)) -> Result<Option<&mut Area>> {
        unimplemented!()
    }

    pub fn get_area_from_category(&self, category: usize) -> Result<Option<&Area>> {
        unimplemented!()
    }

    pub fn get_area_from_category_mut(&mut self, category: usize) -> Result<Option<&mut Area>> {
        unimplemented!()
    }

    pub fn list_areas(&self) -> &[Area] {
        unimplemented!()
    }

    pub fn search(&self, query: &str) -> Vec<Item> {
        unimplemented!()
    }
}

pub enum ResolverConfig {
    DiskResolver { root: PathBuf },
}

pub struct JDConfig {
    index_path: PathBuf,
    resolvers: HashMap<usize, ResolverConfig>,
}

pub struct JD {
    config: JDConfig,
    pub index: Box<Index>,
    resolvers: HashMap<usize, Box<dyn LocationResolver>>,
}

impl JD {
    pub fn mv(&mut self, category: usize, source_path: &Path) -> Result<Item> {
        let resolver = self
            .resolvers
            .get(&category)
            .ok_or_else(|| anyhow!("no resolver for category: {}", category))?;

        let area = self
            .index
            .get_area_from_category_mut(category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category_mut(category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        let name = source_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let item = category.add_item(&name)?;

        let src_location = Location::Path(PathBuf::from(source_path));

        resolver.set(&item, src_location)?;

        Ok(item)
    }

    pub fn locate(&self, id: &ID) -> Result<Option<Location>> {
        let resolver = self
            .resolvers
            .get(&id.category)
            .ok_or_else(|| anyhow!("no resolver for category: {}", id.category))?;

        let area = self
            .index
            .get_area_from_category(id.category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category(id.category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        if let Some(item) = category.get_item(id.category)? {
            resolver.get(&item)
        } else {
            Ok(None)
        }
    }

    pub fn rm(&mut self, id: &ID) -> Result<()> {
        let area = self
            .index
            .get_area_from_category_mut(id.category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category_mut(id.category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        if let Some(item) = category.get_item(id.id)? {
            let resolver = self
                .resolvers
                .get(&item.id.category)
                .ok_or_else(|| anyhow!("missing resolver"))?;

            resolver.remove(&item)?;
            category.remove_item(&item.id)?;
        }

        Ok(())
    }
}
