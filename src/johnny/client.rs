use std::collections::HashMap;
use std::path::{PathBuf, Path};

use anyhow::{anyhow, Result};

use crate::{Config, Index, Location, LocationResolver, ID, Item};

pub struct JohnnyDecimal {
    config: Config,
    pub index: Box<Index>,
    resolvers: HashMap<usize, Box<dyn LocationResolver>>,
}

impl JohnnyDecimal {
    pub fn new(config: Config) -> Result<Self> {
        let index = Box::new(Index::load(&config.index_path)?);

        let mut resolvers = HashMap::new();
        for (id, resolver) in config.resolvers.iter() {
            // TODO: Initialize resolvers.
        }

        Ok(Self {
            config,
            index,
            resolvers,
        })
    }

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
        self.save()?;

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

        if let Some(item) = category.get_item(id)? {
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

        if let Some(item) = category.get_item(id)? {
            let resolver = self
                .resolvers
                .get(&item.id.category)
                .ok_or_else(|| anyhow!("missing resolver"))?;

            resolver.remove(&item)?;
            category.remove_item(&item.id)?;
        }

        self.save()?;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        self.index.save(&self.config.index_path).into()
    }
}
