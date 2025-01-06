use std::collections::HashMap;

fn _get_ws_handlers() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("address", "wsGetAddress"),
        ("make_transaction", "wsMakeTransaction"),
        ("me", "wsGetMe"),
        ("subscribe", "wsSubscribe"),
        ("unsubscribe", "wsUnsubscribe"),
        ("get_subscription_level", "wsGetSubscriptionLevel"),
        (
            "get_valid_subscription_levels",
            "wsGetValidSubscriptionLevels",
        ),
        ("login", "wsLogin"),
        ("logout", "wsLogout"),
    ])
}
