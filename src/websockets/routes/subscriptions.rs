use crate::{models::websockets::{OutgoingWebSocketMessage, ResponseMessageType, WebSocketMessageType, WsSessionModification}, websockets::{types::common::WebSocketSubscriptionList, wrapped_ws::WrappedWsData}};



pub fn get_subscription_level(
    ws_metadata: &WrappedWsData,
    msg_id: String,
) -> WsSessionModification {

    // Just need to pull out the subscription levels from the wrapped data, and display it
    let subscription_levels = &ws_metadata.subs;

    WsSessionModification {
        msg_type: Some(OutgoingWebSocketMessage {
            ok: Some(true),
            id: msg_id.clone(),
            message: WebSocketMessageType::Response {
                message: ResponseMessageType::GetSubscriptionLevel {
                    subscription_level:  subscription_levels.to_owned().to_string(),
                }
                
            }
        }),
        wrapped_ws_data: None,
    }
}

pub fn get_valid_subscription_levels(
    msg_id: String,
) -> WsSessionModification {
    
    let valid_subscription_levels = WebSocketSubscriptionList::new_all_subs();

    WsSessionModification {
        msg_type: Some(OutgoingWebSocketMessage {
            ok: Some(true),
            id: msg_id.clone(),
            message: WebSocketMessageType::Response {
                message: ResponseMessageType::GetValidSubscriptionLevels {
                    valid_subscription_levels: valid_subscription_levels.to_string(),
                }
            }
        }),
        wrapped_ws_data: None, 
    }
}