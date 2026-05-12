use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub emoji: Option<String>,
    pub collection_id: Option<String>,
    pub parent_document_id: Option<String>,
    pub template: Option<bool>,
    pub published_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub created_by: Option<UserRef>,
    pub updated_by: Option<UserRef>,
    pub revision: Option<i64>,
    #[serde(default)]
    pub url_id: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub full_url: Option<String>,
}

impl Document {
    pub fn set_full_url(&mut self, base_url: &str) {
        if let Some(path) = &self.url {
            self.full_url = Some(format!("{}{}", base_url.trim_end_matches('/'), path));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRef {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub ranking: Option<f64>,
    pub context: Option<String>,
    pub document: Document,
}
