use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use parking_lot::{Mutex as SyncMutex, RwLock};
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info, warn};

use crate::config::Config;
use crate::device;
use crate::filesystem;
use crate::network::api::ApiClient;
use crate::network::peer::PeerManager;
use crate::network::websocket::WebSocketClient;
use crate::sync::conflict::{
    apply_remote_delete_with_conflict_check, apply_remote_with_conflict_check,
};
use crate::sync::events::{FileOperation, PendingLocalChange, SyncEvent};
use crate::sync::state::{LocalState, ManifestEntry};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatus {
    pub status: String,
    pub connected: bool,
    pub paused: bool,
    pub last_sync: Option<String>,
    pub folder: Option<String>,
    pub device_name: String,
    pub pending_events: usize,
    pub failed_events: usize,
}

pub struct SyncEngine {
    config: Arc<RwLock<Config>>,
    state: Arc<RwLock<LocalState>>,
    api: Arc<ApiClient>,
    ws: Arc<WebSocketClient>,
    pub peers: Arc<PeerManager>,
    app: AppHandle,
    status: Arc<RwLock<SyncStatus>>,
    retry_queue: Arc<Mutex<VecDeque<SyncEvent>>>,
    watcher: SyncMutex<Option<Debouncer<notify::RecommendedWatcher, FileIdMap>>>,
    pub shutdown: tokio::sync::watch::Sender<bool>,
}

pub struct SyncEngineHandle {
    pub shutdown: tokio::sync::watch::Sender<bool>,
    pub join: tokio::task::JoinHandle<()>,
}

impl SyncEngine {
    pub fn new(app: AppHandle, config: Arc<RwLock<Config>>) -> Arc<Self> {
        let api = Arc::new(ApiClient::new(config.clone()));
        let ws = Arc::new(WebSocketClient::new(config.clone(), api.clone()));
        let peers = Arc::new(PeerManager::new(config.clone(), api.clone()));
        let (shutdown, _) = tokio::sync::watch::channel(false);

        Arc::new(Self {
            config,
            state: Arc::new(RwLock::new(LocalState::load().unwrap_or_default())),
            api,
            ws,
            peers,
            app,
            status: Arc::new(RwLock::new(SyncStatus {
                status: "idle".into(),
                connected: false,
                paused: false,
                last_sync: None,
                folder: None,
                device_name: String::new(),
                pending_events: 0,
                failed_events: 0,
            })),
            retry_queue: Arc::new(Mutex::new(VecDeque::new())),
            watcher: SyncMutex::new(None),
            shutdown,
        })
    }

    pub fn status(&self) -> SyncStatus {
        self.status.read().clone()
    }

    pub fn update_status<F>(&self, f: F)
    where
        F: FnOnce(&mut SyncStatus),
    {
        let mut status = self.status.write();
        f(&mut status);
        let _ = self.app.emit("sync-status", status.clone());
    }

    pub async fn start(self: &Arc<Self>) -> Result<()> {
        let cfg = self.config.read().clone();
        if !cfg.setup_complete {
            anyhow::bail!("setup is not complete");
        }
        let folder = cfg
            .sync_folder
            .clone()
            .context("sync folder not configured")?;
        device::validate_sync_folder(&folder)?;
        device::validate_api_url(&cfg.api_url)?;

        self.update_status(|s| {
            s.folder = Some(folder.clone());
            s.device_name = cfg.device_name.clone();
            s.paused = cfg.paused;
            s.status = if cfg.paused {
                "paused".into()
            } else {
                "starting".into()
            };
        });

        {
            let mut config = self.config.read().clone();
            device::ensure_registered(&mut config, &self.api).await?;
            *self.config.write() = config;
        }

        let graph_id = if self.config.read().graph_id.is_some() {
            self.config.read().graph_id.clone().unwrap()
        } else {
            let device_id = self.config.read().device_id.clone();
            let graph = self
                .api
                .register_graph(&device_id, &folder)
                .await?;
            let graph_id = graph.graph_id.clone();
            {
                let mut config = self.config.write();
                config.graph_id = Some(graph_id.clone());
                crate::config::save_config(&config)?;
            }
            graph_id
        };

        {
            let mut state = self.state.write();
            if state.files.is_empty() {
                state.scan_folder(Path::new(&folder), &graph_id)?;
            } else {
                state.graph_id = Some(graph_id.clone());
                state.save()?;
            }
        }

        self.bootstrap_manifest(&folder, &graph_id).await?;
        self.ws.connect(self.clone()).await?;
        self.peers.start(self.clone()).await?;
        self.start_watcher(folder.clone())?;
        self.spawn_background_loop();

        self.update_status(|s| {
            s.connected = true;
            s.status = "synced".into();
            s.last_sync = Some("Just now".into());
        });
        let _ = self.app.emit("connection-status", "connected");
        info!("sync engine started");
        Ok(())
    }

