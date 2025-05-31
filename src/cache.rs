use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    stats: HashMap<String, usize>,
}

pub fn get_cache_path(project_dir: &Path, language: &str) -> PathBuf {
    let mut hasher = Sha256::new();
    hasher.update(project_dir.to_string_lossy().as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from(std::env::var("HOME").unwrap()).join(".cache"))
        .join("repostats");
    cache_dir.join(format!("{}_{}.json", hash, language.to_lowercase()))
}

pub fn load_cache(project_dir: &Path, language: &str) -> Option<HashMap<String, usize>> {
    let cache_path = get_cache_path(project_dir, language);
    if cache_path.exists() {
        if let Ok(mut file) = File::open(&cache_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(cache) = serde_json::from_str::<Cache>(&contents) {
                    return Some(cache.stats);
                }
            }
        }
    }
    None
}

pub fn save_cache(project_dir: &Path, language: &str, stats: &HashMap<String, usize>) {
    let cache_path = get_cache_path(project_dir, language);
    if let Some(parent) = cache_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(file) = File::create(&cache_path) {
        let cache = Cache { stats: stats.clone() };
        let _ = serde_json::to_writer(file, &cache);
    }
}
