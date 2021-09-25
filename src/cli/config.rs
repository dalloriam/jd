use std::path::PathBuf;

use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub index_path: PathBuf, // TODO: Support storing the index in menmos
    pub mapping_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let index_path = dirs::data_dir()
            .unwrap()
            .join("dalloriam")
            .join("johnny")
            .join("index.json"); // yolo

        let mapping_path = dirs::data_dir()
            .unwrap()
            .join("dalloriam")
            .join("johnny")
            .join("mapping.json"); // yolo

        Self {
            mapping_path,
            index_path,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(cfgloader::load_or_default(
            "dalloriam/johnny",
            "config",
            Self::default(),
        )?)
    }
}
