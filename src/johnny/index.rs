use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::{convert::TryInto, path::Path};

use anyhow::{anyhow, bail, ensure, Result};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use serde::{de::Error, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

use crate::mapping::{Destination, Mapping};

#[derive(Clone, Deserialize, Serialize)]
pub struct Item {
    pub category: usize,
    pub id: usize,
    pub name: String,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:03} {}", self.category, self.id, self.name)
    }
}

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

#[derive(Deserialize, Serialize)]
pub struct Category {
    pub id: usize,
    pub name: String,

    #[serde(deserialize_with = "from_vec", serialize_with = "to_vec")]
    pub items: [Option<Box<Item>>; 1000],
}

impl Category {
    fn new(name: &str, cat_id: usize) -> Result<Self> {
        let mut v = Vec::new();
        v.resize(1000, None);
        let items: [Option<Box<Item>>; 1000] = v
            .try_into()
            .map_err(|_| anyhow!("not supposed to happen"))?;

        Ok(Self {
            id: cat_id,
            name: String::from(name),
            items,
        })
    }

    fn build_from(name: &str, path: &Path, cat_id: usize) -> Result<Self> {
        let mut v = Vec::new();
        v.resize(1000, None);
        let mut items: [Option<Box<Item>>; 1000] = v
            .try_into()
            .map_err(|_| anyhow!("not supposed to happen"))?;

        lazy_static! {
            static ref ITEM_RE: Regex = Regex::new(r"(\d\d)\.(\d\d\d) (.*)").unwrap();
        }

        for entry in fs::read_dir(&path)?.filter_map(|f| f.ok()) {
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            if !entry.path().is_dir() {
                bail!("file in category root: {:?}", entry.path());
            }

            let fname_str = entry.file_name().to_string_lossy().to_string();
            if !ITEM_RE.is_match(&fname_str) {
                bail!("invalid dir in category root: {:?}", entry.path());
            }

            if let Some(cap) = ITEM_RE.captures(&fname_str) {
                ensure!(cap.len() == 4, "invalid directory: {:?}", &fname_str,);
                let category = cap.get(1).unwrap().as_str().parse::<usize>()?;
                let id = cap.get(2).unwrap().as_str().parse::<usize>()?;
                let name = cap.get(3).unwrap().as_str();

                ensure!(
                    category == cat_id,
                    "invalid ID in category {}: {}.{}",
                    cat_id,
                    category,
                    id
                );

                ensure!(items[id].is_none(), "duplicate id: {}.{}", cat_id, id);

                items[id] = Some(Box::new(Item {
                    category: cat_id,
                    id,
                    name: String::from(name),
                }));
            }
        }

        Ok(Self {
            id: cat_id,
            name: String::from(name),
            items,
        })
    }

    fn search(&self, query: &str) -> Vec<Item> {
        self.items
            .par_iter()
            .filter_map(|f| f.as_ref())
            .filter(|item| item.name.to_lowercase().contains(query))
            .map(|i| *i.clone())
            .collect()
    }

    fn list(&self) -> Vec<Item> {
        self.items
            .iter()
            .filter_map(|f| f.as_ref())
            .map(|i| *i.clone())
            .collect()
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02} {}", self.id, self.name)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Area {
    pub bounds: (usize, usize),
    pub name: String,
    pub categories: [Option<Category>; 10],
}

impl Area {
    fn build_from(name: &str, path: &Path, lower_bound: usize, upper_bound: usize) -> Result<Self> {
        lazy_static! {
            static ref CATEGORY_RE: Regex = Regex::new(r"(\d\d) (.*)").unwrap();
        }

        let mut categories: [Option<Category>; 10] = Default::default();

        for entry in fs::read_dir(&path)?.filter_map(|f| f.ok()) {
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            if !entry.path().is_dir() {
                bail!("file in area root: {:?}", entry.path());
            }

            let fname_str = entry.file_name().to_string_lossy().to_string();
            if !CATEGORY_RE.is_match(&fname_str) {
                bail!("invalid dir in area root: {:?}", entry.path());
            }

            if let Some(cap) = CATEGORY_RE.captures(&fname_str) {
                ensure!(cap.len() == 3, "invalid directory: {:?}", &fname_str,);
                let category = cap.get(1).unwrap().as_str().parse::<usize>()?;
                let name = cap.get(2).unwrap().as_str();
                ensure!(
                    category <= upper_bound && category >= lower_bound,
                    "category {} {} must be in range {}-{}",
                    category,
                    name,
                    lower_bound,
                    upper_bound
                );

                let cat = Category::build_from(name, &entry.path(), category)?;
                ensure!(
                    categories[category % 10].is_none(),
                    "duplicate category: {}",
                    category
                );
                categories[category % 10] = Some(cat);
            }
        }

        Ok(Self {
            bounds: (lower_bound, upper_bound),
            name: String::from(name),
            categories,
        })
    }

