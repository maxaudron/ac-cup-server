use crate::models::ListResponse;
use crate::storage::SharedStorage;
use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    Json,
};

pub async fn list_all(State(storage): State<SharedStorage>) -> Json<ListResponse> {
    let storage = storage.read().unwrap();
    Json(ListResponse::from(&*storage))
}

pub async fn get_item(
    State(storage): State<SharedStorage>,
    Path((content_type, name)): Path<(String, String)>,
) -> Response {
    let storage = storage.read().unwrap();
    
    let item = match content_type.as_str() {
        "car" => storage.car.get(&name),
        "track" => storage.track.get(&name),
        "luaapp" => storage.luaapp.get(&name),
        "app" => storage.app.get(&name),
        "filter" => storage.filter.get(&name),
        _ => return (StatusCode::NOT_FOUND, "Invalid content type").into_response(),
    };

    match item {
        Some(item) => Json(item.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, "Item not found").into_response(),
    }
}

pub async fn get_download(
    State(storage): State<SharedStorage>,
    Path((content_type, name)): Path<(String, String)>,
) -> Response {
    let storage = storage.read().unwrap();
    
    let item = match content_type.as_str() {
        "car" => storage.car.get(&name),
        "track" => storage.track.get(&name),
        "luaapp" => storage.luaapp.get(&name),
        "app" => storage.app.get(&name),
        "filter" => storage.filter.get(&name),
        _ => return (StatusCode::NOT_FOUND, "Invalid content type").into_response(),
    };

    match item {
        Some(item) => {
            (
                StatusCode::FOUND,
                [(header::LOCATION, item.download_url.as_str())],
            ).into_response()
        },
        None => (StatusCode::NOT_FOUND, "Item not found").into_response(),
    }
}




