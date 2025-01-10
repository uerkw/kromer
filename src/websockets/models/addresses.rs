use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AddressJson {
    pub address: String,
    pub balance: i64,
    #[serde(rename = "totalin")]
    pub total_in: i64,
    #[serde(rename = "totalout")]
    pub total_out: i64,
    #[serde(rename = "firstseen")]
    pub first_seen: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub authed: bool,
    pub address: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
    struct ExpectedResponse {
        pub ok: bool,
        pub address: AddressJson,
    }

    #[test]
    fn test_deserialize() {
        let response = r#"{"ok":true,"address":{"address":"kre3w0i79j","balance":86945,"totalin":123364,"totalout":38292,"firstseen":"2015-03-13T12:55:18.000Z"}}"#;
        let response: ExpectedResponse =
            serde_json::from_str(response).expect("Failed to deserialize");
        assert_eq!(response.ok, true);
    }

    #[test]
    fn test_serialize() {
        let response = ExpectedResponse {
            ok: true,
            address: AddressJson {
                address: "kre3w0i79j".to_owned(),
                balance: 86945,
                total_in: 123364,
                total_out: 38292,
                first_seen: "2015-03-13T12:55:18.000Z".to_owned(),
                names: None,
            },
        };
        let response_str = serde_json::to_string(&response).expect("Failed to serialize");
        let response_str_test = r#"{"ok":true,"address":{"address":"kre3w0i79j","balance":86945,"totalin":123364,"totalout":38292,"firstseen":"2015-03-13T12:55:18.000Z"}}"#;

        assert_eq!(response_str, response_str_test);
    }
}
