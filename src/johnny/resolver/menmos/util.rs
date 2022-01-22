use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;

use async_stream::try_stream;

use futures::Stream;

use menmos_client::{Client as Menmos, Meta, Type};

pub async fn file<P: AsRef<Path>>(
    path: P,
    client: Arc<Menmos>,
    tags: Vec<String>,
    meta_map: HashMap<String, String>,
    blob_type: Type,
    parent: String,
) -> Result<String> {
    let mut meta = Meta::new(
        path.as_ref()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        blob_type.clone(),
    )
    .with_parent(parent);

    if blob_type == Type::File {
        meta = meta.with_size(path.as_ref().metadata().unwrap().len())
    }

    for tag in tags.iter() {
        meta = meta.with_tag(tag);
    }

    for (k, v) in meta_map.iter() {
        meta = meta.with_meta(k, v);
    }

    let item_id = client.push(path.as_ref(), meta).await?;

    Ok(item_id)
}

pub fn get_file_stream(
    client: Arc<Menmos>,
    path: PathBuf,
    parent_id: String,
    tags: Vec<String>,
    meta_map: HashMap<String, String>,
) -> impl Stream<Item = Result<(String, PathBuf)>> {
    // Convert a non-recursive (stack based) directory traversal to a stream
    try_stream! {
        let mut working_stack = vec![];

        for child in path.read_dir()?.filter_map(|f| f.ok()) {
            working_stack.push((parent_id.clone(), child.path()));
        }

        while !working_stack.is_empty() {
            let (parent_maybe, file_path) = working_stack.pop().unwrap();

            if file_path.is_file() {
                yield (parent_maybe, file_path); // File can be uploaded directly.
            } else {
                let directory_id = file(
                    file_path.clone(),
                    client.clone(),
                    tags.clone(),
                    meta_map.clone(),
                    Type::Directory,
                    parent_maybe,
                )
                .await?;

                // Add this directory's children to the working stack.
                for child in file_path.read_dir()?.filter_map(|f| f.ok()) {
                    working_stack.push((directory_id.clone(), child.path()));
                }
            }
        }
    }
}
