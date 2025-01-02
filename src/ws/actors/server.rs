use std::collections::HashMap;

use actix::prelude::*;
use actix::{Actor, Context, Handler, MessageResult};
use actix_broker::BrokerSubscribe;
use surrealdb::Uuid;

use crate::ws::{
    actors::session::WebSocketSession,
    types::actor_message::{
        GetActiveSessions, GetCacheConnection, KromerMessage, ReceiveMessage,
        RemoveCacheConnection, SetCacheConnection,
    },
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

    fn receive_payload_message(&mut self, id: Uuid, msg: &str) {
        tracing::debug!("[WS_SERVER_ACTOR] Received, ID: {id}, msg: {msg}");
        let opt_addr = self.sessions.get(&id);
        let opt_addr2 = opt_addr.cloned();
        if let Some(ws_actor) = opt_addr2 {
            let return_msg = format!("Found your UUID: {id}. Your msg was '{msg}'");
            tracing::debug!("[WS_SERVER_ACTOR] Processing message for UUID: {id}");
            let success_msg = KromerMessage(return_msg);
            let future = async move {
                let _ = ws_actor.send(success_msg).await;
            };

            actix::spawn(future);
        } else {
            tracing::debug!("[WS_SERVER_ACTOR] Could not find session for {id}");
        }
    }
}

impl Actor for WebSocketServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("[WS_SERVER_ACTOR] Started WS Server Actor");
        self.subscribe_system_async::<ReceiveMessage>(ctx);
    }
}

impl Handler<ReceiveMessage> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: ReceiveMessage, _ctx: &mut Self::Context) {
        let ReceiveMessage(id, msg) = msg;
        self.receive_payload_message(id, &msg);
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

impl Handler<RemoveCacheConnection> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveCacheConnection, _ctx: &mut Self::Context) -> Self::Result {
        let RemoveCacheConnection(uuid) = msg;
        tracing::debug!("[WS_SERVER_ACTOR] Message recevied to remove UUID from cache: {uuid}");

        self.sessions.remove(&uuid);
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
