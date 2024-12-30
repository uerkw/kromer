pub mod message;
pub mod server;
pub mod session;

use std::str::FromStr;

use actix_web::{
    web::{self, Data},
    Error, HttpRequest, Responder,
};

use actix_web_actors::ws;
use message::GetCacheConnection;
use names::Generator;

use surrealdb::Uuid;

use crate::AppState;



pub fn generate_room_name() -> String {
    let mut generator = Generator::default();
    let gen_name_1 = generator.next().unwrap();
    let gen_name_2 = generator.next().unwrap();
    format!("{gen_name_1}-{gen_name_2}")
}


// This is the main Service for the WebSocket at "/ws/token/{id}"
pub async fn payload_ws(
    req: HttpRequest,
    stream: web::Payload,
    state: Data<AppState>,
    token: web::Path<String>,
) -> Result<impl Responder, Error> {
    // Grab our app state
    //let db = &state.db;
    let ws_manager = (&state.ws_manager).clone();

    // Extract the token
    let token = token.into_inner();
    tracing::debug!("Token was {token}");


    let session_id = Uuid::from_str(&token).expect("Could not parse UUID");

    let cache_lookup_msg = GetCacheConnection(session_id);
    let session = ws_manager.send(cache_lookup_msg).await.expect("Could not find token in the cache");

    let response_ws= ws::start(session.unwrap(), &req, stream);

    // session.

    response_ws
}

#[derive(Debug, Clone)]
enum KromerAddress {
    Guest,
    Custom
}

impl Default for KromerAddress {
    fn default() -> Self {
        KromerAddress::Guest
    }
}

impl KromerAddress {
    fn from_string(input: String) -> Self {
        if input == "guest" {
            KromerAddress::Guest
        } else {
            KromerAddress::Custom
        }
    }
}


#[derive(Default)]
struct KromerWsSubList {
    subscriptions: Vec<WebSocketSubscriptionType>,
}

impl Clone for KromerWsSubList {
    fn clone(&self) -> Self {
        KromerWsSubList{
            subscriptions: self.subscriptions.clone()
        }
    }
}

impl KromerWsSubList {
    pub fn new() -> Self {
        KromerWsSubList {
            subscriptions: vec![WebSocketSubscriptionType::from_str("ownTransactions").unwrap()],
        }
    }
}

#[derive(Clone)]
enum WebSocketSubscriptionType {
    Transactions,
    OwnTransactions,
    Names,
    OwnNames,
    Motd,
}

impl FromStr for WebSocketSubscriptionType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "transactions" => Ok(WebSocketSubscriptionType::Transactions),
            "ownTransactions" => Ok(WebSocketSubscriptionType::OwnTransactions),
            "names" => Ok(WebSocketSubscriptionType::Names),
            "ownNames" => Ok(WebSocketSubscriptionType::OwnNames),
            "motd" => Ok(WebSocketSubscriptionType::Motd),
            _ => Err(()),
        }
    }
}

impl ToString for WebSocketSubscriptionType {
    fn to_string(&self) -> String {
        match self {
            WebSocketSubscriptionType::Transactions => "transactions".to_string(),
            WebSocketSubscriptionType::OwnTransactions => "ownTransactions".to_string(),
            WebSocketSubscriptionType::Names => "names".to_string(),
            WebSocketSubscriptionType::OwnNames => "ownNames".to_string(),
            WebSocketSubscriptionType::Motd => "motd".to_string(),
        }
    }
}
