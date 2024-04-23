use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum ActivityType {
    #[serde(rename = "OTHER_ACTIVITY_TYPE")]
    Other,
    #[serde(rename = "ACTIVITY_TYPE_LIMIT_ORDER")]
    LimitOrder,
    #[serde(rename = "ACTIVITY_TYPE_STOP_LIMIT_ORDER")]
    StopLimitOrder,
    #[serde(rename = "ACTIVITY_TYPE_MARKET_ORDER")]
    MarketOrder,
    #[serde(rename = "ACTIVITY_TYPE_TWAP_ORDER")]
    TwapOrder,
    #[serde(rename = "ACTIVITY_TYPE_DEPOSIT")]
    Deposit,
    #[serde(rename = "ACTIVITY_TYPE_WITHDRAWAL")]
    Withdrawal,
    #[serde(rename = "ACTIVITY_TYPE_INTERNAL_TRANSFER")]
    InternalTransfer,
    #[serde(rename = "ACTIVITY_TYPE_CREATE_WALLET")]
    CreateWallet,
    #[serde(rename = "ACTIVITY_TYPE_REMOVE_WALLET")]
    RemoveWallet,
    #[serde(rename = "ACTIVITY_TYPE_UPDATE_WALLET")]
    UpdateWallet,
    #[serde(rename = "ACTIVITY_TYPE_CAST_VOTE")]
    CastVote,
    #[serde(rename = "ACTIVITY_TYPE_ENABLE_VOTING")]
    EnableVoting,
    #[serde(rename = "ACTIVITY_TYPE_STAKE")]
    Stake,
    #[serde(rename = "ACTIVITY_TYPE_UNSTAKE")]
    Unstake,
    #[serde(rename = "ACTIVITY_TYPE_CHANGE_VALIDATOR")]
    ChangeValidator,
    #[serde(rename = "ACTIVITY_TYPE_RESTAKE")]
    Restake,
    #[serde(rename = "ACTIVITY_TYPE_ADDRESS_BOOK")]
    AddressBook,
    #[serde(rename = "ACTIVITY_TYPE_TEAM_MEMBERS")]
    TeamMembers,
    #[serde(rename = "ACTIVITY_TYPE_BILLING")]
    Billing,
    #[serde(rename = "ACTIVITY_TYPE_SECURITY")]
    Security,
    #[serde(rename = "ACTIVITY_TYPE_API")]
    Api,
    #[serde(rename = "ACTIVITY_TYPE_SETTINGS")]
    Settings,
    #[serde(rename = "ACTIVITY_TYPE_SMART_CONTRACT")]
    SmartContract,
    #[serde(rename = "ACTIVITY_TYPE_ALLOCATION_IN")]
    AllocationIn,
    #[serde(rename = "ACTIVITY_TYPE_ALLOCATION_OUT")]
    AllocationOut,
    #[serde(rename = "ACTIVITY_TYPE_ALLOCATION_IN_REVERSAL")]
    AllocationInReversal,
    #[serde(rename = "ACTIVITY_TYPE_ALLOCATION_OUT_REVERSAL")]
    AllocationOutReversal,
    #[serde(rename = "ACTIVITY_TYPE_CONVERSION")]
    Conversion,
    #[serde(rename = "ACTIVITY_TYPE_BLOCK_TRADE")]
    BlockTrade,
    #[serde(rename = "ACTIVITY_TYPE_VWAP_ORDER")]
    VwapOrder,
    #[serde(rename = "ACTIVITY_TYPE_WEB3_MESSAGE")]
    Web3Message,
    #[serde(rename = "ACTIVITY_TYPE_WEB3_TRANSACTION")]
    Web3Transaction,
    #[serde(rename = "ACTIVITY_TYPE_WEB3_DEVICE_RECOVERY")]
    Web3DeviceRecovery,
    #[serde(rename = "ACTIVITY_TYPE_WEB3_RECREATE_BACKUP")]
    Web3RecreateBackup,
    #[serde(rename = "ACTIVITY_TYPE_WEB3_ONBOARDING")]
    Web3Onboarding,
}
#[cfg(feature = "db")]
forward_display_to_serde!(ActivityType);
#[cfg(feature = "db")]
forward_from_str_to_serde!(ActivityType);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_roundtrip() {
        for typ in enum_iterator::all::<ActivityType>() {
            let s = serde_json::to_string(&typ).unwrap();
            let typ2: ActivityType = serde_json::from_str(&s).unwrap();
            assert_eq!(typ, typ2);
        }
    }
}
