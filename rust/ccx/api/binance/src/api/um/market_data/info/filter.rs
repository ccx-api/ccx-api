use crate::api::um::prelude::*;

/// Filters define trading rules on a symbol or an exchange. Filters come in two forms:
/// symbol filters and exchange filters.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    Price(PriceFilter),
    #[serde(rename = "LOT_SIZE")]
    LotSize(LotSizeFilter),
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize(MarketLotSizeFilter),
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders(MaxNumOrdersFilter),
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders(MaxNumAlgoOrdersFilter),
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional(MinNotionalFilter),
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice(PercentPriceFilter),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub max_price: Decimal,
    pub min_price: Decimal,
    pub tick_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    pub max_qty: Decimal,
    pub min_qty: Decimal,
    pub step_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MarketLotSizeFilter {
    pub max_qty: Decimal,
    pub min_qty: Decimal,
    pub step_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumOrdersFilter {
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumAlgoOrdersFilter {
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MinNotionalFilter {
    pub notional: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PercentPriceFilter {
    pub multiplier_up: Decimal,
    pub multiplier_down: Decimal,
    pub multiplier_decimal: Decimal,
}
