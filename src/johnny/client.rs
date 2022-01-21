use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::config::ResolverConfig;
use crate::resolver::{DiskResolver, GithubResolver};
use crate::{Config, Index, Item, Location, LocationResolver, ResolverConstraint, ID};

pub struct JohnnyDecimal {
    config: Config,
    pub index: Box<Index>,
    resolvers: Vec<(ResolverConstraint, Arc<dyn LocationResolver>)>,
}

impl JohnnyDecimal {
    pub fn new(config: Config) -> Result<Self> {
        let index = Box::new(Index::load(&config.index_path).unwrap_or_default());

        let mut resolvers = Vec::new();
        for resolver in config.resolvers.iter() {
            // TODO: Detect resolver conflict
            let r: Arc<dyn LocationResolver> = match &resolver.config {
                ResolverConfig::DiskResolver { root } => Arc::new(DiskResolver::new(root.clone())),
                &ResolverConfig::GithubResolver { github_area } => {
                    Arc::new(GithubResolver::new(github_area))
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

    fn find_resolver(&self, category: usize) -> Option<Arc<dyn LocationResolver>> {
        self.resolvers
            .iter()
            .find(|(c, _r)| c.matches(category))
            .map(|(_, r)| r)
            .cloned()
    }

    pub fn mv(&mut self, category: usize, source_path: &Path, id: Option<&ID>) -> Result<Item> {
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
        let item = category.add_item(&name, id)?;
        self.save()?;

        let src_location = Location::Path(PathBuf::from(source_path));

        resolver.set(&item, src_location, &self.index)?;

        Ok(item)
    }

    pub fn alloc_url(&mut self, category: usize, name: &str, url: &str) -> Result<Item> {
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

        let item = category.add_item(name, None)?;

        resolver.set(&item, Location::URL(String::from(url)), &self.index)?;

        self.save()?;

        Ok(item)
    }

    pub fn relocate(&mut self, id: &ID, category: usize) -> Result<Item> {
        let item = {
            let current_area = self
                .index
                .get_area_from_category_mut(id.category)?
                .ok_or_else(|| anyhow!("missing area"))?;

            let current_category = current_area
                .get_category_mut(id.category)?
                .ok_or_else(|| anyhow!("missing area"))?;

            let item = current_category
                .get_item(id)?
                .ok_or_else(|| anyhow!("missing item"))?;

            current_category.remove_item(id)?;
            item
        };

        let src_resolver = self
            .find_resolver(id.category)
            .ok_or_else(|| anyhow!("no resolver for category: {}", id.category))?;

        let src_path = src_resolver
            .get(&item, &self.index)?
            .ok_or_else(|| anyhow!("source file not found"))?;

        let tgt_area = self
            .index
            .get_area_from_category_mut(category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let tgt_category = tgt_area
            .get_category_mut(category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let item = tgt_category.add_item(&item.name, None)?;

        // Now that the index is updated we need to move the files.
        let dst_resolver = self
            .find_resolver(category)
            .ok_or_else(|| anyhow!("no resolver for category: {}", category))?;
        dst_resolver.set(&item, src_path, &self.index)?;

        // Now we save.
        self.save()?;

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

    pub fn rename_category(&mut self, category: usize, new_name: &str) -> Result<()> {
        {
            let resolver = self
                .find_resolver(category)
                .ok_or_else(|| anyhow!("no resolver for category: {}", category))?;

            resolver.rename_category(category, new_name, &self.index)?;
        }

        let area = self
            .index
            .get_area_from_category_mut(category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let cat = area
            .get_category_mut(category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        cat.name = String::from(new_name);

        self.save()?;

        Ok(())
    }

    pub fn rename(&mut self, id: ID, new_name: &str) -> Result<Item> {
        let resolver = self
            .find_resolver(id.category)
            .ok_or_else(|| anyhow!("no resolver for category: {}", id.category))?;

        let area = self
            .index
            .get_area_from_category_mut(id.category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category_mut(id.category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        let old_item = category
            .get_item(&id)?
            .ok_or_else(|| anyhow!("id doesn't exist"))?;

        category.remove_item(&id)?;
        let new_item = category.add_item(new_name, Some(&id))?;

        resolver.rename_item(&old_item, &new_item, &self.index)?;

        self.save()?;

        Ok(new_item)
    }
}
