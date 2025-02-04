use serde::Deserialize;
use serde::Serialize;

use crate::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeEstimate {
    fee: Decimal,
}
