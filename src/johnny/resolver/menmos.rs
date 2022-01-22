use std::path::PathBuf;

use anyhow::{anyhow, bail, ensure, Result};

use futures::future::LocalBoxFuture;
use menmos_client::Client as Menmos;
use menmos_client::{Meta, Query};

use super::LocationResolver;
use crate::{Index, Item, Location, ID};

const JD_ITEM_TAG: &str = "jd_item";
const JD_ID_META: &str = "jd_id";

pub struct MenmosResolver {
    mmos: Menmos,
}

impl MenmosResolver {
    pub async fn new(profile: &str) -> Result<Self> {
        let mmos = Menmos::new_with_profile(profile).await?;
        Ok(Self { mmos })
    }
}

#[async_trait::async_trait]
impl LocationResolver for MenmosResolver {
    async fn get(&self, item: &Item, _index: &Index) -> Result<Option<Location>> {
        let results = self
            .mmos
            .query(
                Query::default()
                    .and_meta(JD_ID_META, item.id.to_string())
                    .with_size(1),
            )
            .await?;

        match results.hits.into_iter().next() {
            Some(result) => Ok(Some(Location::URL(result.url))),
            None => Ok(None),
        }
    }

    async fn collect(&self, _index: &mut Index) -> Result<()> {
        // TODO: Implement
        Ok(())
    }

    async fn set(&self, item: &Item, src_location: Location, index: &Index) -> Result<()> {
        match src_location {
            Location::URL(_) => {
                unimplemented!("url download not implemented yet");
            }
            Location::Path(src_dir) => {
                // Step 1 - Upload the directory
                let parent_blob_id = self.mmos.create_empty(Meta::directory(&item.name)).await?;

                // TODO: Recursive & parallel
                for entry in std::fs::read_dir(src_dir)?.filter_map(|e| e.ok()) {
                    let blob_id = self
                        .mmos
                        .push(
                            entry.path(),
                            Meta::file("test_file")
                                .with_parent(&parent_blob_id)
                                .with_tag(JD_ID_META),
                        )
                        .await?;

                    println!("uploaded blob id: {blob_id}");
                }

                Ok(())
            }
        }
    }

    async fn remove(&self, _id: &Item, _index: &Index) -> Result<()> {
        unimplemented!()
    }

    async fn rename_category(
        &self,
        _category: usize,
        _new_name: &str,
        _index: &Index,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn rename_item(&self, _old_item: &Item, _new_item: &Item, _index: &Index) -> Result<()> {
        unimplemented!();
    }
}
