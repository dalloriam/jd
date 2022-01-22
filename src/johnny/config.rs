use std::path::PathBuf;

use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResolverConfig {
    DiskResolver { root: PathBuf },
    GithubResolver { github_area: usize },
    MenmosResolver { profile: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResolverConstraint {
    ID(usize),
    Range((usize, usize)),
}

impl ResolverConstraint {
    pub fn matches(&self, id: usize) -> bool {
        match &self {
            ResolverConstraint::ID(i) => id == *i,
            ResolverConstraint::Range((min, max)) => id >= *min && id <= *max,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Resolver {
    pub constraint: ResolverConstraint,
    pub config: ResolverConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub index_path: PathBuf,

    #[serde(default = "Vec::new")]
    pub resolvers: Vec<Resolver>,
}

impl Default for Config {
    fn default() -> Self {
        let index_path = dirs::data_dir()
            .unwrap()
            .join("dalloriam")
            .join("jd")
            .join("index.json"); // yolo

        let resolvers = Vec::new();

        Self {
            index_path,
            resolvers,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(cfgloader::load_or_default(
            "dalloriam/jd",
            "config",
            Self::default(),
        )?)
    }
}
