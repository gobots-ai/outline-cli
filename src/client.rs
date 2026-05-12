use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::Config;
use crate::error::AppError;

pub struct OutlineClient {
    base_url: String,
    token: String,
    http: reqwest::Client,
}

#[derive(serde::Deserialize)]
struct ApiResponse<T> {
    ok: Option<bool>,
    data: Option<T>,
    message: Option<String>,
    status: Option<u16>,
}

impl OutlineClient {
    pub fn from_config(config: &Config) -> Self {
        Self {
            base_url: config.base_url.trim_end_matches('/').to_string(),
            token: config.token.clone(),
            http: reqwest::Client::new(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T, AppError> {
        let url = format!("{}/api/{}", self.base_url, endpoint.trim_start_matches('/'));
        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            if let Ok(api_resp) = serde_json::from_str::<ApiResponse<serde_json::Value>>(&text) {
                return Err(AppError::ApiError {
                    status: status.as_u16(),
                    message: api_resp
                        .message
                        .unwrap_or_else(|| format!("HTTP {status}")),
                });
            }
            return Err(AppError::ApiError {
                status: status.as_u16(),
                message: if text.is_empty() {
                    format!("HTTP {status}")
                } else {
                    text
                },
            });
        }

        let text = resp.text().await?;
        let api_resp: ApiResponse<T> = serde_json::from_str(&text).map_err(|e| {
            AppError::ApiError {
                status: status.as_u16(),
                message: format!("failed to parse response: {e}"),
            }
        })?;

        if api_resp.ok == Some(false) {
            return Err(AppError::ApiError {
                status: api_resp.status.unwrap_or(status.as_u16()),
                message: api_resp.message.unwrap_or_else(|| "unknown API error".into()),
            });
        }

        api_resp.data.ok_or_else(|| AppError::ApiError {
            status: status.as_u16(),
            message: "response missing data field".into(),
        })
    }

    /// POST that returns raw text (for export endpoint).
    pub async fn post_text(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<String, AppError> {
        let url = format!("{}/api/{}", self.base_url, endpoint.trim_start_matches('/'));
        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(AppError::ApiError {
                status: status.as_u16(),
                message: if text.is_empty() {
                    format!("HTTP {status}")
                } else {
                    text
                },
            });
        }

        Ok(resp.text().await?)
    }
}
