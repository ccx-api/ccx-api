use ccx_api_lib::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::PublicRequest;
use crate::api::Request;

#[derive(Debug, Clone, Serialize)]
pub struct AllCurrenciesRequest;

impl Request for AllCurrenciesRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    type Response = Vec<Currency>;
}

impl PublicRequest for AllCurrenciesRequest {}

#[derive(Debug, Clone, Serialize)]
pub struct CurrencyRequest;

impl Request for CurrencyRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    type Response = Currency;
}

impl PublicRequest for CurrencyRequest {}

/// Represents the details of a currency.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Currency {
    /// Currency name
    pub currency: SmartString,
    /// Whether currency is de-listed
    pub delisted: bool,
    /// Whether currency's withdrawal is disabled
    pub withdraw_disabled: bool,
    /// Whether currency's withdrawal is delayed
    pub withdraw_delayed: bool,
    /// Whether currency's deposit is disabled
    pub deposit_disabled: bool,
    /// Whether currency's trading is disabled
    pub trade_disabled: bool,
    /// Fixed fee rate. Only for fixed rate currencies, not valid for normal currencies
    #[serde(default, with = "crate::util::maybe_str")]
    pub fixed_rate: Option<Decimal>,
    /// Chain of currency
    #[serde(default)]
    pub chain: Option<SmartString>,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::api::spot::SpotApi;
    use crate::client::rest::RequestError;

    impl<S> SpotApi<S> {
        /// List all currencies' details
        ///
        /// `GET /spot/currencies`
        ///
        /// Currency has two forms:
        /// * Only currency name, e.g., `BTC`, `USDT`
        /// * `<currency>_<chain>`, e.g., `HT_ETH`
        ///
        /// ## Parameters
        /// None
        pub async fn list_currencies(&self) -> Result<Vec<Currency>, RequestError> {
            let request = AllCurrenciesRequest;
            self.0.request("/spot/currencies", &request).await
        }

        /// Get details of a specific currency
        ///
        /// `GET /spot/currencies/{name}`
        ///
        /// Get details of a specific currency
        /// ## Parameters
        /// * `currency`
        pub async fn get_currency(&self, currency: &str) -> Result<Currency, RequestError> {
            let path = format!("/spot/currencies/{currency}");
            self.0.request(&path, &CurrencyRequest).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_currency() {
        let json = r#"{
            "currency": "GT",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false,
            "chain": "GT"
        }"#;

        let expected = Currency {
            currency: "GT".into(),
            delisted: false,
            withdraw_disabled: false,
            withdraw_delayed: false,
            deposit_disabled: false,
            trade_disabled: false,
            fixed_rate: None,
            chain: Some("GT".into()),
        };
        assert_eq!(serde_json::from_str::<Currency>(json).unwrap(), expected);
    }
}
