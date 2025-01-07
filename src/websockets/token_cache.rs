use super::types::common::WebSocketTokenData;
use std::collections::HashMap;
use surrealdb::Uuid;

#[derive(Default, Debug, Clone)]
pub struct TokenCache {
    token_cache: HashMap<Uuid, WebSocketTokenData>,
}

impl TokenCache {
    pub fn new() -> Self {
        Self {
            token_cache: HashMap::new(),
        }
    }
    pub fn add_token(&mut self, uuid: Uuid, token_params: WebSocketTokenData) {
        // Insert the token into the cache
        self.token_cache.insert(uuid, token_params);
        tracing::info!("Added token {uuid} to cache");
    }

    pub fn check_token(&mut self, uuid: Uuid) -> bool {
        tracing::info!("Checking token {uuid}");
        if self.token_cache.contains_key(&uuid) {
            tracing::info!("Token exists in cache");
            true
        } else {
            tracing::info!("Token did not exist in cache");
            false
        }
    }

    pub fn remove_token(&mut self, uuid: Uuid) -> Option<WebSocketTokenData> {
        let token_data = self.token_cache.remove(&uuid);
        tracing::info!("Removed token {uuid} from cache");
        token_data
    }
}
