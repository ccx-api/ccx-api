use serde::Deserialize;
use serde::Serialize;

use crate::client::recv_window::RecvWindow;
use crate::types::timestamp::BinanceTimestamp;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeWindow {
    timestamp: BinanceTimestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recv_window: Option<RecvWindow>,
}

impl TimeWindow {
    pub fn new(timestamp: BinanceTimestamp) -> Self {
        let recv_window = None;
        TimeWindow {
            timestamp,
            recv_window,
        }
    }

    pub fn now() -> Self {
        TimeWindow::new(BinanceTimestamp::now())
    }
}
