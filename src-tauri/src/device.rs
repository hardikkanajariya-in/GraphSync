use std::path::Path;

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::network::api::ApiClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub platform: String,
    pub registered_at: DateTime<Utc>,
}

pub fn platform_name() -> String {
    std::env::consts::OS.to_string()
}

pub async fn ensure_registered(config: &mut Config, api: &ApiClient) -> Result<DeviceInfo> {
    let auth = api.authenticate_device(&config.device_id).await?;
    config.auth_token = Some(auth.token);
    save_config_token(config)?;

    let device = api
        .register_device(
            &config.device_id,
            &config.device_name,
            &platform_name(),
        )
        .await?;

    Ok(device)
}

pub async fn reset_device_identity(config: &mut Config, api: &ApiClient) -> Result<DeviceInfo> {
    config.device_id = uuid::Uuid::new_v4().to_string();
    config.graph_id = None;
    config.auth_token = None;
    config.setup_complete = false;
    crate::config::save_config(config)?;
    ensure_registered(config, api).await
}

fn save_config_token(config: &Config) -> Result<()> {
    crate::config::save_config(config)
}

pub fn validate_api_url(url: &str) -> Result<()> {
    let parsed = url::Url::parse(url)?;
    match parsed.scheme() {
        "https" | "http" => Ok(()),
        other => bail!("unsupported API URL scheme: {other}"),
    }
}

pub fn validate_sync_folder(path: &str) -> Result<()> {
    let p = Path::new(path);
    if !p.is_absolute() {
        bail!("sync folder must be an absolute path");
    }
    if !p.exists() {
        bail!("sync folder does not exist");
    }
    if !p.is_dir() {
        bail!("sync folder must be a directory");
    }
    Ok(())
}
