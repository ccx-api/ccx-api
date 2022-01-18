use super::prelude::*;

pub const SAPI_V1_BROKER_INFO: &str = "/sapi/v1/broker/info";
pub const SAPI_V1_BROKER_SUB_ACCOUNT: &str = "/sapi/v1/broker/subAccount";
pub const SAPI_V1_BROKER_SUB_ACCOUNT_API: &str = "/sapi/v1/broker/subAccountApi";
pub const SAPI_V1_BROKER_SUB_ACCOUNT_API_IP_RESTRICTION: &str =
    "/sapi/v1/broker/subAccountApi/ipRestriction";
pub const SAPI_V1_BROKER_SUB_ACCOUNT_API_IP_RESTRICTION_IP_LIST: &str =
    "/sapi/v1/broker/subAccountApi/ipRestriction/ipList";
pub const SAPI_V1_BROKER_SUB_ACCOUNT_DEPOSIT_HIST: &str = "/sapi/v1/broker/subaccount/depositHist";
pub const SAPI_V1_BROKER_SUB_ACCOUNT_SPOT_SUMMARY: &str = "/sapi/v1/broker/subaccount/spotSummary";
pub const SAPI_V1_BROKER_TRANSFER: &str = "/sapi/v1/broker/transfer";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccount {
    pub subaccount_id: String,
    pub email: String,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountApiKey {
    pub subaccount_id: String,
    pub api_key: String,
    pub secret_key: String,
    pub can_trade: bool,
    pub margin_trade: bool,
    pub futures_trade: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrokerSubaccountApiKeyDeleted {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerAccountInfo {
    pub max_maker_commission: Decimal,
    pub min_maker_commission: Decimal,
    pub max_taker_commission: Decimal,
    pub min_taker_commission: Decimal,
    pub sub_account_qty: Decimal,
    pub max_sub_account_qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountTransferCreated {
    pub txn_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_tran_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountTransfer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_id: Option<String>,
    pub asset: Atom,
    pub qty: Decimal,
    pub time: u64,
    pub txn_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_tran_id: Option<String>,
    pub status: BrokerSubaccountTransferStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum BrokerSubaccountTransferStatus {
    #[serde(rename = "INIT")]
    Init,
    #[serde(rename = "PROCESS")]
    Process,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
}

impl BrokerSubaccountTransferStatus {
    pub fn is_finished(&self) -> bool {
        use BrokerSubaccountTransferStatus as S;
        matches!(self, S::Success | S::Failure)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountDeposit {
    pub subaccount_id: String,
    pub address: String,
    pub address_tag: String,
    pub account: String,
    pub coin: String,
    pub insert_time: u64,
    pub network: String,
    pub status: BrokerSubaccountDepositStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum BrokerSubaccountDepositStatus {
    Pending = 0,
    Success = 1,
    /// Credited but cannot withdraw.
    Credited = 6,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountAssetInfoList {
    pub data: Vec<BrokerSubaccountAssetInfo>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountAssetInfo {
    pub subaccount_id: String,
    pub total_balance_of_btc: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountApiIpRestriction {
    pub subaccount_id: String,
    pub ip_restriction: bool,
    pub apikey: String,
    pub ip_list: Vec<String>,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountApiIpAddressAdded {
    pub subaccount_id: String,
    pub apikey: String,
    pub ip: String,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrokerSubaccountApiIpAddressDeleted {
    pub subaccount_id: String,
    pub apikey: String,
    pub ip_list: Vec<String>,
    pub update_time: u64,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<Signer: crate::client::BinaneSigner> SpotApi<Signer> {
        /// Create a Sub Account.
        ///
        /// * This request will generate a sub account under your brokerage master account.
        /// * You need to enable "trade" option for the api key which requests this endpoint.
        pub async fn broker_subaccount_create(
            &self,
            tag: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccount> {
            self.client
                .post(SAPI_V1_BROKER_SUB_ACCOUNT)?
                .signed(time_window)?
                .try_query_arg("tag", &tag)?
                .send()
                .await
        }

        // TODO Enable Margin for Sub Account.
        // TODO Enable Futures for Sub Account

        /// Create Api Key for Sub Account.
        ///
        /// * This request will generate a api key for a sub account.
        /// * You need to enable "trade" option for the api key which requests this endpoint.
        /// * Sub account should be enable margin before its api-key's marginTrade being enabled.
        /// * Sub account should be enable futures before its api-key's futuresTrade being enabled.
        pub async fn broker_subaccount_api_key_create(
            &self,
            subaccount_id: impl Serialize,
            can_trade: bool,
            margin_trade: Option<bool>,
            futures_trade: Option<bool>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccountApiKey> {
            self.client
                .post(SAPI_V1_BROKER_SUB_ACCOUNT_API)?
                .signed(time_window)?
                .query_arg("subAccountId", &subaccount_id)?
                .query_arg("canTrade", &can_trade)?
                .try_query_arg("marginTrade", &margin_trade)?
                .try_query_arg("futuresTrade", &futures_trade)?
                .send()
                .await
        }

        /// Delete Sub Account Api Key
        ///
        /// * This request will delete a api key for a sub account
        /// * You need to enable "trade" option for the api key which requests this endpoint
        pub async fn broker_subaccount_api_key_delete(
            &self,
            subaccount_id: impl Serialize,
            subaccount_api_key: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccountApiKeyDeleted> {
            self.client
                .delete(SAPI_V1_BROKER_SUB_ACCOUNT_API)?
                .signed(time_window)?
                .query_arg("subAccountId", &subaccount_id)?
                .query_arg("subAccountApiKey", &subaccount_api_key)?
                .send()
                .await
        }

        // TODO Query Sub Account Api Key
        // TODO Change Sub Account Api Permission
        // TODO Query Sub Account
        // TODO Change Sub Account Commission
        // TODO Change Sub Account USDT-Ⓜ Futures Commission Adjustment
        // TODO Query Sub Account USDT-Ⓜ Futures Commission Adjustment
        // TODO Change Sub Account COIN-Ⓜ Futures Commission Adjustment
        // TODO Query Sub Account COIN-Ⓜ Futures Commission Adjustment

        /// Broker Account Information
        pub async fn broker_account_info(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerAccountInfo> {
            self.client
                .get(SAPI_V1_BROKER_INFO)?
                .signed(time_window)?
                .send()
                .await
        }

        /// Sub Account Transfer（SPOT）
        ///
        /// * clientTranId - client transfer id, must be unique.
        ///     The value should be an ASCII alphanumeric.
        ///
        /// * You need to enable "internal transfer" option for the api key which requests this
        ///     endpoint.
        /// * Transfer from master account if fromId not sent.
        /// * Transfer to master account if toId not sent.
        pub async fn broker_transfer_create(
            &self,
            from_id: Option<impl Serialize>,
            to_id: Option<impl Serialize>,
            client_tran_id: Option<impl Serialize>,
            asset: impl Serialize,
            amount: Decimal,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccountTransferCreated> {
            self.client
                .post(SAPI_V1_BROKER_TRANSFER)?
                .signed(time_window)?
                .try_query_arg("fromId", &from_id)?
                .try_query_arg("toId", &to_id)?
                .try_query_arg("clientTranId", &client_tran_id)?
                .query_arg("asset", &asset)?
                .query_arg("amount", &amount)?
                .send()
                .await
        }

        /// Query Sub Account Transfer History（SPOT）
        ///
        /// * clientTranId - client transfer id
        /// * showAllStatus - default false
        /// * limit - default 500, max 500
        ///
        /// * If showAllStatus is true, the status in response will show four types:
        ///     INIT,PROCESS,SUCCESS,FAILURE.
        /// * If showAllStatus is false, the status in response will show three types:
        ///     INIT,PROCESS,SUCCESS.
        /// * Either fromId or toId must be sent. Return fromId equal master account by default.
        pub async fn broker_transfer_history(
            &self,
            from_id: Option<impl Serialize>,
            to_id: Option<impl Serialize>,
            client_tran_id: Option<impl Serialize>,
            show_all_status: Option<bool>,
            start_time: Option<u32>,
            end_time: Option<u32>,
            page: Option<u16>,
            limit: Option<u16>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Vec<BrokerSubaccountTransfer>> {
            self.client
                .get(SAPI_V1_BROKER_TRANSFER)?
                .signed(time_window)?
                .try_query_arg("fromId", &from_id)?
                .try_query_arg("toId", &to_id)?
                .try_query_arg("clientTranId", &client_tran_id)?
                .try_query_arg("showAllStatus", &show_all_status)?
                .try_query_arg("startTime", &start_time)?
                .try_query_arg("endTime", &end_time)?
                .try_query_arg("page", &page)?
                .try_query_arg("limit", &limit)?
                .send()
                .await
        }

        // TODO Sub Account Transfer（FUTURES）
        // TODO Query Sub Account Transfer History（FUTURES）
        // TODO Query Broker Commission Rebate Recent Record（Spot）
        // TODO Generate Broker Commission Rebate History（Spot）
        // TODO Query Broker Commission Rebate History（Spot）
        // TODO Enable Or Disable BNB Burn for Sub Account SPOT and MARGIN
        // TODO Enable Or Disable BNB Burn for Sub Account Margin Interest
        // TODO Get BNB Burn Status for Sub Account

        /// Get Sub Account Deposit History
        ///
        /// weight: 10
        ///
        /// * status - 0 (0: pending, 6: credited but cannot withdraw, 1: success)
        /// * startTime - Default: 7 days from current timestamp
        /// * endTime - Default: present timestamp
        /// * limit - Default：500
        /// * offset - Default：0
        ///
        /// * Please notice the default startTime and endTime to make sure that time interval
        ///     is within 0-7 days.
        /// * If both startTime and endTime are sent, time between startTime and endTime must be
        ///     less than 7 days.
        pub async fn broker_subaccount_deposit_history(
            &self,
            sub_account_id: Option<impl Serialize>,
            coin: Option<impl Serialize>,
            status: Option<BrokerSubaccountDepositStatus>,
            start_time: Option<u32>,
            end_time: Option<u32>,
            limit: Option<u16>,
            offset: Option<u16>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Vec<BrokerSubaccountDeposit>> {
            self.client
                .get(SAPI_V1_BROKER_SUB_ACCOUNT_DEPOSIT_HIST)?
                .signed(time_window)?
                .try_query_arg("subAccountId", &sub_account_id)?
                .try_query_arg("coin", &coin)?
                .try_query_arg("status", &status)?
                .try_query_arg("startTime", &start_time)?
                .try_query_arg("endTime", &end_time)?
                .try_query_arg("limit", &limit)?
                .try_query_arg("offset", &offset)?
                .send()
                .await
        }

        /// Query Sub Account Spot Asset info
        ///
        /// * page - default 1
        /// * size - default 10, max 20
        ///
        /// * If subAccountId is not sent, the size must be sent.
        pub async fn broker_subaccount_asset_info(
            &self,
            sub_account_id: Option<impl Serialize>,
            page: Option<u16>,
            size: Option<u16>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccountAssetInfoList> {
            self.client
                .get(SAPI_V1_BROKER_SUB_ACCOUNT_SPOT_SUMMARY)?
                .signed(time_window)?
                .try_query_arg("subAccountId", &sub_account_id)?
                .try_query_arg("page", &page)?
                .try_query_arg("size", &size)?
                .send()
                .await
        }

        // TODO Query Subaccount Margin Asset info
        // TODO Query Subaccount Futures Asset info
        // TODO Query Subaccount Futures Asset info (V2)
        // TODO Enable Leverage Token for Sub Account
        // TODO Query Broker Futures Commission Rebate Record

        /// Get IP Restriction for Sub Account Api Key
        pub async fn broker_subaccount_api_ip_restriction_get(
            &self,
            sub_account_id: impl Serialize,
            sub_account_api_key: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccountApiIpRestriction> {
            self.client
                .get(SAPI_V1_BROKER_SUB_ACCOUNT_API_IP_RESTRICTION)?
                .signed(time_window)?
                .query_arg("subAccountId", &sub_account_id)?
                .query_arg("subAccountApiKey", &sub_account_api_key)?
                .send()
                .await
        }

        /// Enable or Disable IP Restriction for Sub Account Api Key
        pub async fn broker_subaccount_api_ip_restriction_set(
            &self,
            sub_account_id: impl Serialize,
            sub_account_api_key: impl Serialize,
            ip_restrict: bool,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccountApiIpRestriction> {
            self.client
                .post(SAPI_V1_BROKER_SUB_ACCOUNT_API_IP_RESTRICTION)?
                .signed(time_window)?
                .query_arg("subAccountId", &sub_account_id)?
                .query_arg("subAccountApiKey", &sub_account_api_key)?
                .query_arg("ipRestrict", &ip_restrict)?
                .send()
                .await
        }

        /// Add IP Restriction for Sub Account Api Key
        pub async fn broker_subaccount_api_ip_address_add(
            &self,
            sub_account_id: impl Serialize,
            sub_account_api_key: impl Serialize,
            ip_address: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccountApiIpAddressAdded> {
            self.client
                .post(SAPI_V1_BROKER_SUB_ACCOUNT_API_IP_RESTRICTION_IP_LIST)?
                .signed(time_window)?
                .query_arg("subAccountId", &sub_account_id)?
                .query_arg("subAccountApiKey", &sub_account_api_key)?
                .query_arg("ipAddress", &ip_address)?
                .send()
                .await
        }

        /// Delete IP Restriction for Sub Account Api Key
        pub async fn broker_subaccount_api_ip_address_delete(
            &self,
            sub_account_id: impl Serialize,
            sub_account_api_key: impl Serialize,
            ip_address: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<BrokerSubaccountApiIpAddressDeleted> {
            self.client
                .delete(SAPI_V1_BROKER_SUB_ACCOUNT_API_IP_RESTRICTION_IP_LIST)?
                .signed(time_window)?
                .query_arg("subAccountId", &sub_account_id)?
                .query_arg("subAccountApiKey", &sub_account_api_key)?
                .query_arg("ipAddress", &ip_address)?
                .send()
                .await
        }

        // TODO Universal Transfer
        // TODO Query Universal Transfer History
        // TODO Enable Universal Transfer Permission For Sub Account Api Key
        // TODO Enable Vanilla Options Permission For Sub Account Api Key
    }
}
