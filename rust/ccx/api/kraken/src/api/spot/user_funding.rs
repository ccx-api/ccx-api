use ccx_api_lib::serde_util::is_false;

use super::prelude::*;

pub const API_0_PRIVATE_DEPOSIT_METHODS: &str = "/0/private/DepositMethods";
pub const API_0_PRIVATE_DEPOSIT_ADDRESSES: &str = "/0/private/DepositAddresses";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct GetDepositMethodsRequest<'a> {
    asset: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct GetDepositMethodsResponse {
    pub deposit_methods: Vec<DepositMethod>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DepositMethod {
    /// Name of deposit method.
    pub method: Atom,
    /// Maximum net amount that can be deposited right now, or false if no limit.
    pub limit: DepositMethodLimit,
    /// Amount of fees that will be paid.
    pub fee: Option<Decimal>,
    /// Whether or not method has an address setup fee.
    #[serde(rename = "address-setup-fee")]
    pub address_setup_fee: Option<Decimal>,
    /// Whether new addresses can be generated for this method..
    #[serde(rename = "gen-address")]
    pub gen_address: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum DepositMethodLimit {
    Limited(Decimal),
    /// The value expected to be false.
    Unlimited(bool),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct GetDepositAddressesRequest<'a> {
    asset: &'a str,
    method: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct GetDepositAddressesResponse {
    pub deposit_addresses: Vec<DepositAddress>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DepositAddress {
    /// Deposit Address.
    pub address: String,
    /// Deposit Address tag.
    pub tag: Option<String>,
    /// Expiration time in unix timestamp, or 0 if not expiring.
    pub expiretm: String,
    /// Whether or not address has ever been used.
    #[serde(default, skip_serializing_if = "is_false")]
    pub new: bool,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl SpotApi {
        /// Get Deposit Methods
        ///
        /// Retrieve methods available for depositing a particular asset.
        ///
        /// * `asset` - Asset being deposited
        pub async fn get_deposit_methods(
            &self,
            nonce: Nonce,
            asset: &str,
        ) -> KrakenApiResult<GetDepositMethodsResponse> {
            self.client
                .post(API_0_PRIVATE_DEPOSIT_METHODS)?
                .signed(nonce)?
                .request_body(GetDepositMethodsRequest { asset })?
                .send()
                .await
        }

        /// Get Deposit Methods
        ///
        /// Retrieve methods available for depositing a particular asset.
        ///
        /// * `asset` - Asset being deposited
        pub async fn get_deposit_addresses(
            &self,
            nonce: Nonce,
            asset: &str,
            method: &str,
        ) -> KrakenApiResult<GetDepositAddressesResponse> {
            self.client
                .post(API_0_PRIVATE_DEPOSIT_ADDRESSES)?
                .signed(nonce)?
                .request_body(GetDepositAddressesRequest { asset, method })?
                .send()
                .await
        }
    }
}
