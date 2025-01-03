use std::str::FromStr;

#[derive(Default, Clone)]
pub struct TokenParams {
    pub address: String,
    pub privatekey: String,
}

#[derive(Default, Clone)]
pub struct KromerWsSubList {
    _subscriptions: Vec<WebSocketSubscriptionType>,
}

impl KromerWsSubList {
    pub fn new() -> Self {
        KromerWsSubList {
            _subscriptions: vec![WebSocketSubscriptionType::from_str("ownTransactions").unwrap()],
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
