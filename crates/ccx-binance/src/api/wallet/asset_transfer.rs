use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::rate_limits::RateLimitType;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TransferKind {
    #[serde(rename = "CMFUTURE_FUNDING")]
    CmFutureFunding, // COIN-M Futures account transfer to Funding account
    #[serde(rename = "CMFUTURE_MAIN")]
    CmFutureMain, // COIN-M Futures account transfer to Spot account
    #[serde(rename = "CMFUTURE_MARGIN")]
    CmFutureMargin, // COIN-M Futures account transfer to Margin(cross) account

    #[serde(rename = "FUNDING_CMFUTURE")]
    FundingCmFuture, // Funding account transfer to COIN-M Futures account
    #[serde(rename = "FUNDING_MAIN")]
    FundingMain, // Funding account transfer to Spot account
    #[serde(rename = "FUNDING_MARGIN")]
    FundingMargin, // Funding account transfer to Margin (cross) account
    #[serde(rename = "FUNDING_UMFUTURE")]
    FundingUmFuture, // Funding account transfer to USDⓈ-M Futures account

    #[serde(rename = "MAIN_CMFUTURE")]
    MainCmFuture, // Spot account transfer to COIN-M Futures account
    #[serde(rename = "MAIN_FUNDING")]
    MainFunding, // Spot account transfer to Funding account
    #[serde(rename = "MAIN_MARGIN")]
    MainMargin, // Spot account transfer to Margin（cross）account
    #[serde(rename = "MAIN_MINING")]
    MainMining, // Spot account transfer to Mining account
    #[serde(rename = "MAIN_UMFUTURE")]
    MainUmFuture, // Spot account transfer to USDⓈ-M Futures account

    #[serde(rename = "MARGIN_MAIN")]
    MarginMain, // Margin（cross）account transfer to Spot account
    #[serde(rename = "MARGIN_CMFUTURE")]
    MarginCmFuture, // Margin（cross）account transfer to COIN-M Futures
    #[serde(rename = "MARGIN_FUNDING")]
    MarginFunding, // Margin（cross）account transfer to Funding account
    #[serde(rename = "MARGIN_MINING")]
    MarginMining, // Margin（cross）account transfer to Mining account
    #[serde(rename = "MARGIN_UMFUTURE")]
    MarginUmFuture, // Margin（cross）account transfer to USDⓈ-M Futures

    #[serde(rename = "MINING_MAIN")]
    MiningMain, // Mining account transfer to Spot account
    #[serde(rename = "MINING_UMFUTURE")]
    MiningUmFuture, // Mining account transfer to USDⓈ-M Futures account
    #[serde(rename = "MINING_MARGIN")]
    MiningMargin, // Mining account transfer to Margin(cross) account

    #[serde(rename = "UMFUTURE_FUNDING")]
    UmFutureFunding, // USDⓈ-M Futures account transfer to Funding account
    #[serde(rename = "UMFUTURE_MAIN")]
    UmFutureMain, // USDⓈ-M Futures account transfer to Spot account
    #[serde(rename = "UMFUTURE_MARGIN")]
    UmFutureMargin, // USDⓈ-M Futures account transfer to Margin（cross）account
}

/// [User Universal Transfer](https://developers.binance.com/docs/wallet/asset/user-universal-transfer)
///
/// Weight: 900
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(SmartString, into))]
pub struct AssetTransfer {
    #[serde(rename = "type")]
    transfer_type: TransferKind,
    asset: SmartString,
    amount: SmartString,
    from_symbol: Option<SmartString>,
    to_symbol: Option<SmartString>,
}

impl Request for AssetTransfer {
    type Response = AssetTransferResponse;
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/sapi/v1/asset/transfer";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 900)];
}

impl SignedRequest for AssetTransfer {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetTransferResponse {
    #[serde(rename = "tranId")]
    pub transfer_id: u64,
}

impl Response for AssetTransferResponse {}
