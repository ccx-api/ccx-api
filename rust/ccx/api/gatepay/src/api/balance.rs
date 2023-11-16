use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::Request;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceRequest;

impl Request for BalanceRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V1;
    const PATH: &'static str = "pay/balance/query";
    type Response = BalancesResponse;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalancesResponse {
    pub balance_list: Vec<BalanceItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceItem {
    /// The currency of the balance.
    pub currency: String,
    /// The balance of the currency in the merchant's spot account. The balance is truncated
    /// to six decimal places, rounded down, and trailing zeros are omitted.
    pub available: Decimal,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::rest::RequestError;
    use crate::client::signer::GatepaySigner;
    use crate::MerchantApi;

    impl<S: GatepaySigner> MerchantApi<S> {
        /// Query merchant balance
        pub async fn balance(&self) -> Result<BalancesResponse, RequestError> {
            self.request(&BalanceRequest).await
        }
    }
}