    fn search(&self, query: &str) -> Vec<Item> {
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

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}-{:02} {}", self.bounds.0, self.bounds.1, self.name)
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Index {
    pub areas: [Option<Box<Area>>; 10],
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

    pub fn build_from(&mut self, path: PathBuf) -> Result<()> {
        let area_re = Regex::new(r"(\d\d)-(\d\d) (.*)")?;
        for entry in fs::read_dir(&path)?.filter_map(|f| f.ok()) {
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            if !entry.path().is_dir() {
                bail!("file in Johnny Decimal Root: {:?}", entry.path());
            }

            let fname_str = entry.file_name().to_string_lossy().to_string();
            if !area_re.is_match(&fname_str) {
                bail!("invalid dir in Johnny Decimal Root: {:?}", entry.path());
            }

            if let Some(cap) = area_re.captures(&fname_str) {
                ensure!(cap.len() == 4, "invalid directory: {:?}", &fname_str,);
                let lower_bound = cap.get(1).unwrap().as_str().parse::<usize>()?;
                let upper_bound = cap.get(2).unwrap().as_str().parse::<usize>()?;
                let name = cap.get(3).unwrap().as_str();
                ensure!(
                    lower_bound + 9 == upper_bound,
                    "invalid directory bounds: {}-{}",
                    lower_bound,
                    upper_bound
                );

                let area = Area::build_from(name, &entry.path(), lower_bound, upper_bound)?;
                ensure!(
                    self.areas[lower_bound / 10].is_none(),
                    "area {}-{} already exists",
                    lower_bound,
                    upper_bound
                );
                self.areas[lower_bound / 10] = Some(Box::from(area));
            } else {
                bail!("invalid directory");
            }
        }

        Ok(())
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

    pub fn get(&self, id: &str) -> Option<Item> {
        // TODO: Validate ID.
        let (category, item_id) = id.split_once('.')?;

        let category = category.parse::<usize>().ok()?;
        let item_id = item_id.parse::<usize>().ok()?;
        self.areas.as_ref()[category / 10]
            .as_ref()?
            .categories
            .as_ref()[category % 10]
            .as_ref()?
            .items[item_id]
            .as_ref()
            .map(|i| *i.clone())
    }

    pub fn locate(&self, id: &str, mapping: &Mapping) -> Option<Destination> {
        let (category, item_id) = id.split_once('.')?;

        let category = category.parse::<usize>().ok()?;
        let item_id = item_id.parse::<usize>().ok()?;

        let area_ref = self.areas[category / 10].as_ref()?;
        let category_ref = area_ref.categories[category % 10].as_ref()?;

        let item = category_ref.items[item_id].as_ref().map(|i| *i.clone())?;

        let base_path = if let Some(over) = mapping.overrides.get(&item.category) {
            match &over.destination {
                Destination::Path(p) => p,
            }
        } else {
            &mapping.default_root
        };

        let category_path = base_path
            .join(PathBuf::from(format!("{}", area_ref)))
            .join(PathBuf::from(format!("{}", category_ref)));

        if !category_path.exists() {
            fs::create_dir_all(&category_path).ok()?;
        }

        let dst = category_path.join(PathBuf::from(format!("{}", item)));

        Some(Destination::Path(dst))
    }

    pub fn alloc_item(&mut self, category: usize, name: &str) -> Result<Item> {
        let area_ref = self.areas[category / 10]
            .as_mut()
            .ok_or_else(|| anyhow!("category {} does not exist", category))?;
        let category_ref = area_ref.categories[category % 10]
            .as_mut()
            .ok_or_else(|| anyhow!("category {} does not exist", category))?;

        for i in 1..1000 {
            if category_ref.items[i].is_some() {
                continue;
            }

            let item = Item {
                category,
                id: i,
                name: String::from(name),
            };

            category_ref.items[i] = Some(Box::new(item.clone()));
            return Ok(item);
        }

        bail!("no IDs left");
    }

    pub fn add_category(&mut self, category: usize, name: &str) -> Result<()> {
        if let Some(area) = self.areas[category / 10].as_mut() {
            ensure!(
                area.categories[category % 10].as_ref().is_none(),
                "category {} already exists",
                category
            );

            area.categories[category % 10] = Some(Category::new(name, category)?);

            Ok(())
        } else {
            bail!("missing area: {}-{}", category / 10, category / 10 + 9);
        }
    }

    pub fn list_for_category(&self, category: usize) -> Result<Vec<Item>> {
        if let Some(area_ref) = self.areas[category / 10].as_ref() {
            if let Some(cat_ref) = area_ref.categories[category % 10].as_ref() {
                Ok(cat_ref.list())
            } else {
                bail!("missing category: {}", category);
            }
        } else {
            bail!("missing area: {}-{}", category / 10, category / 10 + 9);
        }
    }

    pub fn rm(&mut self, id: &str) -> Result<()> {
        let (category, item_id) = id.split_once('.').ok_or_else(|| anyhow!("invalid id"))?;

        let category = category.parse::<usize>()?;
        let item_id = item_id.parse::<usize>()?;

        let area_ref = self.areas[category / 10].as_mut().ok_or_else(|| {
            anyhow!(
                "area {}-{} does not exist",
                category / 10,
                category / 10 + 9
            )
        })?;

        let category_ref = area_ref.categories[category % 10]
            .as_mut()
            .ok_or_else(|| anyhow!("category {} does not exist", category))?;

        category_ref.items[item_id] = None;

        Ok(())
    }
}
