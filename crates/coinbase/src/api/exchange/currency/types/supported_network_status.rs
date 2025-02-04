use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SupportedNetworkStatus {
    Online,
    Offline,
    /// The network has been delisted.
    /// Isn't available for deposits/withdrawals.
    Delisted,
}
