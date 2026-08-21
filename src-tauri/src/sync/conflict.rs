use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::Utc;

use crate::filesystem;
use crate::sync::events::FileOperation;

pub fn conflict_copy_path(original: &Path, device_name: &str) -> PathBuf {
    let stem = original
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = original
        .extension()
        .and_then(|s| s.to_str())
        .map(|e| format!(".{e}"))
        .unwrap_or_default();
    let date = Utc::now().format("%Y-%m-%d");
    let parent = original.parent().unwrap_or_else(|| Path::new("."));
    parent.join(format!("{stem} (conflict - {device_name} - {date}){ext}"))
}

pub fn apply_remote_with_conflict_check(
    root: &Path,
    relative: &str,
    remote_hash: &str,
    remote_revision: u64,
    local_entry: Option<&crate::sync::state::FileStateEntry>,
    device_name: &str,
    incoming: &[u8],
) -> Result<(String, u64, i64, bool)> {
    let dest = filesystem::resolve_safe_path(root, relative)?;
    let had_conflict = if dest.exists() {
        if let Some(local) = local_entry {
            if !local.deleted && local.sha256 != remote_hash && local.revision >= remote_revision {
                let conflict_path = conflict_copy_path(&dest, device_name);
                if dest.exists() {
                    std::fs::copy(&dest, &conflict_path)?;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    let mut cursor = std::io::Cursor::new(incoming);
    let (hash, size, mtime) = filesystem::write_file_atomic(root, relative, &mut cursor)?;
    Ok((hash, size, mtime, had_conflict))
}

pub fn apply_remote_delete_with_conflict_check(
    root: &Path,
    relative: &str,
    local_entry: Option<&crate::sync::state::FileStateEntry>,
    local_unsynced: bool,
    device_name: &str,
) -> Result<bool> {
    let dest = filesystem::resolve_safe_path(root, relative)?;
    if !dest.exists() {
        return Ok(false);
    }

    if local_unsynced {
        if let Some(local) = local_entry {
            if !local.deleted {
                let conflict_path = conflict_copy_path(&dest, device_name);
                std::fs::copy(&dest, &conflict_path)?;
            }
        }
        return Ok(true);
    }

    filesystem::delete_file(root, relative)?;
    Ok(false)
}

pub fn operation_label(op: FileOperation) -> &'static str {
    match op {
        FileOperation::Create => "CREATE",
        FileOperation::Update => "UPDATE",
        FileOperation::Delete => "DELETE",
        FileOperation::Rename => "RENAME",
    }
}
