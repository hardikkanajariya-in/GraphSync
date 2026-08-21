use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use parking_lot::RwLock;
use tokio::sync::{Mutex, oneshot};
use tracing::{info, warn};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::data_channel::data_channel_init::RTCDataChannelInit;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;

use crate::config::Config;
use crate::filesystem;
use crate::network::api::ApiClient;
use crate::network::signaling::{FileRequestMessage, SignalingMessage};
use crate::sync::engine::SyncEngine;

const CHUNK_SIZE: usize = 64 * 1024;

pub struct PeerManager {
    config: Arc<RwLock<Config>>,
    api: Arc<ApiClient>,
    sessions: Arc<Mutex<HashMap<String, Arc<PeerSession>>>>,
    pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<Result<Vec<u8>>>>>>,
}

struct PeerSession {
    pc: Arc<RTCPeerConnection>,
}

impl PeerManager {
    pub fn new(config: Arc<RwLock<Config>>, api: Arc<ApiClient>) -> Self {
        Self {
            config,
            api,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start(&self, _engine: Arc<SyncEngine>) -> Result<()> {
        info!("peer manager started");
        Ok(())
    }

    fn rtc_config(&self) -> RTCConfiguration {
        let cfg = self.config.read();
        let mut servers = Vec::new();
        if !cfg.stun_servers.is_empty() {
            servers.push(RTCIceServer {
                urls: cfg.stun_servers.clone(),
                ..Default::default()
            });
        }
        for turn in &cfg.turn_servers {
            servers.push(RTCIceServer {
                urls: turn.urls.clone(),
                username: turn.username.clone().unwrap_or_default(),
                credential: turn.credential.clone().unwrap_or_default(),
                ..Default::default()
            });
        }
        RTCConfiguration {
            ice_servers: servers,
            ..Default::default()
        }
    }

    async fn build_api(&self) -> Result<Arc<webrtc::api::API>> {
        let mut media_engine = MediaEngine::default();
        media_engine.register_default_codecs()?;
        let registry = register_default_interceptors(Default::default(), &mut media_engine)?;
        let api = APIBuilder::new()
            .with_media_engine(media_engine)
            .with_interceptor_registry(registry)
            .build();
        Ok(Arc::new(api))
    }

    pub async fn request_file_from_any_peer(
        &self,
        graph_id: &str,
        path: &str,
        sha256: &str,
    ) -> Result<Vec<u8>> {
        let devices = self.api.get_devices().await.unwrap_or_default();
        let self_id = self.config.read().device_id.clone();

        for device in devices {
            if device.device_id == self_id {
                continue;
            }
            match self
                .request_file_from_peer(graph_id, &device.device_id, path, sha256)
                .await
            {
                Ok(data) => return Ok(data),
                Err(err) => warn!(
                    error = %err,
                    peer = %device.device_id,
                    path = %path,
                    "peer file request failed"
                ),
            }
        }

        if let Some(folder) = self.config.read().sync_folder.clone() {
            let full = filesystem::resolve_safe_path(Path::new(&folder), path)?;
            if full.exists() {
                let (hash, _, _) = filesystem::hash_file(&full)?;
                if hash.eq_ignore_ascii_case(sha256) {
                    return Ok(std::fs::read(full)?);
                }
            }
        }

        anyhow::bail!("no peer could provide file: {path}")
    }

    pub async fn request_file_from_peer(
        &self,
        graph_id: &str,
        target_device_id: &str,
        path: &str,
        sha256: &str,
    ) -> Result<Vec<u8>> {
        let api = self.build_api().await?;
        let pc = Arc::new(api.new_peer_connection(self.rtc_config()).await?);
        let (tx, rx) = oneshot::channel();
        let request_key = format!("{target_device_id}:{path}:{sha256}");
        self.pending_requests
            .lock()
            .await
            .insert(request_key.clone(), tx);

        let pending = self.pending_requests.clone();
        let request_key_clone = request_key.clone();
        pc.on_peer_connection_state_change(Box::new(move |state: RTCPeerConnectionState| {
            if state == RTCPeerConnectionState::Failed {
                if let Some(sender) = pending.blocking_lock().remove(&request_key_clone) {
                    let _ = sender.send(Err(anyhow::anyhow!("peer connection failed")));
                }
            }
            Box::pin(async {})
        }));

        let dc = pc
            .create_data_channel(
                "graphsync",
                Some(RTCDataChannelInit {
                    ordered: Some(true),
                    ..Default::default()
                }),
            )
            .await?;

        let buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        Self::wire_incoming_channel(
            dc.clone(),
            self.config.clone(),
            buffer.clone(),
            self.pending_requests.clone(),
            request_key.clone(),
        )
        .await;

        let offer = pc.create_offer(None).await?;
        pc.set_local_description(offer.clone()).await?;
        let session_id = self
            .api
            .post_signaling_offer(graph_id, target_device_id, &offer.sdp)
            .await?;

        self.sessions.lock().await.insert(
            session_id,
            Arc::new(PeerSession {
                pc: pc.clone(),
            }),
        );

        let request = FileRequestMessage {
            path: path.to_string(),
            sha256: sha256.to_string(),
        };
        let open = dc.clone();
        let payload = serde_json::to_string(&request)?;
        tokio::spawn(async move {
            for _ in 0..20 {
                if open.ready_state()
                    == webrtc::data_channel::data_channel_state::RTCDataChannelState::Open
                {
                    let _ = open.send_text(payload.clone()).await;
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            }
        });

        match tokio::time::timeout(std::time::Duration::from_secs(45), rx).await {
            Ok(Ok(result)) => result,
            Ok(Err(_)) => anyhow::bail!("peer response channel closed"),
            Err(_) => anyhow::bail!("timed out waiting for peer file"),
        }
    }

    pub async fn handle_signaling(&self, message: SignalingMessage) -> Result<()> {
        match message {
            SignalingMessage::Offer {
                session_id,
                target_device_id,
                sdp,
                ..
            } => {
                if target_device_id != self.config.read().device_id {
                    return Ok(());
                }
                let api = self.build_api().await?;
                let pc = Arc::new(api.new_peer_connection(self.rtc_config()).await?);

                let config = self.config.clone();
                let pending_requests = self.pending_requests.clone();
                pc.on_data_channel(Box::new(move |dc: Arc<RTCDataChannel>| {
                    let config = config.clone();
                    let pending = pending_requests.clone();
                    Box::pin(async move {
                        let buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
                        Self::wire_incoming_channel(
                            dc,
                            config,
                            buffer,
                            pending,
                            String::new(),
                        )
                        .await;
                    })
                }));

                let offer = RTCSessionDescription::offer(sdp)?;
                pc.set_remote_description(offer).await?;
                let answer = pc.create_answer(None).await?;
                pc.set_local_description(answer.clone()).await?;
                self.api
                    .post_signaling_answer(&session_id, &answer.sdp)
                    .await?;
                self.sessions.lock().await.insert(
                    session_id.clone(),
                    Arc::new(PeerSession {
                        pc,
                    }),
                );
            }
            SignalingMessage::Answer {
                session_id,
                target_device_id,
                sdp,
                ..
            } => {
                if target_device_id != self.config.read().device_id {
                    return Ok(());
                }
                if let Some(session) = self.sessions.lock().await.get(&session_id) {
                    let answer = RTCSessionDescription::answer(sdp)?;
                    session.pc.set_remote_description(answer).await?;
                }
            }
            SignalingMessage::Ice {
                session_id,
                candidate,
                sdp_mid,
                sdp_mline_index,
                ..
            } => {
                if let Some(session) = self.sessions.lock().await.get(&session_id) {
                    session
                        .pc
                        .add_ice_candidate(RTCIceCandidateInit {
                            candidate,
                            sdp_mid,
                            sdp_mline_index,
                            username_fragment: None,
                        })
                        .await?;
                }
            }
        }
        Ok(())
    }

    async fn wire_incoming_channel(
        dc: Arc<RTCDataChannel>,
        config: Arc<RwLock<Config>>,
        buffer: Arc<Mutex<Vec<u8>>>,
        pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<Result<Vec<u8>>>>>>,
        request_key: String,
    ) {
        let config_for_handler = config.clone();
        let dc_for_handler = dc.clone();
        dc.on_message(Box::new(move |msg| {
            let config = config_for_handler.clone();
            let buffer = buffer.clone();
            let pending = pending_requests.clone();
            let request_key = request_key.clone();
            let dc = dc_for_handler.clone();
            Box::pin(async move {
                if msg.is_string {
                    let text = match std::str::from_utf8(&msg.data) {
                        Ok(text) => text,
                        Err(_) => return,
                    };

                    if let Ok(request) = serde_json::from_str::<FileRequestMessage>(text) {
                        let folder_opt = config.read().sync_folder.clone();
                        if let Some(folder) = folder_opt {
                            if let Ok(full) =
                                filesystem::resolve_safe_path(Path::new(&folder), &request.path)
                            {
                                if full.exists() {
                                    if let Ok(mut file) = std::fs::File::open(full) {
                                        let mut chunk = vec![0u8; CHUNK_SIZE];
                                        loop {
                                            match file.read(&mut chunk) {
                                                Ok(0) => break,
                                                Ok(n) => {
                                                    let _ = dc.send(&Bytes::copy_from_slice(&chunk[..n])).await;
                                                }
                                                Err(_) => break,
                                            }
                                        }
                                        let _ = dc.send_text("EOF".to_string()).await;
                                    }
                                }
                            }
                        }
                        return;
                    }

                    if text == "EOF" {
                        let data = buffer.lock().await.clone();
                        if !request_key.is_empty() {
                            if let Some(sender) = pending.lock().await.remove(&request_key) {
                                let _ = sender.send(Ok(data));
                            }
                        }
                    }
                } else {
                    buffer.lock().await.extend_from_slice(&msg.data);
                }
            })
        }));
    }
}
