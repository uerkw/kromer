use std::collections::HashMap;

use actix::prelude::*;
use actix::{Actor, Context, Handler, MessageResult};
use actix_broker::BrokerSubscribe;
use surrealdb::Uuid;

use crate::ws::types::actor_message::ReceiveMessage;

use super::{
    session::WebSocketSession,
    types::actor_message::{GetActiveSessions, GetCacheConnection, SetCacheConnection},
};

#[derive(Default)]
pub struct WebSocketServer {
    sessions: HashMap<Uuid, Addr<WebSocketSession>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        WebSocketServer {
            sessions: HashMap::new(),
        }
    }

    pub fn add_client_to_sessions(&mut self, uuid: Uuid, conn_to_cache: Addr<WebSocketSession>) {
        self.sessions.insert(uuid, conn_to_cache);
    }

    fn send_payload_message(&mut self, id: Uuid, msg: &str) {
        tracing::debug!("Received, ID: {id}, msg: {msg}");
        
    }
}

impl Actor for WebSocketServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("Started WS Server Actor");
        self.subscribe_system_async::<ReceiveMessage>(ctx);
    }
}

impl Handler<ReceiveMessage> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: ReceiveMessage, _ctx: &mut Self::Context) {
        let ReceiveMessage(id, msg) = msg;
        self.send_payload_message(id, &msg);
    }
}

impl Handler<SetCacheConnection> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: SetCacheConnection, _ctx: &mut Self::Context) {
        let SetCacheConnection(uuid, conn_to_cache) = msg;

        self.add_client_to_sessions(uuid, conn_to_cache);
    }
}

impl Handler<GetCacheConnection> for WebSocketServer {
    type Result = MessageResult<GetCacheConnection>;

    fn handle(&mut self, msg: GetCacheConnection, _ctx: &mut Self::Context) -> Self::Result {
        let GetCacheConnection(uuid) = msg;

        let result = self.sessions.get(&uuid).cloned();

        MessageResult(result)
    }
}

impl Handler<GetActiveSessions> for WebSocketServer {
    type Result = MessageResult<GetActiveSessions>;

    fn handle(&mut self, _: GetActiveSessions, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.sessions.keys().cloned().collect())
    }
}

impl SystemService for WebSocketServer {}
impl Supervised for WebSocketServer {}
