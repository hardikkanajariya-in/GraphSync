use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;
use tauri::{AppHandle, Manager, State};
use tracing::info;

use crate::config::{load_config, public_config, save_config, Config, PublicConfig};
use crate::device::{self, validate_api_url, validate_sync_folder};
use crate::network::api::ApiClient;
use crate::sync::{spawn_engine, SyncEngine, SyncEngineHandle, SyncStatus};

pub struct AppState {
    pub config: Arc<RwLock<Config>>,
    pub engine: Arc<RwLock<Option<SyncEngineHandle>>>,
    pub sync: Arc<RwLock<Option<Arc<SyncEngine>>>>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: Arc::new(RwLock::new(load_config()?)),
            engine: Arc::new(RwLock::new(None)),
            sync: Arc::new(RwLock::new(None)),
        })
    }
}

#[tauri::command]
pub fn get_config(state: State<'_, AppState>) -> PublicConfig {
    public_config(&state.config.read())
}

#[tauri::command]
pub fn get_status(state: State<'_, AppState>) -> SyncStatus {
    if let Some(engine) = state.sync.read().clone() {
        engine.status()
    } else {
        let cfg = state.config.read();
        SyncStatus {
            status: if cfg.setup_complete {
                "idle".into()
            } else {
                "setup".into()
            },
            connected: false,
            paused: cfg.paused,
            last_sync: None,
            folder: cfg.sync_folder.clone(),
            device_name: cfg.device_name.clone(),
            pending_events: 0,
            failed_events: 0,
        }
    }
}

#[tauri::command]
pub async fn save_setup(
    api_url: String,
    sync_folder: String,
    device_name: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<PublicConfig, String> {
    validate_api_url(&api_url).map_err(|e| e.to_string())?;
    validate_sync_folder(&sync_folder).map_err(|e| e.to_string())?;

    {
        let mut config = state.config.write();
        config.api_url = api_url.trim().trim_end_matches('/').to_string();
        config.sync_folder = Some(sync_folder);
        config.device_name = device_name;
        config.setup_complete = true;
        config.enabled = true;
        save_config(&config).map_err(|e| e.to_string())?;
    }

    start_sync_internal(app, state.clone()).await?;
    Ok(public_config(&state.config.read()))
}

async fn start_sync_internal(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    if state.engine.read().is_some() {
        return Ok(());
    }

    let config = state.config.clone();
    let engine = SyncEngine::new(app.clone(), config.clone());
    state.sync.write().replace(engine.clone());

    let handle = spawn_engine(engine);
    state.engine.write().replace(handle);

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    info!("sync started from UI");
    Ok(())
}

#[tauri::command]
pub async fn start_sync(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    start_sync_internal(app, state).await
}

#[tauri::command]
pub fn pause_sync(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(engine) = state.sync.read().clone() {
        engine.pause().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn resume_sync(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(engine) = state.sync.read().clone() {
        engine.resume().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn update_settings(
    api_url: Option<String>,
    sync_folder: Option<String>,
    device_name: Option<String>,
    state: State<'_, AppState>,
) -> Result<PublicConfig, String> {
    {
        let mut config = state.config.write();
        if let Some(url) = api_url {
            validate_api_url(&url).map_err(|e| e.to_string())?;
            config.api_url = url.trim().trim_end_matches('/').to_string();
        }
        if let Some(folder) = sync_folder {
            validate_sync_folder(&folder).map_err(|e| e.to_string())?;
            config.sync_folder = Some(folder);
        }
        if let Some(name) = device_name {
            config.device_name = name;
        }
        save_config(&config).map_err(|e| e.to_string())?;
    }
    Ok(public_config(&state.config.read()))
}

#[tauri::command]
pub async fn reset_device(state: State<'_, AppState>) -> Result<PublicConfig, String> {
    let api = ApiClient::new(state.config.clone());
    let mut config = state.config.read().clone();
    device::reset_device_identity(&mut config, &api)
        .await
        .map_err(|e| e.to_string())?;
    *state.config.write() = config.clone();
    Ok(public_config(&config))
}

#[tauri::command]
pub fn get_log_path() -> Result<String, String> {
    crate::config::log_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn show_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn hide_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn is_setup_complete(state: State<'_, AppState>) -> bool {
    state.config.read().setup_complete
}
