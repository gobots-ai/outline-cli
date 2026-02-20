use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub base_url: String,
}

fn config_path() -> Result<PathBuf, AppError> {
    let dir = dirs::config_dir()
        .ok_or_else(|| AppError::ConfigError("cannot determine config directory".into()))?
        .join("outline-cli");
    Ok(dir.join("config.toml"))
}

pub fn load() -> Result<Config, AppError> {
    let path = config_path()?;
    if !path.exists() {
        return Err(AppError::NotAuthenticated);
    }
    let content = fs::read_to_string(&path).map_err(|e| {
        AppError::ConfigError(format!("failed to read {}: {e}", path.display()))
    })?;
    let config: Config = toml::from_str(&content).map_err(|e| {
        AppError::ConfigError(format!("invalid config: {e}"))
    })?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<(), AppError> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            AppError::ConfigError(format!("failed to create {}: {e}", parent.display()))
        })?;
    }
    let content = toml::to_string_pretty(config).map_err(|e| {
        AppError::ConfigError(format!("failed to serialize config: {e}"))
    })?;
    fs::write(&path, content).map_err(|e| {
        AppError::ConfigError(format!("failed to write {}: {e}", path.display()))
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_save_and_load() {
        let tmp = env::temp_dir().join("outline-cli-test-config");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let config_file = tmp.join("config.toml");
        let config = Config {
            token: "test-token".into(),
            base_url: "https://wiki.example.com".into(),
        };

        let content = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_file, &content).unwrap();

        let loaded: Config = toml::from_str(&fs::read_to_string(&config_file).unwrap()).unwrap();
        assert_eq!(loaded.token, "test-token");
        assert_eq!(loaded.base_url, "https://wiki.example.com");

        let _ = fs::remove_dir_all(&tmp);
    }
}
