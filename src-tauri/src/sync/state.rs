use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::config::state_path;
use crate::sync::events::FileOperation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStateEntry {
    pub relative_path: String,
    pub sha256: String,
    pub size: u64,
    pub mtime: i64,
    pub revision: u64,
    pub last_known_operation: FileOperation,
    #[serde(default)]
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalState {
    pub graph_id: Option<String>,
    pub last_event_cursor: Option<String>,
    pub files: HashMap<String, FileStateEntry>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl LocalState {
    pub fn load() -> Result<Self> {
        let path = state_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = state_path()?;
        let mut value = self.clone();
        value.updated_at = Some(Utc::now());
        let data = serde_json::to_string_pretty(&value)?;
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, data)?;
        std::fs::rename(tmp, path)?;
        Ok(())
    }

    pub fn next_revision(&mut self, path: &str) -> u64 {
        self.files
            .get(path)
            .map(|entry| entry.revision + 1)
            .unwrap_or(1)
    }

    pub fn upsert_file(
        &mut self,
        path: &str,
        sha256: &str,
        size: u64,
        mtime: i64,
        revision: u64,
        operation: FileOperation,
    ) {
        self.files.insert(
            path.to_string(),
            FileStateEntry {
                relative_path: path.to_string(),
                sha256: sha256.to_string(),
                size,
                mtime,
                revision,
                last_known_operation: operation,
                deleted: false,
            },
        );
    }

    pub fn mark_deleted(&mut self, path: &str, revision: u64) {
        self.files.insert(
            path.to_string(),
            FileStateEntry {
                relative_path: path.to_string(),
                sha256: String::new(),
                size: 0,
                mtime: 0,
                revision,
                last_known_operation: FileOperation::Delete,
                deleted: true,
            },
        );
    }

    pub fn rename_path(&mut self, old_path: &str, new_path: &str, revision: u64) {
        if let Some(mut entry) = self.files.remove(old_path) {
            entry.relative_path = new_path.to_string();
            entry.revision = revision;
            entry.last_known_operation = FileOperation::Rename;
            self.files.insert(new_path.to_string(), entry);
        }
    }

    pub fn build_manifest(&self) -> Vec<ManifestEntry> {
        self.files
            .values()
            .filter(|entry| !entry.deleted)
            .map(|entry| ManifestEntry {
                path: entry.relative_path.clone(),
                sha256: entry.sha256.clone(),
                size: entry.size,
                mtime: entry.mtime,
                revision: entry.revision,
            })
            .collect()
    }

    pub fn scan_folder(&mut self, root: &Path, graph_id: &str) -> Result<()> {
        info!("building local manifest from folder");
        self.graph_id = Some(graph_id.to_string());
        self.files.clear();

        for (relative, hash, size, mtime) in crate::filesystem::walk_sync_files(root)? {
            self.upsert_file(
                &relative,
                &hash,
                size,
                mtime,
                1,
                FileOperation::Create,
            );
        }
        self.save()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub path: String,
    pub sha256: String,
    pub size: u64,
    pub mtime: i64,
    pub revision: u64,
}
