use std::io::{self, BufRead, Write};

use serde::Serialize;
use serde_json::Value;

use crate::client::OutlineClient;
use crate::config::{self, Config};
use crate::error::AppError;
use crate::output::print_json;

#[derive(Serialize)]
struct AuthSaved {
    ok: bool,
    message: String,
}

#[derive(Serialize)]
struct AuthStatus {
    authenticated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<Value>,
}

pub async fn run(token: Option<String>, url: Option<String>) -> Result<(), AppError> {
    let (token, base_url) = match (token, url) {
        (Some(t), Some(u)) => (t, u),
        _ => prompt_credentials()?,
    };

    let config = Config {
        token,
        base_url,
    };
    config::save(&config)?;

    print_json(&AuthSaved {
        ok: true,
        message: "credentials saved".into(),
    });
    Ok(())
}

pub async fn status() -> Result<(), AppError> {
    let config = match config::load() {
        Ok(c) => c,
        Err(AppError::NotAuthenticated) => {
            print_json(&AuthStatus {
                authenticated: false,
                base_url: None,
                user: None,
            });
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    let client = OutlineClient::from_config(&config);
    match client.post::<Value>("auth.info", &serde_json::json!({})).await {
        Ok(user_data) => {
            print_json(&AuthStatus {
                authenticated: true,
                base_url: Some(config.base_url),
                user: Some(user_data),
            });
        }
        Err(e) => {
            print_json(&AuthStatus {
                authenticated: false,
                base_url: Some(config.base_url),
                user: None,
            });
            eprintln!("{}", e);
        }
    }
    Ok(())
}

fn prompt_credentials() -> Result<(String, String), AppError> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Base URL (e.g. https://wiki.example.com): ")?;
    stdout.flush()?;
    let mut base_url = String::new();
    stdin.lock().read_line(&mut base_url)?;
    let base_url = base_url.trim().to_string();
    if base_url.is_empty() {
        return Err(AppError::ConfigError("base URL is required".into()));
    }

    write!(stdout, "API Token: ")?;
    stdout.flush()?;
    let mut token = String::new();
    stdin.lock().read_line(&mut token)?;
    let token = token.trim().to_string();
    if token.is_empty() {
        return Err(AppError::ConfigError("token is required".into()));
    }

    Ok((token, base_url))
}
