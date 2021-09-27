use std::{fmt::Display, str::FromStr};

use anyhow::{ensure, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ID {
    pub category: usize,
    pub id: usize,
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:03}", self.category, self.id)
    }
}

impl FromStr for ID {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (category, item_id) = s.split_once('.').ok_or_else(|| anyhow!("invalid id"))?;
        let category = category.parse::<usize>()?;
        let id = item_id.parse::<usize>()?;

        ensure!(category < 10, "invalid category");
        ensure!(id < 1000, "invalid item ID");

        Ok(ID { category, id })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub id: ID,
    pub name: String,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.name)
    }
}
