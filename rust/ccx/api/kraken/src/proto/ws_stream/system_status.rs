use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SystemStatus {
    #[serde(default, rename = "connectionID")]
    pub connection_id: Option<u64>,
    pub event: String,
    pub status: String,
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_decode_system_status() {
        let input = r#"{
            "connectionID":16104859528062827651,
            "event":"systemStatus",
            "status":"online",
            "version":"1.9.0"
        }"#;
        let resp: UpstreamWebsocketMessage<WsEvent> = serde_json::from_str(input).unwrap();

        match resp {
            UpstreamWebsocketMessage::Event(WsEvent::SystemStatus(e)) => {
                assert_eq!(e.connection_id, Some(16104859528062827651));
                assert_eq!(e.event, "systemStatus");
            }
            _ => unreachable!(),
        }
    }
}
