use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::config::ResolverConfig;
use crate::resolver::DiskResolver;
use crate::{Config, Index, Item, Location, LocationResolver, ResolverConstraint, ID};

pub struct JohnnyDecimal {
    config: Config,
    pub index: Box<Index>,
    resolvers: Vec<(ResolverConstraint, Arc<Box<dyn LocationResolver>>)>,
}

impl JohnnyDecimal {
    pub fn new(config: Config) -> Result<Self> {
        let index = Box::new(Index::load(&config.index_path).unwrap_or_default());

        let mut resolvers = Vec::new();
        for resolver in config.resolvers.iter() {
            // TODO: Detect resolver conflict
            let r: Arc<Box<dyn LocationResolver>> = match &resolver.config {
                ResolverConfig::DiskResolver { root } => {
                    Arc::new(Box::from(DiskResolver::new(root.clone())))
                }
            };
            resolvers.push((resolver.constraint.clone(), r));
        }

        Ok(Self {
            config,
            index,
            resolvers,
        })
    }

    fn find_resolver(&self, category: usize) -> Option<Arc<Box<dyn LocationResolver>>> {
        self.resolvers
            .iter()
            .find(|(c, _r)| c.matches(category))
            .map(|(_, r)| r)
            .cloned()
    }

    pub fn mv(&mut self, category: usize, source_path: &Path) -> Result<Item> {
        let resolver = self
            .find_resolver(category)
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

        resolver.set(&item, src_location, &self.index)?;

        Ok(item)
    }

    pub fn locate(&self, id: &ID) -> Result<Option<Location>> {
        let resolver = self
            .find_resolver(id.category)
            .ok_or_else(|| anyhow!("no resolver for category: {}", id.category))?;

        let area = self
            .index
            .get_area_from_category(id.category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category(id.category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        if let Some(item) = category.get_item(id)? {
            resolver.get(&item, &self.index)
        } else {
            Ok(None)
        }
    }

    pub fn rm(&mut self, id: &ID) -> Result<()> {
        let opt_item = {
            let area = self
                .index
                .get_area_from_category_mut(id.category)?
                .ok_or_else(|| anyhow!("missing area"))?;

            let category = area
                .get_category_mut(id.category)?
                .ok_or_else(|| anyhow!("missing category"))?;

            let opt = category.get_item(id)?;

            if let Some(i) = &opt {
                category.remove_item(&i.id)?;
            }

            opt
        };

        if let Some(item) = opt_item {
            let resolver = self
                .find_resolver(id.category)
                .ok_or_else(|| anyhow!("no resolver for category: {}", id.category))?;

            resolver.remove(&item, &self.index)?;
        }

        self.save()?;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        self.index.save(&self.config.index_path)
    }

    pub fn rebuild(&mut self) -> Result<()> {
        self.index = Box::new(Index::default());

        for (_, resolver) in self.resolvers.iter() {
            resolver.collect(&mut self.index)?;
        }

        self.save()?;

        Ok(())
    }
}
