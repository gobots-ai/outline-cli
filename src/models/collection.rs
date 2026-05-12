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
    pub url_id: Option<String>,
    #[serde(default)]
    pub full_url: Option<String>,
    #[serde(default)]
    pub children: Vec<DocumentNode>,
}

impl DocumentNode {
    pub fn set_full_url(&mut self, base_url: &str) {
        if let Some(path) = &self.url {
            self.full_url = Some(format!("{}{}", base_url.trim_end_matches('/'), path));
        }
        for child in &mut self.children {
            child.set_full_url(base_url);
        }
    }
}
