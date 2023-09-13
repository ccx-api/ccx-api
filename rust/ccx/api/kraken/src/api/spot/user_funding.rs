use ccx_api_lib::serde_util::is_false;

use super::prelude::*;
use crate::client::Task;

use super::RL_PRIVATE_PER_MINUTE;

pub const API_0_PRIVATE_DEPOSIT_METHODS: &str = "/0/private/DepositMethods";
pub const API_0_PRIVATE_DEPOSIT_ADDRESSES: &str = "/0/private/DepositAddresses";
pub const API_0_PRIVATE_DEPOSIT_STATUS: &str = "/0/private/DepositStatus";
pub const API_0_PRIVATE_WITHDRAW_INFO: &str = "/0/private/WithdrawInfo";
pub const API_0_PRIVATE_WITHDRAW: &str = "/0/private/Withdraw";
pub const API_0_PRIVATE_WITHDRAW_STATUS: &str = "/0/private/WithdrawStatus";
pub const API_0_PRIVATE_WITHDRAW_CANCEL: &str = "/0/private/WithdrawCancel";
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct GetStatusOfRecentDepositsRequest<'a> {
    asset: Option<&'a str>,
    method: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct GetStatusOfRecentDepositsResponse {
    pub deposits: Vec<Deposit>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Deposit {
    /// Name of deposit method.
    pub method: String,

    /// Asset class.
    pub aclass: String,

    /// Asset.
    pub asset: String,

    /// Reference ID.
    pub refid: String,

    /// Method transaction ID.
    pub txid: String,

    /// Method transaction information.
    pub info: String,

    /// Amount deposited.
    pub amount: Decimal,

    /// Fees paid (if any).
    #[serde(default)]
    pub fee: Option<Decimal>,

    /// Unix timestamp when request made.
    pub time: i64,

    /// Status of deposit.
    pub status: DepositStatus,

    /// Addition status properties (if available)
    #[serde(default, rename = "status-prop")]
    pub status_prop: Option<DepositStatusProperties>,

    /// Client sending transaction id(s) for deposits that credit with a
    /// sweeping transaction
    pub originators: Vec<String>,
}

/// Deposit status according to [IFEX financial transaction states][1].
///
/// [1]: https://github.com/globalcitizen/ifex-protocol/blob/master/draft-ifex-00.txt#L837
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DepositStatus {
    Initial,
    Pending,
    Settled,
    Success,
    Failure,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DepositStatusProperties {
    /// A return transaction initiated by Kraken; it cannot be canceled.
    #[serde(rename = "return")]
    Return,

    /// Deposit is on hold pending review
    #[serde(rename = "onhold")]
    OnHold,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GetWithdrawalInformationRequest<'a> {
    asset: &'a str,
    key: &'a str,
    amount: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct GetWithdrawalInformationResponse(pub WithdrawInfo);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WithdrawFundsRequest<'a> {
    asset: &'a str,
    key: &'a str,
    amount: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WithdrawFundsResponse {
    /// Reference ID of the withdraw.
    pub refid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GetStatusOfRecentWithdrawalsRequest<'a> {
    asset: &'a str,
    method: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct GetStatusOfRecentWithdrawalsResponse(pub Vec<Withdraw>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WithdrawalCancelationRequest<'a> {
    asset: &'a str,
    refid: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct WithdrawalCancelationResponse(pub bool);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WithdrawInfo {
    /// Name of the withdrawal method that will be used.
    pub method: String,

    /// Maximum net amount that can be withdrawn right now.
    pub limit: String,

    /// Net amount that will be sent, after fees.
    pub amount: String,

    /// Amount of fees that will be paid
    pub fee: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Withdraw {
    /// Name og withdrawal method.
    pub method: String,

    /// Asset class.
    pub aclass: String,

    /// Asset being withdrawn.
    pub asset: String,

    /// Reference ID
    pub refid: String,

    /// Method transaction ID.
    pub txid: Option<String>,

    /// Method transaction information.
    pub info: String,

    /// Amount withdrawn.
    pub amount: String,

    /// Fees paid.
    pub fee: String,

    /// Unix timestamp when request was made.
    pub time: i64,

    /// Status of withdraw.
    pub status: WithdrawStatus,

    /// Additional status properties.
    pub status_prop: Option<WithdrawStatusProperties>,
}

/// Withdrawal status according to [IFEX financial transaction states][1].
///
/// [1]: https://github.com/globalcitizen/ifex-protocol/blob/master/draft-ifex-00.txt#L837
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum WithdrawStatus {
    Initial,
    Pending,
    Settled,
    Success,
    Failure,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum WithdrawStatusProperties {
    /// Cancelation requested.
    #[serde(rename = "cancel-pending")]
    CancelPending,

    /// Withdraw was canceled.
    #[serde(rename = "canceled")]
    Canceled,

    /// Cancelation requested but was denied.
    #[serde(rename = "cancel-denied")]
    CancelDenied,

    /// A return transaction initiated by Kraken; it cannot be canceled.
    #[serde(rename = "return")]
    Return,

    /// Withdrawal is on hold pending review
    #[serde(rename = "onhold")]
    OnHold,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S> SpotApi<S>
    where
        S: crate::client::KrakenSigner,
        S: Unpin + 'static,
    {
        /// Get Deposit Methods
        ///
        /// Retrieve methods available for depositing a particular asset.
        ///
        /// * `asset` - Asset being deposited
        pub fn get_deposit_methods(
            &self,
            nonce: Nonce,
            asset: &str,
        ) -> KrakenResult<Task<GetDepositMethodsResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_DEPOSIT_METHODS)?
                        .signed(nonce)?
                        .request_body(GetDepositMethodsRequest { asset })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .send())
        }

        /// Get Deposit Methods
        ///
        /// Retrieve methods available for depositing a particular asset.
        ///
        /// * `asset` - Asset being deposited
        pub fn get_deposit_addresses(
            &self,
            nonce: Nonce,
            asset: &str,
            method: &str,
        ) -> KrakenResult<Task<GetDepositAddressesResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_DEPOSIT_ADDRESSES)?
                        .signed(nonce)?
                        .request_body(GetDepositAddressesRequest { asset, method })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .send())
        }

        /// Get Status of Recent Deposits
        ///
        /// Retrieve information about recent deposits. Any deposits initiated
        /// in the past 90 days will be included in the response, up to a
        /// maximum of 25 results, sorted by recency.
        ///
        /// * `asset` - Filter for specific asset being deposited
        /// * `method` - Filter for specific name of deposit method
        pub fn get_status_of_recent_deposits(
            &self,
            nonce: Nonce,
            asset: Option<&str>,
            method: Option<&str>,
        ) -> KrakenResult<Task<()>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_DEPOSIT_STATUS)?
                        .signed(nonce)?
                        .request_body(GetStatusOfRecentDepositsRequest { asset, method })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .send())
        }

        /// Get Withdrawal Information
        ///
        /// Retrieve fee information about potential withdrawals for a
        /// particular asset, key and amount.
        ///
        /// * `asset` - Asset being withdrawn
        /// * `key` - Withdrawal key name, as set up on your account
        /// * `amount` - Amount to be withdrawn
        pub fn get_withdrawal_information(
            &self,
            nonce: Nonce,
            asset: &str,
            key: &str,
            amount: &str,
        ) -> KrakenResult<Task<GetWithdrawalInformationResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_WITHDRAW_INFO)?
                        .signed(nonce)?
                        .request_body(GetWithdrawalInformationRequest { asset, key, amount })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .send())
        }

        /// Withdraw Funds
        ///
        /// Make a withdrawal request.
        ///
        /// * `asset` - Asset being withdrawn
        /// * `key` - Withdrawal key name, as set up on your account
        /// * `amount` - Amount to be withdrawn
        pub fn withdraw_funds(
            &self,
            nonce: Nonce,
            asset: &str,
            key: &str,
            amount: &str,
        ) -> KrakenResult<Task<WithdrawFundsResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_WITHDRAW)?
                        .signed(nonce)?
                        .request_body(WithdrawFundsRequest { asset, key, amount })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .send())
        }

        /// Get Status of Recent Withdrawals.
        ///
        /// Retrieve information about recently requests withdrawals.
        ///
        /// * `asset` - Asset being withdrawn.
        /// * `method` - Name of the withdrawal method.
        pub fn get_status_of_recent_withdrawals(
            &self,
            nonce: Nonce,
            asset: &str,
            method: Option<&str>,
        ) -> KrakenResult<Task<GetStatusOfRecentWithdrawalsResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_WITHDRAW_STATUS)?
                        .signed(nonce)?
                        .request_body(GetStatusOfRecentWithdrawalsRequest { asset, method })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .send())
        }

        /// Request Withdrawal Cancelation
        ///
        /// Cancel a recently requested withdrawal, if it has not already been
        /// successfully processed.
        ///
        /// * `asset` - Asset being withdrawn.
        /// * `method` - Name of the withdrawal method.
        pub fn request_withdrawal_cancelation(
            &self,
            nonce: Nonce,
            asset: &str,
            refid: &str,
        ) -> KrakenResult<Task<WithdrawalCancelationResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_WITHDRAW_CANCEL)?
                        .signed(nonce)?
                        .request_body(WithdrawalCancelationRequest { asset, refid })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .send())
        }
    }
}
