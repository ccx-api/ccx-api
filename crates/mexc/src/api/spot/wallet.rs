use super::RL_WEIGHT_PER_MINUTE;
use super::prelude::*;
use crate::client::Task;

pub const API_V3_CAPITAL_CONFIG_GETALL: &str = "/api/v3/capital/config/getall";
pub const API_V3_CAPITAL_WITHDRAW: &str = "/api/v3/capital/withdraw";
pub const API_V3_CAPITAL_DEPOSIT_HISTORY: &str = "/api/v3/capital/deposit/hisrec";
pub const API_V3_CAPITAL_WITHDRAW_HISTORY: &str = "/api/v3/capital/withdraw/history";
pub const API_V3_CAPITAL_DEPOSIT_ADDRESS: &str = "/api/v3/capital/deposit/address";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInformation {
    pub coin: Atom,
    pub name: Atom,
    pub network_list: Vec<NetworkInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInformation {
    pub coin: Atom,
    /// Shown only when "depositEnable" is false.
    pub deposit_desc: Option<Atom>,
    pub deposit_enable: bool,
    pub deposit_tips: Option<Atom>,
    /// Min number for balance confirmation.
    pub min_confirm: i32,
    pub name: Atom,
    #[serde(rename = "netWork")]
    pub network: Atom,
    /// Shown only when "withdrawEnable" is false.
    pub withdraw_desc: Option<Atom>,
    pub withdraw_enable: bool,
    pub withdraw_fee: Decimal,
    pub withdraw_min: Decimal,
    pub withdraw_max: String,
    pub same_address: bool,
    pub contract: Option<Atom>,
    pub withdraw_tip: Option<Atom>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    pub amount: Decimal,
    pub coin: String,
    pub network: Atom,
    pub status: DepositStatus,
    pub address: String,
    pub tx_id: String,
    pub insert_time: u64,
    pub confirm_times: String,
    pub memo: Option<String>,
    pub unlock_confirm: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub address: String,
    pub coin: Atom,
    pub memo: String,
    pub network: Atom,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u8)]
pub enum DepositStatus {
    Small = 1,
    TimeDelay = 2,
    LargeDelay = 3,
    Pending = 4,
    Success = 5,
    Auditing = 6,
    Rejected = 7,
}

impl DepositStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, DepositStatus::Success)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, DepositStatus::Pending)
    }

    pub fn is_processing(&self) -> bool {
        matches!(self, DepositStatus::Pending)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewWithdraw {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    pub coin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub address: String,
    pub amount: Decimal,
    pub transfer_type: TransferType,
    pub status: WithdrawStatus,
    #[serde(default)]
    pub transaction_fee: Decimal,
    pub apply_time: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trans_hash: Option<String>,
    pub update_time: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcoin_id: Option<String>,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum WithdrawStatus {
    Apply = 1,
    Auditing = 2,
    Wait = 3,
    Processing = 4,
    WaitPackaging = 5,
    WaitConfirm = 6,
    Success = 7,
    Failed = 8,
    Cancel = 9,
    Manual = 10,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum TransferType {
    External = 0,
    Internal = 1,
}

impl WithdrawStatus {
    pub fn is_finished(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(self, WS::Success | WS::Failed | WS::Cancel)
    }

    pub fn is_pending(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(
            self,
            WS::Wait | WS::WaitPackaging | WS::WaitConfirm | WS::Processing
        )
    }

    pub fn needs_confirmation(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(self, WS::WaitConfirm)
    }
}

impl TransferType {
    pub fn is_external(&self) -> bool {
        matches!(self, TransferType::External)
    }

    pub fn is_internal(&self) -> bool {
        matches!(self, TransferType::Internal)
    }
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S> SpotApi<S>
    where
        S: crate::client::MexcSigner,
        S: Unpin + 'static,
    {
        /// All Coins' Information (USER_DATA)
        ///
        /// Get information of coins (available for deposit and withdraw) for user.
        ///
        /// Weight(IP): 10
        pub fn all_coins_information(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<Vec<CoinInformation>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_CAPITAL_CONFIG_GETALL)?
                        .signed(time_window)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        /// Deposit Address (supporting network) (USER_DATA)
        ///
        /// Fetch deposit address with network.
        ///
        /// Weight(IP): 1
        ///
        /// If network is not send, return with default network of the coin.
        /// You can get network and isDefault in networkList in the response of
        ///    Get /api/v3/capital/config/getall (HMAC SHA256).
        pub fn get_deposit_address(
            &self,
            coin: impl Serialize,
            network: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<Vec<DepositAddress>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_CAPITAL_DEPOSIT_ADDRESS)?
                        .signed(time_window)?
                        .query_arg("coin", &coin)?
                        .query_arg("network", &network)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Withdraw(API)
        ///
        /// Submit a withdraw request.
        ///
        /// Weight(IP): 1
        ///
        /// * withdrawOrderId - client id for withdraw
        ///
        /// If network is not send, return with default network of the coin.
        /// You can get network and isDefault in networkList in the response of
        ///    Get /api/v3/capital/config/getall (HMAC SHA256).
        #[allow(clippy::too_many_arguments)]
        pub fn withdraw(
            &self,
            coin: impl Serialize,
            withdraw_order_id: Option<impl Serialize>,
            network: Option<impl Serialize>,
            contract_address: Option<impl Serialize>,
            address: impl Serialize,
            memo: Option<impl Serialize>,
            amount: Decimal,
            remark: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<NewWithdraw>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_V3_CAPITAL_WITHDRAW)?
                        .signed(time_window)?
                        .query_arg("coin", &coin)?
                        .try_query_arg("withdrawOrderId", &withdraw_order_id)?
                        .try_query_arg("netWork", &network)?
                        .query_arg("address", &address)?
                        .try_query_arg("contractAddress", &contract_address)?
                        .query_arg("amount", &amount)?
                        .try_query_arg("memo", &memo)?
                        .try_query_arg("remark", &remark)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Deposit History (supporting network) (USER_DATA)
        ///
        /// Fetch deposit history.
        ///
        /// Weight(IP): 1
        ///
        /// * startTime - Default: 7 days from current timestamp
        /// * endTime - Default: present timestamp
        ///
        /// * network may not be in the response for old deposit.
        /// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
        /// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
        #[allow(clippy::too_many_arguments)]
        pub fn deposit_history(
            &self,
            coin: Option<impl Serialize>,
            status: Option<DepositStatus>,
            limit: Option<u16>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<Vec<Deposit>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_CAPITAL_DEPOSIT_HISTORY)?
                        .signed(time_window)?
                        .try_query_arg("coin", &coin)?
                        .try_query_arg("status", &status)?
                        .try_query_arg("limit", &limit)?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Withdraw History (supporting network) (USER_DATA)
        ///
        /// Fetch withdraw history.
        ///
        /// Weight(IP): 1
        ///
        /// * startTime - Default: 7 days from current timestamp
        /// * endTime - Default: present timestamp
        ///
        /// * network may not be in the response for old withdraw.
        /// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
        /// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
        #[allow(clippy::too_many_arguments)]
        pub fn withdraw_history(
            &self,
            coin: Option<impl Serialize>,
            status: Option<WithdrawStatus>,
            limit: Option<u16>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> MexcResult<Task<Vec<Withdraw>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(API_V3_CAPITAL_WITHDRAW_HISTORY)?
                        .signed(time_window)?
                        .try_query_arg("coin", &coin)?
                        .try_query_arg("status", &status)?
                        .try_query_arg("limit", &limit)?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }
    }
}
