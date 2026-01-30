use crate::models::Storage;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub type SharedStorage = Arc<RwLock<Storage>>;

pub fn load_storage(path: &Path) -> Result<Storage, Box<dyn std::error::Error>> {
    if path.exists() {
        let content = fs::read_to_string(path)?;
        let storage = serde_json::from_str(&content)?;
        Ok(storage)
    } else {
        Ok(Storage::default())
    }
}

pub fn watch_storage(
    storage_path: PathBuf,
    shared_storage: SharedStorage,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<Event, notify::Error>>();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&storage_path, RecursiveMode::NonRecursive)?;

    tracing::info!("Watching storage file: {:?}", storage_path);

    std::thread::spawn(move || {
        let _watcher = watcher; // Keep watcher alive
        
        for res in rx {
            match res {
                Ok(event) => {
                    if event.kind.is_modify() || event.kind.is_create() {
                        tracing::info!("Storage file changed, reloading...");
                        
                        match load_storage(&storage_path) {
                            Ok(new_storage) => {
                                if let Ok(mut storage) = shared_storage.write() {
                                    *storage = new_storage;
                                    tracing::info!("Storage reloaded successfully");
                                } else {
                                    tracing::error!("Failed to acquire write lock on storage");
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to reload storage: {}", e);
                            }
                        }
                    }
                }
                Err(e) => tracing::error!("Watch error: {:?}", e),
            }
        }
    });

    Ok(())
}
