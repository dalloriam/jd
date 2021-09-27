use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ResolverConfig {
    DiskResolver { root: PathBuf },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub index_path: PathBuf,

    #[serde(default = "HashMap::new")]
    pub resolvers: HashMap<usize, ResolverConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let index_path = dirs::data_dir()
            .unwrap()
            .join("dalloriam")
            .join("jd")
            .join("index.json"); // yolo

        let resolvers = HashMap::new();

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
