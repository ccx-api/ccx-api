use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyStatus {
    Online,
    Offline,
    // FIXME is this real?
    Unavailable,
    // FIXME is this real?
    Maintenance,
}
