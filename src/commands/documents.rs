use serde::Serialize;
use serde_json::json;

use crate::client::OutlineClient;
use crate::error::AppError;
use crate::models::document::{Document, SearchResult};
use crate::output::print_json;

pub async fn list(
    client: &OutlineClient,
    collection_id: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<(), AppError> {
    let mut body = json!({});
    if let Some(id) = collection_id {
        body["collectionId"] = json!(id);
    }
    if let Some(o) = offset {
        body["offset"] = json!(o);
    }
    if let Some(l) = limit {
        body["limit"] = json!(l);
    }

    let mut docs: Vec<Document> = client.post("documents.list", &body).await?;
    for d in &mut docs {
        d.set_full_url(client.base_url());
    }
    print_json(&docs);
    Ok(())
}

pub async fn get(client: &OutlineClient, id: &str) -> Result<(), AppError> {
    let mut doc: Document = client.post("documents.info", &json!({ "id": id })).await?;
    doc.set_full_url(client.base_url());
    print_json(&doc);
    Ok(())
}

pub async fn create(
    client: &OutlineClient,
    title: &str,
    collection_id: &str,
    text: Option<String>,
) -> Result<(), AppError> {
    let mut body = json!({
        "title": title,
        "collectionId": collection_id,
    });
    if let Some(t) = text {
        body["text"] = json!(t);
    }

    let mut doc: Document = client.post("documents.create", &body).await?;
    doc.set_full_url(client.base_url());
    print_json(&doc);
    Ok(())
}

pub async fn update(
    client: &OutlineClient,
    id: &str,
    title: Option<String>,
    text: Option<String>,
) -> Result<(), AppError> {
    let mut body = json!({ "id": id });
    if let Some(t) = title {
        body["title"] = json!(t);
    }
    if let Some(t) = text {
        body["text"] = json!(t);
    }

    let mut doc: Document = client.post("documents.update", &body).await?;
    doc.set_full_url(client.base_url());
    print_json(&doc);
    Ok(())
}

#[derive(Serialize)]
struct DeleteResult {
    ok: bool,
    message: String,
}

pub async fn delete(
    client: &OutlineClient,
    id: &str,
    permanent: bool,
) -> Result<(), AppError> {
    let mut body = json!({ "id": id });
    if permanent {
        body["permanent"] = json!(true);
    }

    let _: serde_json::Value = client.post("documents.delete", &body).await?;
    print_json(&DeleteResult {
        ok: true,
        message: format!(
            "document {} {}",
            id,
            if permanent { "permanently deleted" } else { "deleted" }
        ),
    });
    Ok(())
}

pub async fn search(
    client: &OutlineClient,
    query: &str,
    collection_id: Option<String>,
    limit: Option<u32>,
) -> Result<(), AppError> {
    let mut body = json!({ "query": query });
    if let Some(id) = collection_id {
        body["collectionId"] = json!(id);
    }
    if let Some(l) = limit {
        body["limit"] = json!(l);
    }

    let mut results: Vec<SearchResult> = client.post("documents.search", &body).await?;
    for r in &mut results {
        r.document.set_full_url(client.base_url());
    }
    print_json(&results);
    Ok(())
}

pub async fn export(client: &OutlineClient, id: &str) -> Result<(), AppError> {
    let text = client
        .post_text("documents.export", &json!({ "id": id }))
        .await?;

    // Export may return JSON with data or raw markdown
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
        if let Some(data) = parsed.get("data") {
            print_json(data);
            return Ok(());
        }
        print_json(&parsed);
    } else {
        // Raw markdown output
        println!("{text}");
    }
    Ok(())
}
