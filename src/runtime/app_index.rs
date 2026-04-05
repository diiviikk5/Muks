use crate::system::AppEntry;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const CACHE_PATH: &str = "config/app_index_cache.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppIndexCache {
    pub generated_at_epoch_ms: u64,
    pub apps: Vec<AppEntry>,
}

pub fn load_cache() -> Result<Option<AppIndexCache>> {
    let path = Path::new(CACHE_PATH);
    if !path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read app index cache at {}", path.display()))?;
    let parsed: AppIndexCache = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse app index cache at {}", path.display()))?;

    Ok(Some(parsed))
}

pub fn save_cache(apps: &[AppEntry]) -> Result<()> {
    let path = Path::new(CACHE_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create cache directory {}", parent.display()))?;
    }

    let payload = AppIndexCache {
        generated_at_epoch_ms: current_epoch_ms(),
        apps: apps.to_vec(),
    };

    let encoded = serde_json::to_string_pretty(&payload).context("failed to encode app cache")?;
    fs::write(path, encoded)
        .with_context(|| format!("failed to write app index cache at {}", path.display()))?;

    Ok(())
}

fn current_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|v| v.as_millis() as u64)
        .unwrap_or_default()
}