    async fn bootstrap_manifest(&self, folder: &str, graph_id: &str) -> Result<()> {
        let local = self.state.read().build_manifest();
        let remote = self.api.get_manifest(graph_id).await.unwrap_or_default();
        let missing = diff_manifests(&local, &remote);

        for entry in missing {
            if let Ok(data) = self
                .peers
                .request_file_from_any_peer(graph_id, &entry.path, &entry.sha256)
                .await
            {
                self.apply_downloaded_file(folder, &entry.path, &entry.sha256, data)
                    .await?;
            }
        }
        Ok(())
    }

    fn start_watcher(&self, folder: String) -> Result<()> {
        let engine = self.clone();
        let root = PathBuf::from(folder.clone());
        let (tx, mut rx) = mpsc::unbounded_channel();

        let mut debouncer = new_debouncer(
            Duration::from_millis(750),
            None,
            move |result: DebounceEventResult| {
                let _ = tx.send(result);
            },
        )?;

        debouncer.watch(&root, RecursiveMode::Recursive)?;

        tokio::spawn(async move {
            while let Some(result) = rx.recv().await {
                if *engine.shutdown.borrow() {
                    break;
                }
                let cfg = engine.config.read().clone();
                if cfg.paused || !cfg.enabled {
                    continue;
                }
                if let Err(err) = engine.handle_watch_result(&folder, result).await {
                    error!(error = %err, "filesystem watch handling failed");
                }
            }
        });

        *self.watcher.lock() = Some(debouncer);
        info!(folder = %folder, "watching folder");
        Ok(())
    }

