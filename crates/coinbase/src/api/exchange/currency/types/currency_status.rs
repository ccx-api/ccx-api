use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyStatus {
    Online,
    Offline,
    // FIXME is this real?
    Unavailable,
    // FIXME is this real?
    Maintenance,
    /// The currency has been delisted and is no longer available for trading.
    Delisted,
}
