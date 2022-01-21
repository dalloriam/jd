mod disk;
mod github;

use std::fmt::Display;
use std::path::PathBuf;

use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::{Index, Item};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Location {
    Path(PathBuf),
    URL(String),
}
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Path(p) => write!(f, "{}", p.to_string_lossy()),
            Location::URL(u) => write!(f, "{}", u),
        }
    }
}

#[async_trait::async_trait]
pub trait LocationResolver {
    async fn get(&self, item: &Item, index: &Index) -> Result<Option<Location>>;
    async fn collect(&self, index: &mut Index) -> Result<()>;
    async fn set(&self, item: &Item, src_location: Location, index: &Index) -> Result<()>;
    async fn remove(&self, id: &Item, index: &Index) -> Result<()>;
    async fn rename_category(&self, category: usize, new_name: &str, index: &Index) -> Result<()>;
    async fn rename_item(&self, old_item: &Item, new_item: &Item, index: &Index) -> Result<()>;
}

pub use disk::DiskResolver;
pub use github::GithubResolver;
