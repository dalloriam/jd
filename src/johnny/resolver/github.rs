use anyhow::{anyhow, bail, ensure, Result};

use super::LocationResolver;
use crate::{Index, Item, Location, ID};

struct Repo {
    org: String,
    name: String,
}

pub struct GithubResolver {
    project_area: usize,
}

impl GithubResolver {
    pub fn new(area: usize) -> Self {
        Self { project_area: area }
    }

    fn get_repo_url(&self, id: &ID, index: &Index) -> Result<Repo> {
        ensure!(
            id.category / 10 == self.project_area / 10,
            "unhandled category: {}",
            id.category
        );
        let area = index
            .get_area_from_category(id.category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category(id.category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        let item = category
            .get_item(id)?
            .ok_or_else(|| anyhow!("missing item"))?;

        Ok(Repo {
            org: category.name.clone(),
            name: item.name,
        })
    }
}

impl LocationResolver for GithubResolver {
    fn get(&self, item: &Item, index: &Index) -> Result<Option<Location>> {
        let repo = self.get_repo_url(&item.id, index)?;

        // TODO: Check if the repo is on disk also, and if so return the disk path instead.

        Ok(Some(Location::URL(format!(
            "https://github.com/{}/{}",
            repo.org, repo.name
        ))))
    }

    fn collect(&self, _index: &mut Index) -> Result<()> {
        // TODO: Implement
        Ok(())
    }

    fn set(&self, _item: &Item, _src_location: Location, _index: &Index) -> Result<()> {
        // Nothing to do here, content is in the cloud.
        Ok(())
    }

    fn remove(&self, _id: &Item, _index: &Index) -> Result<()> {
        // TODO: Remove repo on disk.
        // Obviously we don't want to delete it from github.
        unimplemented!();
    }

    fn rename_category(&self, _category: usize, _new_name: &str, _index: &Index) -> Result<()> {
        // This is unsupported because it would mean renaming the org.
        bail!("unsupported operation");
    }
}
