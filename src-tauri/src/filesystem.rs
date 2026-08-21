use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use tracing::warn;

const CHUNK_SIZE: usize = 64 * 1024;

pub fn normalize_relative_path(path: &str) -> Result<String> {
    let replaced = path.replace('\\', "/");
    if replaced.contains("..") {
        bail!("path traversal rejected: {path}");
    }
    if replaced.starts_with('/') {
        bail!("absolute paths are not allowed: {path}");
    }

    let mut normalized = PathBuf::new();
    for component in Path::new(&replaced).components() {
        match component {
            Component::Normal(part) => normalized.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                bail!("invalid path component in: {path}");
            }
        }
    }

    Ok(normalized.to_string_lossy().replace('\\', "/"))
}

pub fn relative_path_from_root(root: &Path, absolute: &Path) -> Result<String> {
    let rel = absolute
        .strip_prefix(root)
        .with_context(|| format!("path outside sync root: {}", absolute.display()))?;
    normalize_relative_path(&rel.to_string_lossy())
}

pub fn resolve_safe_path(root: &Path, relative: &str) -> Result<PathBuf> {
    let normalized = normalize_relative_path(relative)?;
    let candidate = root.join(&normalized);
    let canonical_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let canonical_candidate = candidate
        .parent()
        .map(|p| p.canonicalize().unwrap_or_else(|_| p.to_path_buf()))
        .unwrap_or_else(|| canonical_root.clone());

    if !canonical_candidate.starts_with(&canonical_root) {
        bail!("resolved path escapes sync directory: {relative}");
    }
    Ok(candidate)
}

pub fn is_temp_or_ignored(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return true;
    };

    if name.starts_with('.') && name != ".gitkeep" {
        if name == ".DS_Store" || name.starts_with(".#") {
            return true;
        }
    }

    let lower = name.to_lowercase();
    lower.ends_with(".tmp")
        || lower.ends_with('~')
        || lower.ends_with(".swp")
        || lower.ends_with(".swx")
        || lower.ends_with(".lock")
        || lower.ends_with(".crdownload")
        || lower.ends_with(".part")
        || lower.ends_with(".partial")
        || lower.ends_with(".download")
        || lower.ends_with(".bak")
        || lower.ends_with(".temp")
        || name.ends_with('#')
}

pub fn hash_file(path: &Path) -> Result<(String, u64, i64)> {
    let metadata = std::fs::metadata(path)?;
    let modified = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; CHUNK_SIZE];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok((hex::encode(hasher.finalize()), metadata.len(), modified))
}

pub fn write_file_atomic(root: &Path, relative: &str, reader: &mut dyn Read) -> Result<(String, u64, i64)> {
    let dest = resolve_safe_path(root, relative)?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let tmp = dest.with_extension("graphsync-tmp");
    let mut file = std::fs::File::create(&tmp)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; CHUNK_SIZE];
    let mut total = 0u64;

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        file.write_all(&buffer[..read])?;
        total += read as u64;
    }
    file.sync_all()?;
    drop(file);

    if dest.exists() {
        std::fs::remove_file(&dest)?;
    }
    std::fs::rename(&tmp, &dest)?;

    let modified = std::fs::metadata(&dest)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    Ok((hex::encode(hasher.finalize()), total, modified))
}

pub fn delete_file(root: &Path, relative: &str) -> Result<()> {
    let dest = resolve_safe_path(root, relative)?;
    if dest.exists() {
        if dest.is_dir() {
            warn!(path = %relative, "attempted to delete directory via file delete");
            bail!("cannot delete directory through file delete: {relative}");
        }
        std::fs::remove_file(dest)?;
    }
    Ok(())
}

pub fn rename_file(root: &Path, old_path: &str, new_path: &str) -> Result<()> {
    let from = resolve_safe_path(root, old_path)?;
    let to = resolve_safe_path(root, new_path)?;
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if from.exists() {
        if to.exists() {
            std::fs::remove_file(&to)?;
        }
        std::fs::rename(from, to)?;
    }
    Ok(())
}

pub fn walk_sync_files(root: &Path) -> Result<Vec<(String, String, u64, i64)>> {
    let mut results = Vec::new();
    walk_dir(root, root, &mut results)?;
    Ok(results)
}

fn walk_dir(root: &Path, current: &Path, out: &mut Vec<(String, String, u64, i64)>) -> Result<()> {
    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_dir(root, &path, out)?;
            continue;
        }
        if is_temp_or_ignored(&path) {
            continue;
        }
        let relative = relative_path_from_root(root, &path)?;
        let (hash, size, mtime) = hash_file(&path)?;
        out.push((relative, hash, size, mtime));
    }
    Ok(())
}
