use std::collections::HashMap;

use actix::prelude::*;
use actix::{Actor, Context, Handler, MessageResult};
use actix_broker::BrokerSubscribe;
use surrealdb::Uuid;

use crate::ws::handler::handle_ws;
use crate::ws::types::actor_message::{AddToken, CheckTokenExists, RemoveToken};
use crate::ws::{
    actors::session::WebSocketSession,
    types::actor_message::{
        GetActiveSessions, GetCacheConnection, KromerMessage, ReceiveMessage,
        RemoveCacheConnection, SetCacheConnection,
    },
    types::server::TokenParams
};

#[derive(Default)]
pub struct WebSocketServer {
    tokens: HashMap<Uuid, TokenParams>,
    sessions: HashMap<Uuid, Addr<WebSocketSession>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        WebSocketServer {
            tokens: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    pub fn add_new_token(&mut self, uuid: Uuid, params: TokenParams) {
        self.tokens.insert(uuid, params);
    }

    pub fn remove_token(&mut self, uuid: Uuid) {
        self.tokens.remove(&uuid);
    }

    pub fn check_token_exists(&mut self, uuid: Uuid) -> bool {
        self.tokens.contains_key(&uuid)
    }

    pub fn add_client_to_sessions(&mut self, uuid: Uuid, conn_to_cache: Addr<WebSocketSession>) {
        self.sessions.insert(uuid, conn_to_cache);
    }

    // TODO: Move this handler over to WsSessionActor, no real need for it to be in this file
    // I'll also have to change the logic in the route to point to the Addr for the Session to send actor messages

    fn receive_payload_message(&mut self, id: Uuid, msg: &str) {
        tracing::debug!("[WS_SERVER_ACTOR] Received, Processing, UUID: {id}, msg: {msg}");
        let opt_addr = self.sessions.get(&id);
        let opt_addr2 = opt_addr.cloned();
        if let Some(ws_actor) = opt_addr2 {
            // Perform handling here
            let default_msg = serde_json::json!({"error":"Could not parse JSON message"});
            let default_msg_2 = default_msg.clone();
            let parsed_msg = handle_ws::handle_ws_msg(msg.to_string()).unwrap_or(default_msg);
            tracing::debug!("[WS_SERVER_ACTOR] Parsed WS MSG as: {:?}", parsed_msg);
            //
            let success_msg = KromerMessage(serde_json::to_string(&parsed_msg).unwrap_or(default_msg_2.to_string()));
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
        tracing::debug!("[WS_SERVER_ACTOR] Message received to remove UUID from cache: {uuid}");

        self.sessions.remove(&uuid);
    }
}

impl Handler<GetActiveSessions> for WebSocketServer {
    type Result = MessageResult<GetActiveSessions>;

    fn handle(&mut self, _: GetActiveSessions, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.sessions.keys().cloned().collect())
    }
}

impl Handler<CheckTokenExists> for WebSocketServer {
    type Result = bool;

    fn handle(&mut self, msg: CheckTokenExists, _ctx: &mut Self::Context) -> Self::Result {
        let CheckTokenExists(uuid) = msg;

        let result = self.check_token_exists(uuid);

        tracing::debug!("[WS_SERVER_ACTOR] Checked token for UUID: {}", uuid.to_string());
        result
    }
}

impl Handler<AddToken> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: AddToken, _ctx: &mut Self::Context) -> Self::Result {
        let AddToken(uuid, params) = msg;

        self.add_new_token(uuid, params);
        tracing::debug!("[WS_SERVER_ACTOR] Added token for UUID: {}", uuid.to_string());
    }
}

impl Handler<RemoveToken> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveToken, _ctx: &mut Self::Context) -> Self::Result {
        let RemoveToken(uuid) = msg;

        self.remove_token(uuid);
        tracing::debug!("[WS_SERVER_ACTOR] Removed token for UUID: {}", uuid.to_string());
    }
}

impl SystemService for WebSocketServer {}
impl Supervised for WebSocketServer {}
