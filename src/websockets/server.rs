use std::collections::HashMap;

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use surrealdb::Uuid;

use crate::websockets::message::{JoinRoom, KromerMessage, LeaveRoom, ListRooms, SendMessage};

type Client = Recipient<KromerMessage>;
type Room = HashMap<Uuid, Client>;

use names::Generator;

use super::{
    message::{CreateRoom, GetActiveSessions, GetCacheConnection, SetCacheConnection},
    session::KromerWsSession,
};

pub type CachedWebSocket = Addr<KromerWsSession>;

#[derive(Default)]
pub struct WebSocketServer {
    rooms: HashMap<String, Room>,
    sessions: HashMap<Uuid, KromerWsSession>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        WebSocketServer {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    pub fn generate_room_name() -> String {
        let mut generator = Generator::default();
        let gen_name_1 = generator.next().unwrap();
        let gen_name_2 = generator.next().unwrap();
        format!("{gen_name_1}-{gen_name_2}")
    }

    fn create_room(&mut self) -> String {
        let room = HashMap::new();
        let rand_name = WebSocketServer::generate_room_name();

        self.rooms.insert(rand_name.clone(), room);

        rand_name
    }

    fn take_room(&mut self, room_name: &str) -> Option<Room> {
        // Get a mut ref to the Option inside HashMap
        let room = self.rooms.get_mut(room_name);

        // "take" the value out of the Option
        if let Some(room) = room {
            return Some(std::mem::take(room));
        }

        None
    }

    pub fn add_client_to_sessions(&mut self, uuid: Uuid, conn_to_cache: KromerWsSession) {
        self.sessions.insert(uuid, conn_to_cache);
    }

    fn add_client_to_room(&mut self, room_name: &str, uuid: Option<Uuid>, client: Client) -> Uuid {
        let mut parsed_uuid = uuid.unwrap();

        if let Some(room) = self.rooms.get_mut(room_name) {
            loop {
                if room.contains_key(&parsed_uuid) {
                    parsed_uuid = Uuid::new_v4();
                } else {
                    break;
                }
            }

            room.insert(parsed_uuid, client);
            return parsed_uuid;
        }

        let mut room: Room = HashMap::new();
        // Create a new room for the first client
        room.insert(parsed_uuid, client);
        self.rooms.insert(room_name.to_owned(), room);

        parsed_uuid
    }

    fn send_payload_message(&mut self, room_name: &str, msg: &str, _src: Uuid) -> Option<()> {
        let mut room = self.take_room(room_name)?;

        for (id, client) in room.drain() {
            if client.try_send(KromerMessage(msg.to_owned())).is_ok() {
                self.add_client_to_room(room_name, Some(id), client);
            }
        }

        Some(())
    }
}

impl Actor for WebSocketServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<LeaveRoom>(ctx);
        self.subscribe_system_async::<SendMessage>(ctx);
    }
}

impl Handler<JoinRoom> for WebSocketServer {
    type Result = MessageResult<JoinRoom>;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom(room_name, uuid, client_name, client) = msg;
        let id = self.add_client_to_room(&room_name, Some(uuid), client);
        let join_msg = format!(
            "{} joined {room_name}",
            client_name.unwrap_or_else(|| "anon".to_owned()),
        );

        self.send_payload_message(&room_name, &join_msg, id);
        MessageResult(id)
    }
}

impl Handler<CreateRoom> for WebSocketServer {
    type Result = MessageResult<CreateRoom>;

    fn handle(&mut self, msg: CreateRoom, _ctx: &mut Self::Context) -> Self::Result {
        let CreateRoom(client_name) = msg;
        let room_name = self.create_room();

        MessageResult(room_name)
    }
}

impl Handler<LeaveRoom> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) {
        if let Some(room) = self.rooms.get_mut(&msg.0) {
            room.remove(&msg.1);
        }

        // Remove from the sessions cache
        self.sessions.remove(&msg.1);
    }
}

impl Handler<ListRooms> for WebSocketServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.rooms.keys().cloned().collect())
    }
}

impl Handler<SendMessage> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) {
        let SendMessage(room_name, id, msg) = msg;
        self.send_payload_message(&room_name, &msg, id);
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

// pub struct GetActiveSessions;

// impl Message for GetActiveSessions {
//     type Result = Vec<Uuid>;
// }

impl Handler<GetActiveSessions> for WebSocketServer {
    type Result = MessageResult<GetActiveSessions>;

    fn handle(&mut self, _: GetActiveSessions, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.sessions.keys().cloned().collect())
    }
}

impl SystemService for WebSocketServer {}
impl Supervised for WebSocketServer {}
