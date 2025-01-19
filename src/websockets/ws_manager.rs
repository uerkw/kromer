use std::collections::HashMap;

use surrealdb::Uuid;

use super::wrapped_ws::WrappedWsData;

#[derive(Default, Debug)]
pub struct WsDataManager {
    pub sockets: HashMap<Uuid, WrappedWsData>,
}

impl WsDataManager {
    pub fn add(
        &mut self,
        token: Uuid,
        address: String,
        privatekey: Option<String>,
    ) -> WrappedWsData {
        let wrapped_ws = WrappedWsData::new(token, address, privatekey);
        tracing::debug!("Adding UUID: {:?} to WsDataManager", token);
        self.sockets.insert(token, wrapped_ws.clone());

        wrapped_ws
    }

    pub fn get(&mut self, uuid: Uuid) -> Option<WrappedWsData> {
        if self.sockets.contains_key(&uuid.clone()) {
            tracing::debug!("WsData Exists");
        } else {
            tracing::debug!("WsData does not exist");
        }
        let result = self.sockets.get(&uuid);
        tracing::debug!("Getting WrappedWsData from WsDataManager: {:?}", result);
        result.cloned()
    }

    pub fn remove(&mut self, uuid: Uuid) {
        self.sockets.remove(&uuid);
    }
}
