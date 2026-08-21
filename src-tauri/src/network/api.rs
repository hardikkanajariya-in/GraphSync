use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use parking_lot::RwLock;
use reqwest::{Client, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::warn;
use url::Url;

use crate::config::Config;
use crate::device::DeviceInfo;
use crate::sync::events::SyncEvent;
use crate::sync::state::ManifestEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub device_id: String,
    pub protocol_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphRegistration {
    pub graph_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventsResponse {
    pub events: Vec<SyncEvent>,
    #[serde(default)]
    pub cursor: Option<String>,
}

pub struct ApiClient {
    config: Arc<RwLock<Config>>,
    http: Client,
}

impl ApiClient {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client");
        Self { config, http }
    }

    fn base_url(&self) -> Result<String> {
        let url = self.config.read().api_url.trim().trim_end_matches('/').to_string();
        if url.is_empty() {
            anyhow::bail!("API URL is not configured");
        }
        Ok(url)
    }

    fn auth_header(&self) -> Result<String> {
        let token = self
            .config
            .read()
            .auth_token
            .clone()
            .context("missing auth token")?;
        Ok(format!("Bearer {token}"))
    }

    async fn json<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            warn!(%status, body = %body, "API request failed");
            anyhow::bail!("API error {status}: {body}");
        }
        Ok(response.json().await?)
    }

    pub async fn authenticate_device(&self, device_id: &str) -> Result<AuthResponse> {
        let url = format!("{}/auth/device", self.base_url()?);
        let payload = serde_json::json!({
            "device_id": device_id,
            "protocol_version": crate::config::PROTOCOL_VERSION,
        });
        let response = self.http.post(url).json(&payload).send().await?;
        Self::json(response).await
    }

    pub async fn register_device(
        &self,
        device_id: &str,
        device_name: &str,
        platform: &str,
    ) -> Result<DeviceInfo> {
        let url = format!("{}/devices/register", self.base_url()?);
        let payload = serde_json::json!({
            "device_id": device_id,
            "device_name": device_name,
            "platform": platform,
            "protocol_version": crate::config::PROTOCOL_VERSION,
        });
        let response = self
            .http
            .post(&url)
            .header(
                "Authorization",
                self.auth_header().unwrap_or_else(|_| String::new()),
            )
            .json(&payload)
            .send()
            .await?;
        if response.status() == StatusCode::UNAUTHORIZED {
            let auth = self.authenticate_device(device_id).await?;
            self.config.write().auth_token = Some(auth.token);
            return Box::pin(self.register_device(device_id, device_name, platform)).await;
        }
        Self::json(response).await
    }

    pub async fn register_graph(&self, device_id: &str, folder: &str) -> Result<GraphRegistration> {
        let url = format!("{}/graphs/register", self.base_url()?);
        let name = std::path::Path::new(folder)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("GraphSync Folder");
        let payload = serde_json::json!({
            "device_id": device_id,
            "name": name,
            "protocol_version": crate::config::PROTOCOL_VERSION,
        });
        let response = self
            .http
            .post(url)
            .header("Authorization", self.auth_header()?)
            .json(&payload)
            .send()
            .await?;
        Self::json(response).await
    }

    pub async fn post_sync_event(&self, event: &SyncEvent) -> Result<()> {
        let url = format!("{}/sync/events", self.base_url()?);
        let response = self
            .http
            .post(url)
            .header("Authorization", self.auth_header()?)
            .json(event)
            .send()
            .await?;
        Self::json::<serde_json::Value>(response).await?;
        Ok(())
    }

    pub async fn get_sync_events(
        &self,
        graph_id: &str,
        cursor: Option<&str>,
    ) -> Result<Vec<SyncEvent>> {
        let mut url = Url::parse(&format!("{}/sync/events", self.base_url()?))?;
        url.query_pairs_mut()
            .append_pair("graph_id", graph_id)
            .append_pair("protocol_version", &crate::config::PROTOCOL_VERSION.to_string());
        if let Some(cursor) = cursor {
            url.query_pairs_mut().append_pair("cursor", cursor);
        }

        let response = self
            .http
            .get(url)
            .header("Authorization", self.auth_header()?)
            .send()
            .await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }
        let body: EventsResponse = Self::json(response).await?;
        Ok(body.events)
    }

    pub async fn ack_event(&self, event_id: &str) -> Result<()> {
        let url = format!("{}/sync/ack", self.base_url()?);
        let payload = serde_json::json!({ "event_id": event_id });
        let response = self
            .http
            .post(url)
            .header("Authorization", self.auth_header()?)
            .json(&payload)
            .send()
            .await?;
        Self::json::<serde_json::Value>(response).await?;
        Ok(())
    }

    pub async fn get_manifest(&self, graph_id: &str) -> Result<Vec<ManifestEntry>> {
        let url = format!("{}/sync/manifest/{}", self.base_url()?, graph_id);
        let response = self
            .http
            .get(url)
            .header("Authorization", self.auth_header()?)
            .send()
            .await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }
        let body: serde_json::Value = Self::json(response).await?;
        Ok(serde_json::from_value(body.get("files").cloned().unwrap_or_default())?)
    }

    pub async fn post_signaling_offer(
        &self,
        graph_id: &str,
        target_device_id: &str,
        sdp: &str,
    ) -> Result<String> {
        let url = format!("{}/signaling/offer", self.base_url()?);
        let payload = serde_json::json!({
            "graph_id": graph_id,
            "source_device_id": self.config.read().device_id,
            "target_device_id": target_device_id,
            "sdp": sdp,
            "protocol_version": crate::config::PROTOCOL_VERSION,
        });
        let response = self
            .http
            .post(url)
            .header("Authorization", self.auth_header()?)
            .json(&payload)
            .send()
            .await?;
        let body: serde_json::Value = Self::json(response).await?;
        Ok(body
            .get("session_id")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string())
    }

    pub async fn post_signaling_answer(&self, session_id: &str, sdp: &str) -> Result<()> {
        let url = format!("{}/signaling/answer", self.base_url()?);
        let payload = serde_json::json!({
            "session_id": session_id,
            "sdp": sdp,
            "protocol_version": crate::config::PROTOCOL_VERSION,
        });
        let response = self
            .http
            .post(url)
            .header("Authorization", self.auth_header()?)
            .json(&payload)
            .send()
            .await?;
        Self::json::<serde_json::Value>(response).await?;
        Ok(())
    }

    pub async fn get_devices(&self) -> Result<Vec<DeviceInfo>> {
        let url = format!("{}/devices", self.base_url()?);
        let response = self
            .http
            .get(url)
            .header("Authorization", self.auth_header()?)
            .send()
            .await?;
        let body: serde_json::Value = Self::json(response).await?;
        Ok(serde_json::from_value(body.get("devices").cloned().unwrap_or_default())?)
    }

    pub fn ws_url(&self) -> Result<String> {
        let base = self.base_url()?;
        let parsed = Url::parse(&base)?;
        let scheme = match parsed.scheme() {
            "https" => "wss",
            "http" => "ws",
            other => anyhow::bail!("unsupported websocket scheme for {other}"),
        };
        let host = parsed.host_str().context("missing host")?;
        let authority = match parsed.port() {
            Some(port) => format!("{host}:{port}"),
            None => host.to_string(),
        };
        let device_id = self.config.read().device_id.clone();
        Ok(format!(
            "{scheme}://{authority}/ws?device_id={device_id}&protocol_version={}",
            crate::config::PROTOCOL_VERSION
        ))
    }
}
