use std::collections::HashMap;
use std::sync::Arc;

use actix::prelude::*;
use actix::{Actor, Context, Handler, MessageResult};
use actix_broker::BrokerSubscribe;
use surrealdb::Surreal;
use surrealdb::{engine::any::Any as SurrealAny, Uuid};

use crate::ws::handler::handle_ws;
use crate::ws::types::actor_message::{AddToken, CheckTokenExists, RemoveToken};
use crate::ws::{
    actors::session::WsSessionActor,
    types::actor_message::{
        GetActiveSessions, GetCacheConnection, KromerMessage, ReceiveMessage,
        RemoveCacheConnection, SetCacheConnection,
    },
    types::common::TokenParams,
};

//#[derive(Default)]
pub struct WsServerActor {
    tokens: HashMap<Uuid, TokenParams>,
    sessions: HashMap<Uuid, Addr<WsSessionActor>>,
    _db_arc: Arc<Surreal<SurrealAny>>,
}

impl WsServerActor {
    pub fn new(_db_arc: Arc<Surreal<SurrealAny>>) -> Self {
        WsServerActor {
            tokens: HashMap::new(),
            sessions: HashMap::new(),
            _db_arc,
        }
    }

    pub fn add_new_token(&mut self, uuid: Uuid, params: TokenParams) {
        self.tokens.insert(uuid, params);
    }

    pub fn remove_token(&mut self, uuid: Uuid) {
        self.tokens.remove(&uuid);
    }

    pub fn check_token_exists(&mut self, uuid: Uuid) -> (bool, TokenParams) {
        if let Some(_value) = self.tokens.get(&uuid) {
            (true, _value.clone())
        } else {
            (false, TokenParams::default())
        }
    }

    pub fn add_client_to_sessions(&mut self, uuid: Uuid, conn_to_cache: Addr<WsSessionActor>) {
        self.sessions.insert(uuid, conn_to_cache);
    }

    // TODO: Move this handler over to WsSessionActor, no real need for it to be in this file
    // I'll also have to change the logic in the route to point to the Addr for the Session to send actor messages

    fn receive_payload_message(&mut self, id: Uuid, msg: &str) {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SERVER_ACTOR_RECEIVE");
        let _tracing_enter = tracing_span.enter();
        tracing::debug!("Received, Processing, UUID: {id}, msg: {msg}");
        let opt_addr = self.sessions.get(&id);
        let opt_addr2 = opt_addr.cloned();
        if let Some(ws_actor) = opt_addr2 {
            // Perform handling here
            let default_msg = serde_json::json!({"error":"Could not parse JSON message"});
            let default_msg_2 = default_msg.clone();
            let parsed_msg = handle_ws::handle_ws_msg(msg.to_string()).unwrap_or(default_msg);
            tracing::debug!("Parsed WS MSG as: {:?}", parsed_msg);
            //
            let success_msg = KromerMessage(
                serde_json::to_string(&parsed_msg).unwrap_or(default_msg_2.to_string()),
            );
            let future = async move {
                let _ = ws_actor.send(success_msg).await;
            };

            actix::spawn(future);
        } else {
            tracing::debug!("Could not find session for {id}");
        }
    }
}

impl Actor for WsServerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SERVER_ACTOR_START");
        let _tracing_enter = tracing_span.enter();
        tracing::info!("Started WS Server Actor");
        self.subscribe_system_async::<ReceiveMessage>(ctx);
    }
}

impl Handler<ReceiveMessage> for WsServerActor {
    type Result = ();

    fn handle(&mut self, msg: ReceiveMessage, _ctx: &mut Self::Context) {
        let ReceiveMessage(id, msg) = msg;
        self.receive_payload_message(id, &msg);
    }
}

impl Handler<SetCacheConnection> for WsServerActor {
    type Result = ();

    fn handle(&mut self, msg: SetCacheConnection, _ctx: &mut Self::Context) {
        let SetCacheConnection(uuid, conn_to_cache) = msg;

        self.add_client_to_sessions(uuid, conn_to_cache);
    }
}

impl Handler<GetCacheConnection> for WsServerActor {
    type Result = MessageResult<GetCacheConnection>;

    fn handle(&mut self, msg: GetCacheConnection, _ctx: &mut Self::Context) -> Self::Result {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SERVER_ACTOR_REMOVE_CACHE");
        let _tracing_enter = tracing_span.enter();
        let GetCacheConnection(uuid) = msg;

        let result = self.sessions.get(&uuid).cloned();
        tracing::debug!(
            "Message received to check UUID: {}, Result: {:?}",
            uuid,
            result
        );
        MessageResult(result)
    }
}

impl Handler<RemoveCacheConnection> for WsServerActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveCacheConnection, _ctx: &mut Self::Context) -> Self::Result {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SERVER_ACTOR_REMOVE_CACHE");
        let _tracing_enter = tracing_span.enter();
        let RemoveCacheConnection(uuid) = msg;
        tracing::debug!("Message received to remove UUID from cache: {uuid}");

        self.sessions.remove(&uuid);
    }
}

impl Handler<GetActiveSessions> for WsServerActor {
    type Result = MessageResult<GetActiveSessions>;

    fn handle(&mut self, _: GetActiveSessions, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.sessions.keys().cloned().collect())
    }
}

impl Handler<CheckTokenExists> for WsServerActor {
    type Result = MessageResult<CheckTokenExists>;

    fn handle(&mut self, msg: CheckTokenExists, _ctx: &mut Self::Context) -> Self::Result {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SERVER_ACTOR_CHECK_TOKEN");
        let _tracing_enter = tracing_span.enter();
        let CheckTokenExists(uuid) = msg;

        let (exists, token_params) = self.check_token_exists(uuid);

        tracing::debug!(
            "Checked token for UUID: {}, Exists: {}",
            uuid.to_string(),
            exists
        );

        MessageResult((exists, token_params.address))
    }
}

impl Handler<AddToken> for WsServerActor {
    type Result = ();

    fn handle(&mut self, msg: AddToken, _ctx: &mut Self::Context) -> Self::Result {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SERVER_ACTOR_ADD_TOKEN");
        let _tracing_enter = tracing_span.enter();
        let AddToken(uuid, params) = msg;

        self.add_new_token(uuid, params);
        tracing::debug!("Added token for UUID: {}", uuid.to_string());
    }
}

impl Handler<RemoveToken> for WsServerActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveToken, _ctx: &mut Self::Context) -> Self::Result {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SERVER_ACTOR_REMOVE_TOKEN");
        let _tracing_enter = tracing_span.enter();
        let RemoveToken(uuid) = msg;
        // check if it exists, just so we don't have any weird behavior...
        if self.check_token_exists(uuid).0 {
            self.remove_token(uuid);
            tracing::debug!("Removed token for UUID: {}", uuid.to_string());
        } else {
            tracing::debug!(
                "Token was already removed (30s lifetime), ignoring: {}",
                uuid.to_string()
            );
        }
    }
}

//impl SystemService for WebSocketServer {}
impl Supervised for WsServerActor {}