    async fn handle_watch_result(
        &self,
        folder: &str,
        result: DebounceEventResult,
    ) -> Result<()> {
        let Ok(events) = result else {
            return Ok(());
        };

        for event in events {
            for path in event.event.paths {
                if filesystem::is_temp_or_ignored(&path) {
                    continue;
                }
                let root = Path::new(folder);
                if !path.starts_with(root) {
                    continue;
                }

                if event.event.kind.is_remove() {
                    if let Ok(relative) = filesystem::relative_path_from_root(root, &path) {
                        self.record_local_delete(&relative).await?;
                    }
                    continue;
                }

                if path.is_file() {
                    if let Ok(relative) = filesystem::relative_path_from_root(root, &path) {
                        self.record_local_upsert(root, &relative, FileOperation::Update)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn record_local_upsert(
        &self,
        root: &Path,
        relative: &str,
        operation: FileOperation,
    ) -> Result<()> {
        let full = filesystem::resolve_safe_path(root, relative)?;
        if !full.exists() {
            return Ok(());
        }
        let (hash, size, mtime) = filesystem::hash_file(&full)?;
        let revision;
        {
            let mut state = self.state.write();
            revision = state.next_revision(relative);
            state.upsert_file(relative, &hash, size, mtime, revision, operation);
            state.save()?;
        }

        let cfg = self.config.read().clone();
        let graph_id = cfg.graph_id.clone().context("missing graph id")?;
        let change = PendingLocalChange {
            path: relative.to_string(),
            old_path: None,
            operation,
            sha256: Some(hash.clone()),
            size,
            revision,
        };
        let event = SyncEvent::from_local(&graph_id, &cfg.device_id, change);
        info!(path = %relative, operation = ?operation, "detected local change");
        let _ = self.app.emit(
            "file-sync-started",
            serde_json::json!({ "path": relative, "operation": format!("{:?}", operation) }),
        );
        self.publish_event(event).await?;
        Ok(())
    }

    async fn record_local_delete(&self, relative: &str) -> Result<()> {
        let revision;
        {
            let mut state = self.state.write();
            revision = state.next_revision(relative);
            state.mark_deleted(relative, revision);
            state.save()?;
        }

        let cfg = self.config.read().clone();
        let graph_id = cfg.graph_id.clone().context("missing graph id")?;
        let change = PendingLocalChange {
            path: relative.to_string(),
            old_path: None,
            operation: FileOperation::Delete,
            sha256: None,
            size: 0,
            revision,
        };
        let event = SyncEvent::from_local(&graph_id, &cfg.device_id, change);
        info!(path = %relative, "detected local deletion");
        self.publish_event(event).await?;
        Ok(())
    }

    pub async fn publish_event(&self, event: SyncEvent) -> Result<()> {
        if self.config.read().paused {
            return Ok(());
        }

        match self.api.post_sync_event(&event).await {
            Ok(_) => {
                self.update_status(|s| {
                    s.last_sync = Some("Just now".into());
                    s.status = "synced".into();
                });
                let _ = self.app.emit(
                    "file-sync-completed",
                    serde_json::json!({ "path": event.path, "eventId": event.event_id }),
                );
            }
            Err(err) => {
                warn!(error = %err, path = %event.path, "failed to publish sync event");
                self.retry_queue.lock().await.push_back(event.clone());
                self.update_status(|s| {
                    s.failed_events += 1;
                    s.status = "degraded".into();
                });
                let _ = self.app.emit(
                    "file-sync-failed",
                    serde_json::json!({ "path": event.path, "error": err.to_string() }),
                );
            }
        }
        Ok(())
    }

    pub async fn apply_remote_event(&self, event: SyncEvent) -> Result<()> {
        if event.device_id == self.config.read().device_id {
            return Ok(());
        }

        let folder = self
            .config
            .read()
            .sync_folder
            .clone()
            .context("sync folder missing")?;
        let root = Path::new(&folder);
        let device_name = self.config.read().device_name.clone();

        match event.operation {
            FileOperation::Delete => {
                let local_entry = self.state.read().files.get(&event.path).cloned();
                let local_unsynced = local_entry
                    .as_ref()
                    .map(|e| !e.deleted && e.revision > event.revision)
                    .unwrap_or(false);
                apply_remote_delete_with_conflict_check(
                    root,
                    &event.path,
                    local_entry.as_ref(),
                    local_unsynced,
                    &device_name,
                )?;
                let mut state = self.state.write();
                state.mark_deleted(&event.path, event.revision);
                state.save()?;
            }
            FileOperation::Rename => {
                let old = event.old_path.clone().context("rename missing old_path")?;
                filesystem::rename_file(root, &old, &event.path)?;
                let mut state = self.state.write();
                state.rename_path(&old, &event.path, event.revision);
                state.save()?;
            }
            FileOperation::Create | FileOperation::Update => {
                let hash = event.sha256.clone().context("missing file hash")?;
                let data = self
                    .peers
                    .request_file_from_any_peer(&event.graph_id, &event.path, &hash)
                    .await?;
                self.apply_downloaded_file(&folder, &event.path, &hash, data)
                    .await?;
            }
        }

        self.api.ack_event(&event.event_id).await.ok();
        self.update_status(|s| {
            s.last_sync = Some("Just now".into());
            s.status = "synced".into();
        });
        Ok(())
    }

    async fn apply_downloaded_file(
        &self,
        folder: &str,
        relative: &str,
        expected_hash: &str,
        data: Vec<u8>,
    ) -> Result<()> {
        let root = Path::new(folder);
        let local_entry = self.state.read().files.get(relative).cloned();
        let device_name = self.config.read().device_name.clone();
        let (hash, size, mtime, _conflict) = apply_remote_with_conflict_check(
            root,
            relative,
            expected_hash,
            local_entry
                .as_ref()
                .map(|e| e.revision)
                .unwrap_or(0),
            local_entry.as_ref(),
            &device_name,
            &data,
        )?;

        if hash != expected_hash {
            anyhow::bail!("downloaded file hash mismatch for {relative}");
        }

        let mut state = self.state.write();
        state.upsert_file(
            relative,
            &hash,
            size,
            mtime,
            local_entry.map(|e| e.revision + 1).unwrap_or(1),
            FileOperation::Update,
        );
        state.save()?;
        Ok(())
    }

    fn spawn_background_loop(self: &Arc<Self>) {
        let engine = self.clone();
        tokio::spawn(async move {
            let mut backoff = Duration::from_secs(2);
            loop {
                if *engine.shutdown.borrow() {
                    break;
                }
                let paused = engine.config.read().paused;
                if paused {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }

                while let Some(event) = engine.retry_queue.lock().await.pop_front() {
                    if let Err(err) = engine.publish_event(event.clone()).await {
                        error!(error = %err, "retry queue processing failed");
                        engine.retry_queue.lock().await.push_back(event);
                        break;
                    }
                }

                let graph_id = engine.config.read().graph_id.clone();
                if let Some(graph_id) = graph_id {
                    match engine.api.get_sync_events(&graph_id, None).await {
                        Ok(events) => {
                            for event in events {
                                if let Err(err) = engine.apply_remote_event(event).await {
                                    warn!(error = %err, "failed applying remote event");
                                }
                            }
                            backoff = Duration::from_secs(2);
                        }
                        Err(err) => {
                            warn!(error = %err, "failed polling sync events");
                            tokio::time::sleep(backoff).await;
                            backoff = (backoff * 2).min(Duration::from_secs(60));
                        }
                    }
                }

                let pending = engine.retry_queue.lock().await.len();
                engine.update_status(|s| {
                    s.pending_events = pending;
                });
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    pub fn pause(&self) -> Result<()> {
        let mut config = self.config.write();
        config.paused = true;
        crate::config::save_config(&config)?;
        self.update_status(|s| {
            s.paused = true;
            s.status = "paused".into();
        });
        Ok(())
    }

    pub fn resume(&self) -> Result<()> {
        let mut config = self.config.write();
        config.paused = false;
        crate::config::save_config(&config)?;
        self.update_status(|s| {
            s.paused = false;
            s.status = "synced".into();
        });
        Ok(())
    }

    pub async fn shutdown(&self) {
        let _ = self.shutdown.send(true);
        self.ws.disconnect().await;
        *self.watcher.lock() = None;
    }
}

pub fn spawn_engine(engine: Arc<SyncEngine>) -> SyncEngineHandle {
    let shutdown = engine.shutdown.clone();
    let join = tokio::spawn(async move {
        if let Err(err) = engine.start().await {
            error!(error = %err, "sync engine failed to start");
            engine.update_status(|s| {
                s.status = "error".into();
                s.connected = false;
            });
            let _ = engine.app.emit("connection-status", "error");
        }
    });
    SyncEngineHandle { shutdown, join }
}

fn diff_manifests(local: &[ManifestEntry], remote: &[ManifestEntry]) -> Vec<ManifestEntry> {
    let local_map: HashMap<_, _> = local.iter().map(|e| (e.path.clone(), e)).collect();
    remote
        .iter()
        .filter(|remote_entry| match local_map.get(&remote_entry.path) {
            Some(local_entry) => local_entry.sha256 != remote_entry.sha256,
            None => true,
        })
        .cloned()
        .collect()
}
