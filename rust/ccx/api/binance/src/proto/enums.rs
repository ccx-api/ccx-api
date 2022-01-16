use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ChartInterval {
    #[serde(rename = "1m")]
    Minute1,
    #[serde(rename = "3m")]
    Minute3,
    #[serde(rename = "5m")]
    Minute5,
    #[serde(rename = "15m")]
    Minute15,
    #[serde(rename = "30m")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "2h")]
    Hour2,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "6h")]
    Hour6,
    #[serde(rename = "8h")]
    Hour8,
    #[serde(rename = "12h")]
    Hour12,
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "3d")]
    Day3,
    #[serde(rename = "1w")]
    Week1,
    #[serde(rename = "1M")]
    Month1,
}

impl ChartInterval {
    pub fn as_str(self) -> &'static str {
        use ChartInterval::*;
        match self {
            Minute1 => "1m",
            Minute3 => "3m",
            Minute5 => "5m",
            Minute15 => "15m",
            Minute30 => "30m",
            Hour1 => "1h",
            Hour2 => "2h",
            Hour4 => "4h",
            Hour6 => "6h",
            Hour8 => "8h",
            Hour12 => "12h",
            Day1 => "1d",
            Day3 => "3d",
            Week1 => "1w",
            Month1 => "1M",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SymbolType {
    #[serde(rename = "SPOT")]
    Spot,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderBookStreamLimit {
    N5 = 5,
    N10 = 10,
    N20 = 20,
}

impl OrderBookStreamLimit {
    pub fn as_str(self) -> &'static str {
        use OrderBookStreamLimit::*;
        match self {
            N5 => "5",
            N10 => "10",
            N20 => "20",
        }
    }
}

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
