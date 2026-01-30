use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentItem {
    pub name: String,
    pub author: String,
    pub information_url: String,
    pub version: String,
    pub active: bool,
    pub clean_installation: bool,
    #[serde(skip_serializing)]
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Storage {
    #[serde(default)]
    pub car: HashMap<String, ContentItem>,
    #[serde(default)]
    pub track: HashMap<String, ContentItem>,
    #[serde(default)]
    pub luaapp: HashMap<String, ContentItem>,
    #[serde(default)]
    pub app: HashMap<String, ContentItem>,
    #[serde(default)]
    pub filter: HashMap<String, ContentItem>,
}

#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub car: HashMap<String, String>,
    pub track: HashMap<String, String>,
    pub luaapp: HashMap<String, String>,
    pub app: HashMap<String, String>,
    pub filter: HashMap<String, String>,
}

impl From<&Storage> for ListResponse {
    fn from(storage: &Storage) -> Self {
        Self {
            car: storage.car.iter().map(|(k, v)| (k.clone(), v.version.clone())).collect(),
            track: storage.track.iter().map(|(k, v)| (k.clone(), v.version.clone())).collect(),
            luaapp: storage.luaapp.iter().map(|(k, v)| (k.clone(), v.version.clone())).collect(),
            app: storage.app.iter().map(|(k, v)| (k.clone(), v.version.clone())).collect(),
            filter: storage.filter.iter().map(|(k, v)| (k.clone(), v.version.clone())).collect(),
        }
    }
}
