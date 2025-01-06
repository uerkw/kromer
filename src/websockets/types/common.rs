use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub struct WebSocketTokenData {
    pub address: String,
    pub privatekey: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct WebSocketSubscriptionList {
    _subscriptions: Vec<WebSocketSubscriptionType>,
}

impl WebSocketSubscriptionList {
    pub fn new() -> Self {
        WebSocketSubscriptionList {
            _subscriptions: vec![WebSocketSubscriptionType::from_str("ownTransactions").unwrap()],
        }
    }
}

#[derive(Clone, Debug)]
pub enum WebSocketSubscriptionType {
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

impl fmt::Display for WebSocketSubscriptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebSocketSubscriptionType::Transactions => write!(f, "transactions"),
            WebSocketSubscriptionType::OwnTransactions => write!(f, "ownTransactions"),
            WebSocketSubscriptionType::Names => write!(f, "names"),
            WebSocketSubscriptionType::OwnNames => write!(f, "ownNames"),
            WebSocketSubscriptionType::Motd => write!(f, "motd"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WebSocketMessageType {
    Address,
    Login,
    Logout,
    Me,
    Subscribe,
    GetSubscriptionLevel,
    GetValidSubscriptionLevels,
    Unsubscribe,
    MakeTransaction,
}
