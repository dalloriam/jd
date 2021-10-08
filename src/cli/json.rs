use anyhow::{anyhow, Result};

use johnny::{Category, Item, JohnnyDecimal};

use serde::Serialize;

#[derive(Serialize)]
pub struct CategoryView {
    id: usize,
    name: String,
}

#[derive(Serialize)]
pub struct ItemView {
    id: String,
    name: String,
    category_name: String,
    category: CategoryView,
}

pub struct Viewer<'a> {
    jd: &'a JohnnyDecimal,
}

impl<'a> Viewer<'a> {
    pub fn new(jd: &'a JohnnyDecimal) -> Self {
        Self { jd }
    }

    pub fn category(&self, category: &Category) -> CategoryView {
        CategoryView {
            id: category.id,
            name: category.name.clone(),
        }
    }

    pub fn item(&self, item: &Item) -> Result<ItemView> {
        let area = self
            .jd
            .index
            .get_area_from_category(item.id.category)?
            .ok_or_else(|| anyhow!("missing area"))?;

        let category = area
            .get_category(item.id.category)?
            .ok_or_else(|| anyhow!("missing category"))?;

        let view = ItemView {
            id: format!("{}", item.id),
            category_name: format!("{}", category),
            category: self.category(category),
            name: item.name.clone(),
        };

        Ok(view)
    }
}
