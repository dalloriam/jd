use std::path::PathBuf;
use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Destination {
    Path(PathBuf),
}

#[derive(Deserialize, Serialize)]
pub struct Override {
    pub category: usize,
    pub destination: Destination,
}

#[derive(Deserialize, Serialize)]
pub struct Mapping {
    pub default_root: PathBuf,

    #[serde(default = "HashMap::default")]
    pub overrides: HashMap<usize, Override>,
}

impl Mapping {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = fs::File::open(path.as_ref()).context(format!(
            "failed to find mapping file at {:?}",
            path.as_ref()
        ))?;
        Ok(serde_json::from_reader(f)?)
    }
}
