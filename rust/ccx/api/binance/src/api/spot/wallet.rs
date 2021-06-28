use super::prelude::*;

pub const SAPI_V1_SYSTEM_STATUS: &str = "/sapi/v1/system/status";
pub const SAPI_V1_CAPITAL_CONFIG_GETALL: &str = "/sapi/v1/capital/config/getall";
// TODO pub const SAPI_V1_ACCOUNT_SNAPSHOT: &str = "/sapi/v1/accountSnapshot";
pub const SAPI_V1_ACCOUNT_DISABLE_FAST_WITHDRAW: &str = "/sapi/v1/account/disableFastWithdrawSwitch";
pub const SAPI_V1_ACCOUNT_ENABLE_FAST_WITHDRAW: &str = "/sapi/v1/account/enableFastWithdrawSwitch";
pub const SAPI_V1_CAPITAL_WITHDRAW_APPLY: &str = "/sapi/v1/capital/withdraw/apply";
// TODO pub const SAPI_V1_CAPITAL_DEPOSIT_HISTORY: &str = "/sapi/v1/capital/deposit/history";
pub const SAPI_V1_CAPITAL_WITHDRAW_HISTORY: &str = "/sapi/v1/capital/withdraw/history";
pub const SAPI_V1_CAPITAL_DEPOSIT_ADDRESS: &str = "/sapi/v1/capital/deposit/address";
// TODO pub const SAPI_V1_ACCOUNT_STATUS: &str = "/sapi/v1/account/status";
// TODO pub const SAPI_V1_ACCOUNT_TRADING_STATUS: &str = "/sapi/v1/account/apiTradingStatus";
// TODO pub const SAPI_V1_ASSET_DRIBLET: &str = "/sapi/v1/asset/dribblet";
// TODO pub const SAPI_V1_ASSET_DUST: &str = "/sapi/v1/asset/dust";
// TODO pub const SAPI_V1_ASSET_DIVIDEND: &str = "/sapi/v1/asset/assetDividend";
// TODO pub const SAPI_V1_ASSET_DETAIL: &str = "/sapi/v1/asset/assetDetail";
// TODO pub const SAPI_V1_ASSET_TRADE_FEE: &str = "/sapi/v1/asset/tradeFee";
pub const SAPI_V1_ASSET_TRANSFER: &str = "/sapi/v1/asset/transfer";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    pub status: SystemMaintenanceStatus,
    pub msg: String,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum SystemMaintenanceStatus {
    Normal = 0,
    SystemMaintenance = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInformation {
    pub coin: Atom,
    pub deposit_all_enable: bool,
    pub free: Decimal,
    pub freeze: Decimal,
    pub ipoable: Decimal,
    pub ipoing: Decimal,
    pub is_legal_money: bool,
    pub locked: Decimal,
    pub name: Atom,
    pub network_list: Vec<NetworkInformation>,
    pub storage: Decimal,
    pub trading: bool,
    pub withdraw_all_enable: bool,
    pub withdrawing: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInformation {
    pub address_regex: Atom,
    pub coin: Atom,
    /// Shown only when "depositEnable" is false.
    pub deposit_desc: Option<Atom>,
    pub deposit_enable: bool,
    pub insert_time: Option<u64>,
    pub is_default: bool,
    pub memo_regex: Atom,
    /// Min number for balance confirmation.
    pub min_confirm: i32,
    pub name: Atom,
    pub network: Atom,
    pub reset_address_status: bool,
    pub special_tips: Option<Atom>,
    /// Confirmation number for balance unlock.
    pub un_lock_confirm: i32,
    pub update_time: Option<u64>,
    /// Shown only when "withdrawEnable" is false.
    pub withdraw_desc: Option<Atom>,
    pub withdraw_enable: bool,
    pub withdraw_fee: Decimal,
    pub withdraw_min: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub address: String,
    pub coin: Atom,
    pub tag: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewWithdraw {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    pub address: String,
    pub amount: Decimal,
    // FIXME decode time, example: "2021-04-29 16:08:00"
    pub apply_time: String,
    pub coin: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub transfer_type: TransferType,
    pub status: WithdrawStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum WithdrawStatus {
    EmailSent = 0,
    Cancelled = 1,
    AwaitingApproval = 2,
    Rejected = 3,
    Processing = 4,
    Failure = 5,
    Completed = 6,
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
        matches!(
            self,
            WS::Completed | WS::Cancelled | WS::Rejected | WS::Failure
        )
    }

    pub fn is_pending(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(self, WS::EmailSent | WS::AwaitingApproval | WS::Processing)
    }

    pub fn needs_confirmation(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(self, WS::EmailSent)
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfer {
    #[serde(rename = "tranId")]
    transfer_id: u64,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl SpotApi {

        pub async fn asset_transfer(
            &self,
            transfer_type: TransferKind,
            asset: impl Serialize,
            amount: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> LibResult<Transfer> {
            self.client
                .post(SAPI_V1_ASSET_TRANSFER)?
                .signed(time_window)?
                .query_arg("type", &transfer_type)?
                .query_arg("asset", &asset)?
                .query_arg("amount", &amount)?
                .send()
                .await
        }

        /// System Status (System)
        ///
        /// Fetch system status.
        pub async fn system_status(
            &self,
        ) -> LibResult<SystemStatus> {
            self.client
                .get(SAPI_V1_ACCOUNT_ENABLE_FAST_WITHDRAW)?
                .send()
                .await
        }

        /// All Coins' Information (USER_DATA)
        ///
        /// Get information of coins (available for deposit and withdraw) for user.
        ///
        /// Weight: 1
        pub async fn all_coins_information(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> LibResult<Vec<CoinInformation>> {
            self.client
                .get(SAPI_V1_CAPITAL_CONFIG_GETALL)?
                .signed(time_window)?
                .send()
                .await
        }

        /// Disable Fast Withdraw Switch (USER_DATA)
        ///
        /// Weight: 1
        ///
        /// Caution:
        ///
        /// * This request will disable fastwithdraw switch under your account.
        /// * You need to enable "trade" option for the api key which requests this endpoint.
        pub async fn disable_fast_withdraw_switch(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> LibResult<()> {
            self.client
                .post(SAPI_V1_ACCOUNT_DISABLE_FAST_WITHDRAW)?
                .signed(time_window)?
                .send_no_responce()
                .await
        }

        /// Enable Fast Withdraw Switch (USER_DATA)
        ///
        /// Weight: 1
        ///
        /// This request will enable fastwithdraw switch under your account.
        /// You need to enable "trade" option for the api key which requests this endpoint.
        /// When Fast Withdraw Switch is on, transferring funds to a Binance account will be done
        ///   instantly. There is no on-chain transaction, no transaction ID and no withdrawal fee.
        pub async fn enable_fast_withdraw_switch(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> LibResult<()> {
            self.client
                .post(SAPI_V1_ACCOUNT_ENABLE_FAST_WITHDRAW)?
                .signed(time_window)?
                .send_no_responce()
                .await
        }

        /// Deposit Address (supporting network) (USER_DATA)
        ///
        /// Fetch deposit address with network.
        ///
        /// Weight: 1
        ///
        /// If network is not send, return with default network of the coin.
        /// You can get network and isDefault in networkList in the response of
        ///    Get /sapi/v1/capital/config/getall (HMAC SHA256).
        pub async fn get_deposit_address(
            &self,
            coin: impl Serialize,
            network: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> LibResult<DepositAddress> {
            self.client
                .get(SAPI_V1_CAPITAL_DEPOSIT_ADDRESS)?
                .signed(time_window)?
                .query_arg("coin", &coin)?
                .try_query_arg("network", &network)?
                .send()
                .await
        }

        /// Withdraw(SAPI)
        ///
        /// Submit a withdraw request.
        ///
        /// Weight: 1
        ///
        /// * withdrawOrderId - client id for withdraw
        /// * addressTag - Secondary address identifier for coins like XRP,XMR etc.
        /// * transactionFeeFlag - When making internal transfer, true for returning the fee
        ///     to the destination account; false for returning the fee back to the departure account.
        ///     Default false.
        /// * name - Description of the address. Space in name should be encoded into %20.
        ///
        /// If network is not send, return with default network of the coin.
        /// You can get network and isDefault in networkList in the response of
        ///    Get /sapi/v1/capital/config/getall (HMAC SHA256).
        pub async fn withdraw(
            &self,
            coin: impl Serialize,
            withdraw_order_id: Option<impl Serialize>,
            network: Option<impl Serialize>,
            address: impl Serialize,
            address_tag: Option<impl Serialize>,
            amount: Decimal,
            transaction_fee_flag: Option<bool>,
            name: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> LibResult<NewWithdraw> {
            self.client
                .post(SAPI_V1_CAPITAL_WITHDRAW_APPLY)?
                .signed(time_window)?
                .query_arg("coin", &coin)?
                .try_query_arg("withdrawOrderId", &withdraw_order_id)?
                .try_query_arg("network", &network)?
                .query_arg("address", &address)?
                .try_query_arg("addressTag", &address_tag)?
                .query_arg("amount", &amount)?
                .try_query_arg("transactionFeeFlag", &transaction_fee_flag)?
                .try_query_arg("name", &name)?
                .send()
                .await
        }

        /// Withdraw History (supporting network) (USER_DATA)
        ///
        /// Fetch withdraw history.
        ///
        /// Weight: 1
        ///
        /// * startTime - Default: 90 days from current timestamp
        /// * endTime - Default: present timestamp
        ///
        /// * network may not be in the response for old withdraw.
        /// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
        /// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
        pub async fn withdraw_history(
            &self,
            coin: Option<impl Serialize>,
            status: Option<WithdrawStatus>,
            offset: Option<u16>,
            limit: Option<u16>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> LibResult<Vec<Withdraw>> {
            self.client
                .get(SAPI_V1_CAPITAL_WITHDRAW_HISTORY)?
                .signed(time_window)?
                .try_query_arg("coin", &coin)?
                .try_query_arg("status", &status)?
                .try_query_arg("offset", &offset)?
                .try_query_arg("limit", &limit)?
                .try_query_arg("startTime", &start_time)?
                .try_query_arg("endTime", &end_time)?
                .send()
                .await
        }
    }
}
