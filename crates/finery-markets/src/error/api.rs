use enum_iterator::Sequence;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use thiserror::Error;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq, Sequence, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum ApiFineryError {
    #[error("Ok")]
    Ok = 0,
    #[error("Not implemented")]
    NotImplemented = 1,
    #[error("Not connected")]
    NotConnected = 2,
    #[error("Not authorised")]
    NotAuthorised = 3,
    #[error("Already authorised")]
    AlreadyAuthorised = 4,
    #[error("Invalid password")]
    InvalidPassword = 5,
    #[error("Invalid nonce or signature")]
    InvalidNonceSignature = 6,
    #[error("Invalid timestamp")]
    InvalidTimestamp = 7,
    #[error("API method not available")]
    MethodNotAvailable = 8,
    #[error("API method parameter is invalid")]
    MethodParameterInvalid = 9,
    #[error("Internal error")]
    InternalError = 10,
    #[error("Invalid MFA code")]
    InvalidMfaCode = 11,
    #[error("MFA not enabled")]
    MfaNotEnabled = 12,
    #[error("User agreement not signed")]
    UserAgreementNotSigned = 13,
    #[error("The signature of your request is invalid")]
    InvalidSignature = 14,
    #[error("Link expired")]
    LinkExpired = 15,
    #[error("Invalid currency flags")]
    InvalidCurrencyFlags = 20,
    #[error("Invalid currency price")]
    InvalidCurrencyPrice = 21,
    #[error("Invalid currency balance step")]
    InvalidCurrencyBalanceStep = 22,
    #[error("Invalid currency name")]
    InvalidCurrencyName = 23,
    #[error("Currency name cannot be changed")]
    CurrencyNameCannotChanged = 24,
    #[error("Currency balance step cannot be changed")]
    CurrencyBalanceStepCannotChanged = 25,
    #[error("Currency not found")]
    CurrencyNotFound = 26,
    #[error("Currency cannot be removed")]
    CurrencyCannotRemoved = 27,
    #[error("Invalid instrument flags")]
    InvalidInstrumentFlags = 30,
    #[error("Invalid instrument name")]
    InvalidInstrumentName = 31,
    #[error("Instrument asset currency cannot be changed")]
    InstrumentAssetCurrencyCannotChanged = 32,
    #[error("Instrument balance currency cannot be changed")]
    InstrumentBalanceCurrencyCannotChanged = 33,
    #[error("Instrument not found")]
    InstrumentNotFound = 34,
    #[error("Instrument cannot be removed")]
    InstrumentCannotRemoved = 35,
    #[error("Invalid client flags")]
    InvalidClientFlags = 40,
    #[error("Invalid client taker delta ratio")]
    InvalidClientTakerDeltaRatio = 41,
    #[error("Invalid name")]
    InvalidName = 42,
    #[error("Client type cannot be changed")]
    ClientTypeCannotChanged = 43,
    #[error("Client already exists")]
    ClientAlreadyExists = 44,
    #[error("Client not found")]
    ClientNotFound = 45,
    #[error("User name already exists")]
    UserNameAlreadyExists = 46,
    #[error("Invalid limit flags")]
    InvalidLimitFlags = 50,
    #[error("Invalid limit net limit")]
    InvalidLimitNetLimit = 51,
    #[error("Invalid limit gross limit")]
    InvalidLimitGrossLimit = 52,
    #[error("Limit not found")]
    LimitNotFound = 53,
    #[error("Limit clients are identical")]
    LimitClientsIdentical = 54,
    #[error("Limit client types are identical")]
    LimitClientTypesIdentical = 55,
    #[error("Invalid settlement order flags")]
    InvalidSettlementOrderFlags = 60,
    #[error("Invalid settlement order size")]
    InvalidSettlementOrderSize = 61,
    #[error("Invalid settlement order comment")]
    InvalidSettlementOrderComment = 62,
    #[error("Identical settlement clients")]
    IdenticalSettlementClients = 63,
    #[error("Settlement not found")]
    SettlementNotFound = 64,
    #[error("Settlement order is from transaction")]
    SettlementOrderFromTransaction = 65,
    #[error("Invalid order size")]
    InvalidOrderSize = 70,
    #[error("Invalid order price")]
    InvalidOrderPrice = 71,
    #[error("Invalid order flags")]
    InvalidOrderFlags = 72,
    #[error("Order type not allowed")]
    OrderTypeNotAllowed = 73,
    #[error("Client order id already in use")]
    ClientOrderIdAlreadyUse = 74,
    #[error("Add failed - Post-Only")]
    AddFailedPostOnly = 75,
    #[error("Add failed - IOC: no orders to match")]
    AddFailedIOCNoOrdersMatch = 76,
    #[error("Add failed - FOK: not enough liquidity")]
    AddFailedFOKNotEnoughLiquidity = 77,
    #[error("Add failed - SMP (self-trade prevention)")]
    AddFailedSMPTradePrevention = 78,
    #[error("Add failed - limits")]
    AddFailedLimits = 79,
    #[error("Del failed - not found")]
    DelFailedNotFound = 80,
    #[error("Either volume or size should be specified")]
    EitherVolumeSizeShouldSpecified = 81,
    #[error("Orders by volume not supported for makers")]
    OrdersVolumeNotSupportedForMakers = 82,
    #[error("Invalid order volume")]
    InvalidOrderVolume = 83,
    #[error("Trading not allowed")]
    TradingNotAllowed = 84,
    #[error("No open positions in order currencies")]
    NoOpenPositionsInOrderCurrencies = 85,
    #[error("Mod failed - no size after decrement")]
    ModFailedNoSizeAfterDecrement = 90,
    #[error("Mod failed - side mismatch")]
    ModFailedSideMismatch = 91,
    #[error("Binding already exists")]
    BindingAlreadyExists = 100,
    #[error("Binding not found")]
    BindingNotFound = 101,
    #[error("Invalid feed name")]
    InvalidFeedName = 102,
    #[error("Invalid feed id")]
    InvalidFeedId = 103,
    #[error("Database out-of-sync")]
    DatabaseOutOfSync = 104,
    #[error("Field Required")]
    FieldRequired = 110,
    #[error("Field Invalid")]
    FieldInvalid = 111,
    #[error("Poor Username")]
    PoorUsername = 112,
    #[error("Poor Password")]
    PoorPassword = 113,
    #[error("Password Change Required")]
    PasswordChangeRequired = 114,
    #[error("Maximum number of keys reached")]
    MaximumNumberKeysReached = 120,
    #[error("Key not found")]
    KeyNotFound = 121,
    #[error("Settlement request already exists")]
    SettlementRequestAlreadyExists = 130,
    #[error("Settlement request not found")]
    SettlementRequestNotFound = 131,
    #[error("Invalid settlement request flags")]
    InvalidSettlementFlags = 132,
    #[error("Invalid settlement request counterparty")]
    InvalidSettlementRequestCounterparty = 133,
    #[error("Invalid settlement request comment")]
    InvalidSettlementRequestComment = 134,
    #[error("Invalid settlement request amount130 ")]
    InvalidSettlementRequestAmount130 = 135,
    #[error("Invalid settlement transaction flags")]
    InvalidSettlementTransactionFlags = 140,
    #[error("Invalid settlement transaction amount")]
    InvalidSettlementTransactionAmount = 141,
    #[error("Invalid settlement transaction txId")]
    InvalidSettlementTransactionTxId = 142,
    #[error("Identical clients not allowed")]
    IdenticalClientsNotAllowed = 143,
    #[error("Settlement transaction not found")]
    SettlementTransactionNotFound = 144,
    #[error("Settlement order client A global net limit breached")]
    SettlementOrderClientAGlobalNetLimitBreached = 160,
    #[error("Settlement order client A global gross limit breached")]
    SettlementOrderClientAGlobalGrossLimitBreached = 161,
    #[error("Settlement order client B global net limit breached")]
    SettlementOrderClientBGlobalNetLimitBreached = 162,
    #[error("Settlement order client B global gross limit breached")]
    SettlementOrderClientBGlobalGrossLimitBreached = 163,
    #[error("Settlement order client A counterparty net limit breached")]
    SettlementOrderClientACounterpartyNetLimitBreached = 164,
    #[error("Settlement order client A counterparty gross limit breached")]
    SettlementOrderClientACounterpartyGrossLimitBreached = 165,
    #[error("Settlement order client B counterparty net limit breached")]
    SettlementOrderClientBCounterpartyNetLimitBreached = 166,
    #[error("Settlement order client B counterparty gross limit breached")]
    SettlementOrderClientBCounterpartyGrossLimitBreached = 167,
}

impl ApiFineryError {
    #[allow(dead_code)]
    pub fn code(&self) -> u16 {
        *self as u16
    }

    #[allow(dead_code)]
    pub fn message(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use enum_iterator::all;

    use super::*;

    #[test]
    fn test_finery_error_repr() {
        for item in all::<ApiFineryError>() {
            log::debug!(
                "test_finery_error_repr item :: {:?}; {}; {}",
                item,
                item.code(),
                item.message()
            );
        }
    }

    #[test]
    fn test_finery_error_serde() {
        #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
        struct Error {
            error: ApiFineryError,
        }

        let json = r#"{ "error": 2 }"#;

        let error: Error = serde_json::from_str(json).expect("Failed from_str");
        log::debug!("test_finery_error_serde :: {:?}", error);

        let json = serde_json::to_string(&error).expect("Failed to_string");

        let error_new: Error = serde_json::from_str(&json).expect("Failed from_str");
        log::debug!("test_finery_error_serde :: {:?}", error_new);
        assert_eq!(error, error_new);
    }
}
