use surrealdb::Uuid;

use super::types::common::WebSocketSubscriptionList;

#[derive(Default, Debug, Clone)]
pub struct WrappedWsData {
    pub token: Uuid,
    pub address: String,
    pub privatekey: Option<String>,
    pub subs: WebSocketSubscriptionList,
}

impl WrappedWsData {
    pub fn new(token: Uuid, address: String, privatekey: Option<String>) -> Self {
        let subs = WebSocketSubscriptionList::new();
        WrappedWsData {
            token,
            address,
            privatekey,
            subs,
        }
    }

    pub fn is_guest(&self) -> bool {
        self.address == *"guest"
    }
}
