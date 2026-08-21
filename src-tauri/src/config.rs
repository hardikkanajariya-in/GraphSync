use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnServer {
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub sync_folder: Option<String>,
    pub device_id: String,
    pub device_name: String,
    pub graph_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub paused: bool,
    #[serde(default)]
    pub setup_complete: bool,
    #[serde(default = "default_stun")]
    pub stun_servers: Vec<String>,
    #[serde(default)]
    pub turn_servers: Vec<TurnServer>,
    #[serde(default = "default_protocol_version")]
    pub protocol_version: u32,
}

fn default_enabled() -> bool {
    true
}

fn default_protocol_version() -> u32 {
    PROTOCOL_VERSION
}

fn default_stun() -> Vec<String> {
    vec!["stun:stun.l.google.com:19302".to_string()]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: String::new(),
            sync_folder: None,
            device_id: Uuid::new_v4().to_string(),
            device_name: default_device_name(),
            graph_id: None,
            auth_token: None,
            enabled: true,
            paused: false,
            setup_complete: false,
            stun_servers: default_stun(),
            turn_servers: Vec::new(),
            protocol_version: PROTOCOL_VERSION,
        }
    }
}

fn default_device_name() -> String {
    let host = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "GraphSync Device".to_string());
    host
}

pub fn config_dir() -> Result<PathBuf> {
    let dir = dirs::data_dir()
        .context("unable to resolve application data directory")?
        .join("GraphSync");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

pub fn state_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("sync_state.json"))
}

pub fn log_dir() -> Result<PathBuf> {
    let dir = config_dir()?.join("logs");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        let config = Config::default();
        save_config(&config)?;
        return Ok(config);
    }

    let data = std::fs::read_to_string(&path)?;
    let mut config: Config = serde_json::from_str(&data)?;
    if config.device_id.is_empty() {
        config.device_id = Uuid::new_v4().to_string();
    }
    if config.device_name.is_empty() {
        config.device_name = default_device_name();
    }
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    let data = serde_json::to_string_pretty(config)?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, data)?;
    std::fs::rename(tmp, path)?;
    Ok(())
}

pub fn public_config(config: &Config) -> PublicConfig {
    PublicConfig {
        api_url: config.api_url.clone(),
        sync_folder: config.sync_folder.clone(),
        device_id: config.device_id.clone(),
        device_name: config.device_name.clone(),
        graph_id: config.graph_id.clone(),
        enabled: config.enabled,
        paused: config.paused,
        setup_complete: config.setup_complete,
        stun_servers: config.stun_servers.clone(),
        turn_servers: config.turn_servers.clone(),
        protocol_version: config.protocol_version,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicConfig {
    pub api_url: String,
    pub sync_folder: Option<String>,
    pub device_id: String,
    pub device_name: String,
    pub graph_id: Option<String>,
    pub enabled: bool,
    pub paused: bool,
    pub setup_complete: bool,
    pub stun_servers: Vec<String>,
    pub turn_servers: Vec<TurnServer>,
    pub protocol_version: u32,
}
