use std::sync::Arc;

use crate::ws::actors::server::WsServerActor;
use crate::ws::types::actor_message::{
    CloseWebSocket, KromerMessage, ReceiveMessage, RemoveCacheConnection,
};
use crate::ws::types::common::KromerWsSubList;
use actix::prelude::*;
use actix::Actor;
use actix_ws as ws;
use surrealdb::{engine::any::Any as SurrealAny, Surreal, Uuid};

#[derive(Clone)]
pub struct WsSessionActor {
    pub id: Uuid,
    pub _address: String,
    pub _privatekey: Option<String>,
    pub _subscriptions: KromerWsSubList,
    pub ws_session: Option<ws::Session>,
    pub ws_manager: Addr<WsServerActor>,
    pub db_arc: Arc<Surreal<SurrealAny>>,
}

impl WsSessionActor {
    pub fn new(
        id: Uuid,
        address: String,
        privatekey: Option<String>,
        ws_session: ws::Session,
        ws_manager: Addr<WsServerActor>,
        db_arc: Arc<Surreal<SurrealAny>>,
    ) -> Self {
        let subscriptions = KromerWsSubList::new();
        let address = address;
        let ws_session = Some(ws_session);
        WsSessionActor {
            id,
            _address: address,
            _privatekey: privatekey,
            _subscriptions: subscriptions,
            ws_session,
            ws_manager,
            db_arc,
        }
    }

    pub fn receive_msg(&self, msg: &str) {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SESSION_ACTOR_RECEIVE");
        let _tracing_enter = tracing_span.enter();
        let msg = ReceiveMessage(self.id, msg.to_string());
        tracing::debug!("Received: {}", msg.to_string());
        //self.issue_system_async(msg);
    }

    pub fn receive_payload_message(&mut self, _msg: &str) {}
}

impl Actor for WsSessionActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SESSION_ACTOR");
        let _tracing_enter = tracing_span.enter();
        tracing::debug!("Started WS Actor for {}", self.id);
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SESSION_ACTOR_STOPPED");
        let _tracing_enter = tracing_span.enter();
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
        tracing::debug!("Kromer WS Session closed for ID: {}", self.id)
    }
}

impl Handler<KromerMessage> for WsSessionActor {
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

impl Handler<CloseWebSocket> for WsSessionActor {
    type Result = ();

    fn handle(&mut self, _msg: CloseWebSocket, _ctx: &mut Self::Context) {
        let tracing_span = tracing::span!(tracing::Level::DEBUG, "WS_SESSION_ACTOR_CLOSE");
        let _tracing_enter = tracing_span.enter();
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
            "Receiving WS Close Request with Code: {:?} Reason: {}",
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

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMeHandle;

impl Handler<WsMeHandle> for WsSessionActor {
    type Result = ();

    fn handle(&mut self, _msg: WsMeHandle, ctx: &mut Self::Context) {
        tracing::debug!("Starting Handler<WsMeHandle>");

        let db_arc = self.db_arc.clone();
        tracing::debug!("Kromer address was {:?}", self._address.clone());

        let kromer_address = self._address.clone();
        let kromer_address_1 = kromer_address.clone();
        if !(kromer_address == "guest") {
            if let Some(_session) = self.ws_session.take() {
                let future = async move {
                    let _wallet_addr_info =
                        crate::database::models::wallet::Model::get_by_address_excl(
                            &db_arc,
                            kromer_address_1,
                        )
                        .await;
                    _wallet_addr_info
                }
                .into_actor(self)
                .map(|result, _actor, _ctx| match result {
                    Ok(wallet_info) => {
                        tracing::debug!("Wallet info: {:?}", wallet_info);
                    }
                    Err(e) => {
                        tracing::error!("Error getting wallet info: {:?}", e);
                    }
                });

                ctx.spawn(future);
            }
        }
    }
}
