use std::convert::TryInto;
use std::fmt::Display;
use std::fs;
use std::path::Path;

use anyhow::{bail, ensure, Result};

use rayon::prelude::*;

use serde::{de::Error, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

use crate::{Item, ID};

fn from_vec<'de, D>(deserializer: D) -> std::result::Result<[Option<Box<Item>>; 1000], D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<Option<Box<Item>>> = Deserialize::deserialize(deserializer)?;
    v.try_into()
        .map_err(|_| D::Error::custom("expected a vec of length 1000"))
}

fn to_vec<S>(v: &[Option<Box<Item>>; 1000], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let v = Vec::from(v.as_ref());
    let mut seq = s.serialize_seq(Some(v.len()))?;
    for item in v {
        seq.serialize_element(&item)?;
    }

    seq.end()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: usize,
    pub name: String,

    #[serde(deserialize_with = "from_vec", serialize_with = "to_vec")]
    items: [Option<Box<Item>>; 1000],
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02} {}", self.id, self.name)
    }
}

impl Category {
    pub fn new(id: usize, name: String) -> Self {
        let mut v = Vec::new();
        v.resize(1000, None);
        let items: [Option<Box<Item>>; 1000] = v.try_into().unwrap(); // safe because we alloc the vector one line above

        Self { id, name, items }
    }

    pub fn add_item(&mut self, name: &str) -> Result<Item> {
        for id in 1..1000 {
            if self.items.get(id).is_some() {
                continue;
            }

            let id = ID {
                category: self.id,
                id,
            };

            let item = Item {
                id,
                name: String::from(name),
            };

            return self.import_item(item);
        }

        bail!("no more available IDs");
    }

    pub fn import_item(&mut self, item: Item) -> Result<Item> {
        ensure!(item.id.id < 1000, "invalid item id");
        ensure!(item.id.category == self.id, "invalid item category");
        ensure!(self.items[item.id.id].is_none(), "item already exists");

        self.items[item.id.id] = Some(Box::from(item.clone()));
        Ok(item)
    }

    pub fn get_item(&self, id: &ID) -> Result<Option<Item>> {
        ensure!(id.category == self.id, "invalid category");
        ensure!(id.id < 1000, "id out of range");
        Ok(self.items[id.id].clone().map(|i| *i))
    }

    pub fn list_items(&self) -> Vec<Item> {
        // TODO: Would be faster to return an iterator
        self.items
            .iter()
            .filter_map(|i| i.as_deref().cloned())
            .collect()
    }

    pub fn remove_item(&mut self, id: &ID) -> Result<()> {
        ensure!(id.category == self.id, "invalid category");
        ensure!(id.id < 1000, "id out of range");
        self.items[id.id] = None;
        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<Item> {
        self.items
            .par_iter()
            .filter_map(|f| f.as_ref())
            .filter(|item| item.name.to_lowercase().contains(query))
            .map(|i| *i.clone())
            .collect()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Area {
    pub bounds: (usize, usize),
    pub name: String,
    categories: [Option<Box<Category>>; 10],
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}-{:02} {}", self.bounds.0, self.bounds.1, self.name)
    }
}

impl Area {
    pub fn new(bounds: (usize, usize), name: String) -> Self {
        Self {
            bounds,
            name,
            categories: Default::default(),
        }
    }

    pub fn create_category(&mut self, category_id: usize, name: String) -> Result<&Category> {
        ensure!(
            category_id >= self.bounds.0 && category_id <= self.bounds.1,
            "invalid area {} for category {:02}",
            self,
            category_id
        );

        ensure!(
            self.categories[category_id % 10].is_none(),
            "category already exists"
        );

        self.categories[category_id % 10] = Some(Box::new(Category::new(category_id, name)));

        Ok(self.categories[category_id % 10].as_deref().unwrap())
    }

    pub fn create_category_mut(
        &mut self,
        category_id: usize,
        name: String,
    ) -> Result<&mut Category> {
        ensure!(
            category_id >= self.bounds.0 && category_id <= self.bounds.1,
            "invalid area {} for category {:02}",
            self,
            category_id
        );

        ensure!(
            self.categories[category_id % 10].is_none(),
            "category already exists"
        );

        self.categories[category_id % 10] = Some(Box::new(Category::new(category_id, name)));

        Ok(self.categories[category_id % 10].as_deref_mut().unwrap())
    }

