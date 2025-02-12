use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::PrivateRequest;
use crate::api::Request;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletDepositAddressRequest {
    pub currency: SmartString,
}

impl Request for WalletDepositAddressRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;

    type Response = WalletDepositAddressResponse;
}

impl PrivateRequest for WalletDepositAddressRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletDepositAddressResponse {
    pub currency: SmartString,
    pub address: SmartString,
    pub multichain_addresses: Vec<WalletDepositAddressMultichainAddress>,
    pub min_deposit_amount: SmartString,
    pub min_confirms: Option<SmartString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletDepositAddressMultichainAddress {
    pub chain: SmartString,
    pub address: SmartString,
    pub payment_id: SmartString,
    pub payment_name: SmartString,
    pub obtain_failed: u32,
    pub min_confirms: Option<u32>,
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
        pub async fn deposit_address(
            &self,
            currency: SmartString,
        ) -> Result<<WalletDepositAddressRequest as Request>::Response, RequestError> {
            self.0
                .signed_request(
                    "/wallet/deposit_address",
                    &WalletDepositAddressRequest { currency },
                )
                .await
        }
    }
}
