use crate::ws::actors::server::WebSocketServer;
use crate::ws::types::actor_message::{
    CloseWebSocket, KromerMessage, ReceiveMessage, RemoveCacheConnection,
};
use crate::ws::types::session::{KromerAddress, KromerWsSubList};
use actix::prelude::*;
use actix::Actor;
use actix_ws as ws;
use surrealdb::Uuid;

#[derive(Clone)]
pub struct WebSocketSession {
    id: Uuid,
    _address: Option<KromerAddress>,
    _privatekey: Option<String>,
    _subscriptions: KromerWsSubList,
    ws_session: Option<ws::Session>,
    ws_manager: Addr<WebSocketServer>,
}

impl WebSocketSession {
    pub fn new(
        id: Uuid,
        address: Option<KromerAddress>,
        privatekey: Option<String>,
        ws_session: ws::Session,
        ws_manager: Addr<WebSocketServer>,
    ) -> Self {
        let subscriptions = KromerWsSubList::new();
        let address = address.or_else(|| Some(KromerAddress::from_string("guest".to_string())));
        let ws_session = Some(ws_session);
        WebSocketSession {
            id,
            _address: address,
            _privatekey: privatekey,
            _subscriptions: subscriptions,
            ws_session,
            ws_manager,
        }
    }

    pub fn recieve_msg(&self, msg: &str) {
        let msg = ReceiveMessage(self.id, msg.to_string());
        tracing::debug!("[WS_SESSION_ACTOR][RECEIVE] {}", msg.to_string());
        //self.issue_system_async(msg);
    }
}

impl Actor for WebSocketSession {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::debug!("[WS_SESSION_ACTOR] Started WS Actor for {}", self.id);
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // Close the WS
        if let Some(session) = self.ws_session.take() {
            let future = async move {
                let _ = session.close(None).await;
            };

            actix::spawn(future);
        }
        // Stop the actor
        _ctx.stop();

        // Info to console
        tracing::debug!(
            "[WS_SESSION_ACTOR] Kromer WS Session closed for ID: {}",
            self.id
        )
    }
}

impl Handler<KromerMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: KromerMessage, _ctx: &mut Self::Context) {
        let KromerMessage(msg) = msg;
        //ctx.text(msg.0);
        let mut ws_session = self.ws_session.clone().unwrap();
        let future = async move {
            let _ = ws_session.text(msg).await;
        };

        actix::spawn(future);
    }
}

impl Handler<CloseWebSocket> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, _msg: CloseWebSocket, _ctx: &mut Self::Context) {
        let CloseWebSocket(close_reason) = _msg;
        let cloned_close_reason = close_reason.clone();
        // Close the WS
        if let Some(session) = self.ws_session.take() {
            let future = async move {
                let _ = session.close(Some(close_reason)).await;
            };

            actix::spawn(future);
        }
        tracing::debug!(
            "[WS_SESSION_ACTOR] Receiving WS Close Request with Code: {:?} Reason: {}",
            cloned_close_reason.code,
            cloned_close_reason.description.unwrap_or_default()
        );

        let uuid_to_remove = self.id.clone();
        let thread_ws_manager = self.ws_manager.clone();
        let future = async move {
            let remove_from_cache_msg = RemoveCacheConnection(uuid_to_remove);
            let _ = thread_ws_manager.send(remove_from_cache_msg).await;
        };

        actix::spawn(future);

        // Stop the actor
        _ctx.stop()
    }
}