    pub fn get_category(&self, category_id: usize) -> Result<Option<&Category>> {
        ensure!(
            category_id >= self.bounds.0 && category_id <= self.bounds.1,
            "invalid area {} for category {:02}",
            self,
            category_id
        );

        Ok(self.categories[category_id % 10].as_deref())
    }

    pub fn get_category_mut(&mut self, category_id: usize) -> Result<Option<&mut Category>> {
        ensure!(
            category_id >= self.bounds.0 && category_id <= self.bounds.1,
            "invalid area {} for category {:02}",
            self,
            category_id
        );

        Ok(self.categories[category_id % 10].as_deref_mut())
    }

    pub fn list_categories(&self) -> Vec<&Category> {
        // TODO: Would be faster to return an iterator
        self.categories
            .iter()
            .filter_map(|c| c.as_deref())
            .collect::<Vec<_>>()
    }

    pub fn search(&self, query: &str) -> Vec<Item> {
        self.categories
            .par_iter()
            .filter_map(|f| f.as_ref())
            .map(|cat| cat.search(query))
            .reduce(Vec::default, |mut a, mut b| {
                a.append(&mut b);
                a
            })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Index {
    areas: [Option<Box<Area>>; 10],
}

impl Index {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = fs::File::open(path.as_ref())?;
        Ok(serde_json::from_reader(f)?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = fs::File::create(path.as_ref())?;
        serde_json::to_writer(f, &self)?;
        Ok(())
    }

    pub fn create_area(&mut self, bounds: (usize, usize), name: &str) -> Result<&Area> {
        ensure!(
            bounds.0 % 10 == 0 && bounds.1 == bounds.0 + 9,
            "invalid bounds"
        );
        ensure!(self.areas[bounds.0 / 10].is_none(), "area already exists");

        let area = Area::new(bounds, String::from(name));
        self.areas[bounds.0 / 10] = Some(Box::new(area));
        Ok(self.areas[bounds.0 / 10].as_deref().unwrap())
    }

    pub fn create_area_mut(&mut self, bounds: (usize, usize), name: &str) -> Result<&mut Area> {
        ensure!(
            bounds.0 % 10 == 0 && bounds.1 == bounds.0 + 9,
            "invalid bounds"
        );
        ensure!(self.areas[bounds.0 / 10].is_none(), "area already exists");

        let area = Area::new(bounds, String::from(name));
        self.areas[bounds.0 / 10] = Some(Box::new(area));
        Ok(self.areas[bounds.0 / 10].as_deref_mut().unwrap())
    }

    pub fn get_area(&self, bounds: (usize, usize)) -> Result<Option<&Area>> {
        ensure!(
            bounds.0 % 10 == 0 && bounds.1 == bounds.0 + 9,
            "invalid bounds"
        );
        Ok(self.areas[bounds.0 / 10].as_deref())
    }

    pub fn get_area_mut(&mut self, bounds: (usize, usize)) -> Result<Option<&mut Area>> {
        ensure!(
            bounds.0 % 10 == 0 && bounds.1 == bounds.0 + 9,
            "invalid bounds"
        );
        Ok(self.areas[bounds.0 / 10].as_deref_mut())
    }

    pub fn get_area_from_category(&self, category: usize) -> Result<Option<&Area>> {
        ensure!(category < 100, "invalid category");
        Ok(self.areas[category / 10].as_deref())
    }

    pub fn get_area_from_category_mut(&mut self, category: usize) -> Result<Option<&mut Area>> {
        ensure!(category < 100, "invalid category");
        Ok(self.areas[category / 10].as_deref_mut())
    }

    pub fn list_areas(&self) -> Vec<&Area> {
        // TODO: Would be faster to return an iterator
        self.areas
            .iter()
            .filter_map(|c| c.as_deref())
            .collect::<Vec<_>>()
    }

    pub fn search(&self, query: &str) -> Vec<Item> {
        let q = query.to_lowercase();
        self.areas
            .par_iter()
            .filter_map(|f| f.as_ref())
            .map(|area| area.search(&q))
            .reduce(Vec::default, |mut a, mut b| {
                a.append(&mut b);
                a
            })
    }
}
