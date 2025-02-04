use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smallvec::SmallVec;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::PrivateRequest;
use crate::api::Request;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotAccountsRequest {
    pub currency: Option<SmartString>,
}

impl Request for SpotAccountsRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    type Response = SmallVec<[SpotAccount; 1]>;
}

impl PrivateRequest for SpotAccountsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotAccount {
    /// Currency detail
    pub currency: SmartString,
    /// Available amount
    pub available: Decimal,
    /// Locked amount, used in trading
    pub locked: Decimal,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::api::spot::SpotApi;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;

    impl<S: GateSigner> SpotApi<S> {
        /// # List spot accounts.
        ///
        /// ## Parameters
        ///
        /// * `currency` - Retrieve data of the specified currency
        pub async fn accounts(
            &self,
            currency: Option<SmartString>,
        ) -> Result<<SpotAccountsRequest as Request>::Response, RequestError> {
            let request = SpotAccountsRequest { currency };
            self.0.signed_request("/spot/accounts", &request).await
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"{
    "currency": "ETH",
    "available": "968.8",
    "locked": "0"
  }"#;
        let res: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            SpotAccount {
                currency: "ETH".into(),
                available: dec!(968.8),
                locked: dec!(0),
            }
        );
    }
}
