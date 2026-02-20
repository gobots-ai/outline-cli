use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not authenticated — run `outline auth` to configure credentials")]
    NotAuthenticated,

    #[error("API error: {message}")]
    ApiError { status: u16, message: String },

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("config error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Serialize)]
struct ErrorOutput {
    error: String,
    message: String,
}

impl AppError {
    pub fn code(&self) -> &str {
        match self {
            AppError::NotAuthenticated => "not_authenticated",
            AppError::ApiError { .. } => "api_error",
            AppError::HttpError(_) => "http_error",
            AppError::ConfigError(_) => "config_error",
            AppError::IoError(_) => "io_error",
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::NotAuthenticated => 2,
            AppError::ApiError { .. } => 3,
            AppError::HttpError(_) => 4,
            AppError::ConfigError(_) => 5,
            AppError::IoError(_) => 6,
        }
    }

    pub fn to_json(&self) -> String {
        let output = ErrorOutput {
            error: self.code().to_string(),
            message: self.to_string(),
        };
        serde_json::to_string(&output).unwrap_or_else(|_| {
            format!(r#"{{"error":"internal","message":"{}"}}"#, self)
        })
    }
}
