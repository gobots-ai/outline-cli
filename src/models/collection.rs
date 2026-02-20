use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub permission: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub documents: Option<Vec<DocumentNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentNode {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    #[serde(default)]
    pub children: Vec<DocumentNode>,
}
