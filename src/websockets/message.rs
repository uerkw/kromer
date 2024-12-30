use actix::prelude::*;
use surrealdb::Uuid;

use super::session::KromerWsSession;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct KromerMessage(pub String);

//// Protect this for Admins only
#[derive(Clone, Message)]
#[rtype(result = "Uuid")]
pub struct JoinRoom(pub String, pub Uuid, pub Option<String>, pub Recipient<KromerMessage>);

#[derive(Clone, Message)]
#[rtype(result = "String")]
pub struct CreateRoom(pub Option<String>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub Uuid);

//// Protect this for Admins only
#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
pub struct ListRooms;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String, pub Uuid, pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetCacheConnection(pub Uuid, pub KromerWsSession);

#[derive(Message)]
#[rtype(result="Option<KromerWsSession>")]
pub struct GetCacheConnection(pub Uuid);

#[derive(Clone, Message)]
#[rtype(result = "Vec<Uuid>")]
pub struct GetActiveSessions;