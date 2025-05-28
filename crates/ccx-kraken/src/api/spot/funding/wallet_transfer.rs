use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::asset_info::AssetName;
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

/// Transfer from a Kraken spot wallet to a Kraken Futures wallet. Note that a transfer in the other direction must be requested via the Kraken Futures API endpoint for withdrawals to Spot wallets.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct WalletTransfer {
    /// Asset to transfer (asset ID or altname)
    asset: AssetName,
    /// Source wallet
    #[builder(default)]
    from: WalletFrom,
    /// Destination wallet
    #[builder(default)]
    to: WalletTo,
    /// Amount to transfer
    amount: String,
}

#[derive(Serialize, Clone, Debug, Default)]
pub enum WalletFrom {
    #[default]
    #[serde(rename = "Spot Wallet")]
    SpotWallet,
}

#[derive(Serialize, Clone, Debug, Default)]
pub enum WalletTo {
    #[default]
    #[serde(rename = "Futures Wallet")]
    FuturesWallet,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WalletTransferResponse {
    /// Reference ID
    pub refid: String,
}

impl Response for WalletTransferResponse {}

impl Request for WalletTransfer {
    type Response = WalletTransferResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/WalletTransfer";

    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);
}

impl SignedRequest for WalletTransfer {}
