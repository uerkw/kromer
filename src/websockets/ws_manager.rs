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

        self.sockets.insert(token, wrapped_ws.clone());

        wrapped_ws
    }

    pub fn get(&mut self, uuid: Uuid) -> Option<WrappedWsData> {
        self.sockets.get(&uuid).cloned()
    }

    pub fn remove(&mut self, uuid: Uuid) {
        self.sockets.remove(&uuid);
    }
}
