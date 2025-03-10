use serde::Deserialize;
use serde::Serialize;

use crate::client::recv_window::RecvWindow;
use crate::types::timestamp::MexcTimestamp;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeWindow {
    timestamp: MexcTimestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recv_window: Option<RecvWindow>,
}

impl TimeWindow {
    pub fn new(timestamp: MexcTimestamp) -> Self {
        let recv_window = None;
        TimeWindow {
            timestamp,
            recv_window,
        }
    }

    pub fn now() -> Self {
        TimeWindow::new(MexcTimestamp::now())
    }
}
