use std::sync::Arc;

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{info, warn};

use crate::network::api::ApiClient;
use crate::network::signaling::SignalingMessage;
use crate::sync::engine::SyncEngine;
use crate::sync::events::SyncEvent;

pub struct WebSocketClient {
    api: Arc<ApiClient>,
    connected: Arc<tokio::sync::Mutex<bool>>,
}

impl WebSocketClient {
    pub fn new(api: Arc<ApiClient>) -> Self {
        Self {
            api,
            connected: Arc::new(tokio::sync::Mutex::new(false)),
        }
    }

    pub async fn connect(self: &Arc<Self>, engine: Arc<SyncEngine>) -> Result<()> {
        let ws_url = self.api.ws_url()?;
        let mut backoff = std::time::Duration::from_secs(2);
        let client = self.clone();

        tokio::spawn(async move {
            loop {
                if *engine.shutdown.borrow() {
                    break;
                }
                match connect_async(&ws_url).await {
                    Ok((ws, _)) => {
                        info!("connected to websocket");
                        *client.connected.lock().await = true;
                        backoff = std::time::Duration::from_secs(2);
                        let (mut write, mut read) = ws.split();

                        while let Some(msg) = read.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if let Ok(event) = serde_json::from_str::<SyncEvent>(&text) {
                                        if let Err(err) = engine.apply_remote_event(event).await {
                                            warn!(error = %err, "failed to apply websocket event");
                                        }
                                        continue;
                                    }
                                    if let Ok(signal) =
                                        serde_json::from_str::<SignalingMessage>(&text)
                                    {
                                        if let Err(err) =
                                            engine.peers.handle_signaling(signal).await
                                        {
                                            warn!(error = %err, "signaling message failed");
                                        }
                                    }
                                }
                                Ok(Message::Ping(data)) => {
                                    let _ = write.send(Message::Pong(data)).await;
                                }
                                Ok(Message::Close(_)) => break,
                                Err(err) => {
                                    warn!(error = %err, "websocket read error");
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(err) => {
                        warn!(error = %err, "websocket connection failed");
                    }
                }

                *client.connected.lock().await = false;
                if *engine.shutdown.borrow() {
                    break;
                }
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(std::time::Duration::from_secs(60));
            }
        });

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn disconnect(&self) {
        *self.connected.lock().await = false;
    }
}
