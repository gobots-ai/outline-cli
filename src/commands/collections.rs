use serde_json::json;

use crate::client::OutlineClient;
use crate::error::AppError;
use crate::models::collection::{Collection, DocumentNode};
use crate::output::print_json;

pub async fn list(
    client: &OutlineClient,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<(), AppError> {
    let mut body = json!({});
    if let Some(o) = offset {
        body["offset"] = json!(o);
    }
    if let Some(l) = limit {
        body["limit"] = json!(l);
    }

    let collections: Vec<Collection> = client.post("collections.list", &body).await?;
    print_json(&collections);
    Ok(())
}

pub async fn get(client: &OutlineClient, id: &str) -> Result<(), AppError> {
    let collection: Collection = client
        .post("collections.info", &json!({ "id": id }))
        .await?;
    print_json(&collection);
    Ok(())
}

pub async fn documents(client: &OutlineClient, id: &str) -> Result<(), AppError> {
    let mut tree: Vec<DocumentNode> = client
        .post("collections.documents", &json!({ "id": id }))
        .await?;
    for node in &mut tree {
        node.set_full_url(client.base_url());
    }
    print_json(&tree);
    Ok(())
}
