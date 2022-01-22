use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, bail, ensure, Result};

use async_stream::try_stream;
use futures::{Stream, StreamExt};

use menmos_client::{Client as Menmos, Type};
use menmos_client::{Meta, Query};

use crate::LocationResolver;
use crate::{Index, Item, Location, ID};

use super::util;

const JD_ITEM_TAG: &str = "jd_item";
const JD_ID_META: &str = "jd_id";
const JD_PARENT_ID_META: &str = "jd_parent_id";

pub struct MenmosResolver {
    mmos: Arc<Menmos>,
}

impl MenmosResolver {
    pub async fn new(profile: &str) -> Result<Self> {
        let mmos = Arc::new(
            Menmos::builder()
                .with_metadata_detection()
                .with_profile(profile)
                .build()
                .await?,
        );

        Ok(Self { mmos })
    }

    async fn upload_dir(&self, src_dir: PathBuf, item: &Item) -> Result<()> {
        // Step 1 - Upload the directory
        let parent_blob_id = self
            .mmos
            .create_empty(Meta::directory(&item.name).with_meta(JD_ID_META, item.id.to_string()))
            .await?;

        // TODO: Recursive & parallel
        let tags = vec![String::from(JD_ITEM_TAG)];
        let mut meta_map = HashMap::new();
        meta_map.insert(String::from(JD_PARENT_ID_META), item.id.to_string());

        let file_stream = util::get_file_stream(
            self.mmos.clone(),
            src_dir,
            parent_blob_id,
            tags.clone(),
            meta_map.clone(),
        );

        let puts = file_stream
            .filter_map(|result| async move {
                match result {
                    Ok(pair) => Some(pair),
                    Err(e) => {
                        eprintln!("filestream error: {}", e);
                        None
                    }
                }
            })
            .map(|(parent_maybe, file_path)| {
                let cloned_client = self.mmos.clone();
                let tags_cloned = tags.clone();
                let meta_cloned = meta_map.clone();
                async move {
                    util::file(
                        file_path,
                        cloned_client,
                        tags_cloned,
                        meta_cloned,
                        Type::File,
                        parent_maybe,
                    )
                    .await?;
                    Ok(())
                }
            })
            .buffered(8)
            .collect::<Vec<Result<()>>>()
            .await;

        // Catch any errors that occurred.
        puts.into_iter().collect::<Result<Vec<()>>>()?;

        Ok(())
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

    async fn set(&self, item: &Item, src_location: Location, _index: &Index) -> Result<()> {
        match src_location {
            Location::URL(_) => Ok(()),
            Location::Path(src_dir) => self.upload_dir(src_dir, item).await,
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
