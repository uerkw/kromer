use actix::prelude::*;
use surrealdb::Uuid;

use crate::ws::session::WebSocketSession;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct KromerMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ReceiveMessage(pub Uuid, pub String);

impl ToString for ReceiveMessage {
    fn to_string(&self) -> String {
        self.1.to_string()
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetCacheConnection(pub Uuid, pub Addr<WebSocketSession>);

#[derive(Message)]
#[rtype(result = "Option<Addr<WebSocketSession>>")]
pub struct GetCacheConnection(pub Uuid);

#[derive(Clone, Message)]
#[rtype(result = "Vec<Uuid>")]
pub struct GetActiveSessions;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseWebSocket;