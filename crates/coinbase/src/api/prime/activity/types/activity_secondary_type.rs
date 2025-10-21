use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]

pub enum ActivitySecondaryType {
    #[serde(rename = "NO_SECONDARY_TYPE")]
    NoSecondaryType,
    #[serde(rename = "ACTIVITY_SECONDARY_TYPE_BUY")]
    Buy,
    #[serde(rename = "ACTIVITY_SECONDARY_TYPE_SELL")]
    Sell,
    #[serde(rename = "ACTIVITY_SECONDARY_TYPE_INTERNAL_TRANSFER")]
    InternalTransfer,
    #[serde(rename = "ACTIVITY_SECONDARY_TYPE_SWEEP_TRANSFER_TYPE")]
    SweepTransfer,
    #[serde(rename = "ACTIVITY_SECONDARY_TYPE_WEB3_SIGNER")]
    Web3Signer,
    #[serde(rename = "ACTIVITY_SECONDARY_TYPE_WEB3_WALLET")]
    Web3Wallet,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(ActivitySecondaryType);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(ActivitySecondaryType);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_roundtrip() {
        for typ in enum_iterator::all::<ActivitySecondaryType>() {
            let s = serde_json::to_string(&typ).unwrap();
            let typ2: ActivitySecondaryType = serde_json::from_str(&s).unwrap();
            assert_eq!(typ, typ2);
        }
    }
}
