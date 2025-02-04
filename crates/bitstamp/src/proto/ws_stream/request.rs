use serde::Deserialize;
use serde::Serialize;

use super::WsSubscription;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(tag = "event", content = "data")]
pub enum WsCommand {
    #[serde(rename = "bts:subscribe")]
    Subscribe(WsSubscription),
    #[serde(rename = "bts:unsubscribe")]
    Unsubscribe(WsSubscription),
}
