use actix::prelude::*;
use actix_ws::CloseReason;
use surrealdb::Uuid;

use crate::ws::actors::session::WebSocketSession;

use super::server::TokenParams;

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

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveCacheConnection(pub Uuid);

#[derive(Clone, Message)]
#[rtype(result = "Vec<Uuid>")]
pub struct GetActiveSessions;

#[derive(Message)]
#[rtype(result = "bool")]
pub struct CheckTokenExists(pub Uuid);

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddToken(pub Uuid, pub TokenParams);

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveToken(pub Uuid);

// WebSocketSessionActor
#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseWebSocket(pub CloseReason);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct KromerMessage(pub String);