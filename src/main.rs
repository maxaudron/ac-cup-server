mod handlers;
mod models;
mod storage;

use axum::{
    routing::get,
    Router,
};
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ac_cup_server=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let storage_path = env::var("STORAGE_PATH")
        .unwrap_or_else(|_| "storage.json".to_string());
    let storage_path = PathBuf::from(storage_path);

    let storage = storage::load_storage(&storage_path)
        .unwrap_or_else(|e| {
            tracing::warn!("Failed to load storage: {}. Using empty storage.", e);
            models::Storage::default()
        });

    let shared_storage = Arc::new(RwLock::new(storage));

    // Start watching the storage file for changes
    let watch_storage = shared_storage.clone();
    let watch_path = storage_path.clone();
    if let Err(e) = storage::watch_storage(watch_path, watch_storage) {
        tracing::error!("Failed to start storage file watcher: {}", e);
    }

    let app = Router::new()
        .route("/", get(handlers::list_all))
        .route("/{content_type}/{name}", get(handlers::get_item))
        .route("/{content_type}/{name}/get", get(handlers::get_download))
        .layer(TraceLayer::new_for_http())
        .with_state(shared_storage);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}


