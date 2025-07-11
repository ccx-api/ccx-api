use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::PrivateRequest;
use crate::api::Request;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletDepositsRequest {
    pub currency: Option<SmartString>,
    pub from: Option<SmartString>,
    pub to: Option<SmartString>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Request for WalletDepositsRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;

    type Response = Vec<WalletDepositsResponse>;
}

impl PrivateRequest for WalletDepositsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletDepositsResponse {
    pub id: SmartString,
    pub timestamp: SmartString,
    pub withdraw_order_id: Option<SmartString>,
    pub currency: SmartString,
    pub address: SmartString,
    pub txid: SmartString,
    pub amount: SmartString,
    pub memo: SmartString,
    pub status: WalletDepositsStatus,
    pub chain: SmartString,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletDepositsStatus {
    /// Recharge review (compliance review)
    Review,
    /// Processing
    Pend,
    /// Waiting for funds to be unlocked
    Done,
    /// Invalid data
    Invalid,
    /// Track the number of confirmations, waiting to add funds to the user (spot)
    Track,
    /// Rejected Recharge
    Blocked,
    /// Recharge to account, withdrawal is not unlocked
    DepCredited,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::api::wallet::WalletApi;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;

    impl<S: GateSigner> WalletApi<S> {
        /// # Generate currency deposit address
        ///
        /// Generate currency deposit address
        ///
        /// ## Parameters
        ///
        /// * `currency` - Currency name
        pub async fn deposits(
            &self,
            currency: Option<SmartString>,
            from: Option<SmartString>,
            to: Option<SmartString>,
            limit: Option<u32>,
            offset: Option<u32>,
        ) -> Result<<WalletDepositsRequest as Request>::Response, RequestError> {
            self.0
                .signed_request(
                    "/wallet/deposits",
                    &WalletDepositsRequest {
                        currency,
                        from,
                        to,
                        limit,
                        offset,
                    },
                )
                .await
        }
    }
}
