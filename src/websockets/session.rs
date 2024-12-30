use actix::prelude::*;
use actix::Addr;
use actix_broker::BrokerIssue;
//use actix_broker::BrokerSubscribe;
use actix_web_actors::ws;
use surrealdb::Uuid;

use crate::websockets::{
    message::{JoinRoom, KromerMessage, LeaveRoom, ListRooms, SendMessage},
    server::WebSocketServer,
};

use super::{KromerAddress, KromerWsSubList};

#[derive(Default)]
pub struct KromerWsSession {
    id: Uuid,
    room: String,
    name: Option<String>,
    manager: Option<Addr<WebSocketServer>>,
    address: Option<KromerAddress>,
    privatekey: Option<String>,
    subscriptions: KromerWsSubList,
}

impl KromerWsSession {
    pub fn new(
        id: Uuid,
        room: String,
        manager: Addr<WebSocketServer>,
        address: Option<String>,
        privatekey: Option<String>,
        name: Option<String>,
    ) -> Self {
        let name = name;
        let manager = Some(manager);
        let subscriptions = KromerWsSubList::new();
        let privatekey = privatekey;
        let address = Some(KromerAddress::from_string(address.unwrap()));
        let session = KromerWsSession {
            id,
            room,
            name,
            manager,
            address,
            privatekey,
            subscriptions,
        };

        return session;
    }

    pub fn join_room(
        &mut self,
        room_name: &str,
        socket_uuid: Uuid,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let room_name = room_name.to_owned();

        let leave_msg = LeaveRoom(self.room.clone(), self.id);

        // Issue sync comes from having the "BrokerIssue" trait in scope
        self.issue_system_sync(leave_msg, ctx);

        // Then send a join message for the new room
        let join_msg = JoinRoom(
            room_name.to_owned(),
            socket_uuid,
            self.name.clone(),
            ctx.address().recipient(),
        );

        WebSocketServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(id) = id {
                    act.id = id;
                    act.room = room_name;
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn list_rooms(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        WebSocketServer::from_registry()
            .send(ListRooms)
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Ok(rooms) = res {
                    for room in rooms {
                        ctx.text(room);
                    }
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn send_msg(&self, msg: &str) {
        let content = format!(
            "{}: {msg}",
            self.name.clone().unwrap_or_else(|| "anon".to_owned()),
        );

        let msg = SendMessage(self.room.clone(), self.id, content);

        // issue_async comes from having the `BrokerIssue` trait in scope.
        self.issue_system_async(msg);
    }
}

impl Clone for KromerWsSession {
    fn clone(&self) -> Self {
        KromerWsSession {
            id: self.id.clone(),
            room: self.room.clone(),
            name: self.name.clone(),
            manager: self.manager.clone(),
            address: self.address.clone(),
            privatekey: self.privatekey.clone(),
            subscriptions: self.subscriptions.clone(),
        }
    }
}

impl Actor for KromerWsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.join_room(self.room.clone().as_str(), self.id, ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::info!(
            "KromerWsSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_owned()),
            self.id,
            self.room
        );
    }
}

impl Handler<KromerMessage> for KromerWsSession {
    type Result = ();

    fn handle(&mut self, msg: KromerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for KromerWsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();

                if msg.starts_with('/') {
                    let mut command = msg.splitn(2, ' ');

                    match command.next() {
                        Some("/list") => self.list_rooms(ctx),

                        Some("/join") => {
                            if let Some(room_name) = command.next() {
                                let socket_uuid = self.id;
                                self.join_room(room_name, socket_uuid, ctx);
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }

                        Some("/name") => {
                            if let Some(name) = command.next() {
                                self.name = Some(name.to_owned());
                                ctx.text(format!("name changed to: {name}"));
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }

                        _ => ctx.text(format!("!!! unknown command: {msg:?}")),
                    }

                    return;
                }
                self.send_msg(msg);
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
