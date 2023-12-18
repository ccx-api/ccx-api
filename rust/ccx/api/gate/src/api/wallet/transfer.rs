use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::Request;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletTransferRequest {
    /// Transfer currency. For futures account, currency can be set to POINT or settle currency
    pub currency: SmartString,
    /// Account to transfer from
    pub from: WalletAccountEnum,
    /// Account to transfer to
    pub to: WalletAccountEnum,
    /// Transfer amount
    pub amount: Decimal,
    /// Margin currency pair. Required if transfer from or to margin account
    pub currency_pair: Option<SmartString>,
    /// Futures settle currency. Required if transferring from or to futures account
    pub settle: Option<SmartString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletAccountEnum {
    /// Currently unsupported.
    Cbbc,
    CrossMargin,
    Delivery,
    /// Currently unsupported.
    Finance,
    Futures,
    Margin,
    Options,
    /// Currently unsupported.
    Payment,
    /// Currently unsupported.
    Quant,
    Spot,
    /// Currently unsupported.
    Warrant,
}

impl Request for WalletTransferRequest {
    const METHOD: ApiMethod = ApiMethod::Post;
    const VERSION: ApiVersion = ApiVersion::V4;
    const PATH: &'static str = "wallet/transfers";
    const IS_PUBLIC: bool = false;
    type Response = WalletTransferResponse;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletTransferResponse {
    /// Order id (Transaction id)
    pub tx_id: i64,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;
    use crate::GateApi;

    impl<S: GateSigner> GateApi<S> {
        /// # Transfer between trading accounts
        ///
        /// Transfer between trading accounts
        ///
        /// Transfer between different accounts. Currently support transfers between the following:
        ///
        /// * spot - margin
        /// * spot - futures(perpetual)
        /// * spot - delivery
        /// * spot - cross margin
        /// * spot - options
        ///
        /// ## Parameters
        ///
        /// * `currency` - Transfer currency. For futures account, currency can be set to POINT or
        ///   settle currency.
        /// * `from` - Account to transfer from
        /// * `to` - Account to transfer to
        /// * `amount` - Transfer amount
        /// * `currency_pair` - Margin currency pair. Required if transfer from or to margin
        ///    account.
        /// * `settle` - Futures settle currency. Required if transferring from or to futures
        ///    account.
        pub async fn wallet_transfer(
            &self,
            currency: SmartString,
            from: WalletAccountEnum,
            to: WalletAccountEnum,
            amount: Decimal,
            currency_pair: Option<SmartString>,
            settle: Option<SmartString>,
        ) -> Result<<WalletTransferRequest as Request>::Response, RequestError> {
            self.request(&WalletTransferRequest {
                currency,
                from,
                to,
                amount,
                currency_pair,
                settle,
            })
            .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_docs() {
        let json = r#"{
            "tx_id": 59636381286
        }"#;
        let res: WalletTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(res, WalletTransferResponse { tx_id: 59636381286 });
    }
}
