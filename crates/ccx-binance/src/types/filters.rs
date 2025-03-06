use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

/// Filters define trading rules on a symbol or an exchange. Filters come in two forms:
/// symbol filters and exchange filters.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    Price(PriceFilter),
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice(PercentPriceFilter),
    #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
    PercentPriceBySide(PercentPriceBySideFilter),
    #[serde(rename = "LOT_SIZE")]
    LotSize(LotSizeFilter),
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional(MinNotionalFilter),
    #[serde(rename = "NOTIONAL")]
    Notional(NotionalFilter),
    #[serde(rename = "ICEBERG_PARTS")]
    IcebergParts(IcebergPartsFilter),
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize(MarketLotSizeFilter),
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders(MaxNumOrdersFilter),
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders(MaxNumAlgoOrdersFilter),
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    MaxNumIcebergOrders(MaxNumIcebergOrdersFilter),
    #[serde(rename = "MAX_POSITION")]
    MaxPosition(MaxPositionFilter),
    #[serde(rename = "TRAILING_DELTA")]
    TrailingDelta(TrailingDeltaFilter),
}

/// The PRICE_FILTER defines the price rules for a symbol. There are 3 parts:
///
/// * `min_price` defines the minimum `price`/`stop_price` allowed;
///   disabled on `min_price` == 0.
/// * `max_price` defines the maximum `price`/`stop_price` allowed;
///   disabled on `max_price` == 0.
/// * `tick_size` defines the intervals that a `price`/`stop_price`
///   can be increased/decreased by; disabled on `tick_size` == 0.
///
/// Any of the above variables can be set to 0, which disables that rule in the price filter.
/// In order to pass the price filter, the following must be true for `price`/`stop_price`
/// of the enabled rules:
///
/// * `price` >= `min_price`
/// * `price` <= `max_price`
/// * (`price` - `min_price`) % `tick_size` == 0
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub min_price: Decimal,
    pub max_price: Decimal,
    pub tick_size: Decimal,
}

/// The PERCENT_PRICE filter defines valid range for a price based on the average of the previous
/// trades. `avgPriceMins` is the number of minutes the average price is calculated over. 0 means
/// the last price is used.
///
/// In order to pass the percent price, the following must be true for price:
///
/// * `price` <= `weightedAveragePrice` * `multiplierUp`
/// * `price` >= `weightedAveragePrice` * `multiplierDown`
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PercentPriceFilter {
    pub multiplier_up: Decimal,
    pub multiplier_down: Decimal,
    pub avg_price_mins: u64,
}

/// The PERCENT_PRICE_BY_SIDE filter defines the valid range for the price based on the lastPrice
/// of the symbol. There is a different range depending on whether the order is placed
/// on the `BUY` side or the `SELL` side.
///
/// Buy orders will succeed on this filter if:
///
/// * `Order price` <= `bidMultiplierUp` * `lastPrice`
/// * `Order price` >= `bidMultiplierDown` * `lastPrice`
///
/// Sell orders will succeed on this filter if:
///
/// * `Order Price` <= `askMultiplierUp` * `lastPrice`
/// * `Order Price` >= `askMultiplierDown` * `lastPrice`
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PercentPriceBySideFilter {
    pub bid_multiplier_up: Decimal,
    pub bid_multiplier_down: Decimal,
    pub ask_multiplier_up: Decimal,
    pub ask_multiplier_down: Decimal,
    pub avg_price_mins: u64,
}

/// The LOT_SIZE filter defines the quantity (aka "lots" in auction terms) rules for a symbol.
/// There are 3 parts:
///
/// * `minQty` defines the minimum `quantity`/`icebergQty` allowed.
/// * `maxQty` defines the maximum `quantity`/`icebergQty` allowed.
/// * `stepSize` defines the intervals that a `quantity`/`icebergQty` can be increased/decreased by.
///
/// In order to pass the lot size, the following must be true for `quantity`/`icebergQty`:
///
/// * `quantity` >= `minQty`
/// * `quantity` <= `maxQty`
/// * (`quantity` - `minQty`) % `stepSize` == `0`
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    pub min_qty: Decimal,
    pub max_qty: Decimal,
    pub step_size: Decimal,
}

/// The MIN_NOTIONAL filter defines the minimum notional value allowed for an order on a symbol.
/// An order's notional value is the `price` * `quantity`. If the order is an Algo order
/// (e.g. STOP_LOSS_LIMIT), then the notional value of the `stopPrice` * `quantity` will also be
/// evaluated. If the order is an Iceberg Order, then the notional value of the
/// `price` * `icebergQty` will also be evaluated. `applyToMarket` determines whether or not the
/// MIN_NOTIONAL filter will also be applied to MARKET orders. Since MARKET orders have no `price`,
/// the average price is used over the last `avgPriceMins` minutes. `avgPriceMins` is the number
/// of minutes the average price is calculated over. `0` means the last price is used.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MinNotionalFilter {
    pub min_notional: Decimal,
    pub apply_to_market: bool,
    pub avg_price_mins: u64,
}

/// The NOTIONAL filter defines the acceptable notional range allowed for an order on a symbol.
/// applyMaxToMarket determines whether the maxNotional will be applied to MARKET orders.
///
/// In order to pass this filter, the notional (price * quantity) has to pass the following conditions:
/// price * quantity <= maxNotional
/// price * quantity >= minNotional
///
/// For MARKET orders, the average price used over the last avgPriceMins minutes will be used for calculation.
/// If the avgPriceMins is 0, then the last price will be used.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotionalFilter {
    pub min_notional: Decimal,
    pub max_notional: Decimal,
    #[serde(default)]
    pub apply_to_market: bool,
    #[serde(default)]
    pub avg_price_mins: u64,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IcebergPartsFilter {
    pub limit: u64,
}

/// The MARKET_LOT_SIZE filter defines the quantity (aka "lots" in auction terms) rules for MARKET
/// orders on a symbol. There are 3 parts:
///
/// * `minQty` defines the minimum `quantity`/`icebergQty` allowed.
/// * `maxQty` defines the maximum `quantity`/`icebergQty` allowed.
/// * `stepSize` defines the intervals that a `quantity`/`icebergQty` can be increased/decreased by.
///
/// In order to pass the lot size, the following must be true for `quantity`/`icebergQty`:
///
/// * `quantity` >= `minQty`
/// * `quantity` <= `maxQty`
/// * (`quantity` - `minQty`) % `stepSize` == `0`
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarketLotSizeFilter {
    pub min_qty: Decimal,
    pub max_qty: Decimal,
    pub step_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumOrdersFilter {
    pub max_num_orders: u64,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumAlgoOrdersFilter {
    pub max_num_algo_orders: u64,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumIcebergOrdersFilter {
    pub max_num_iceberg_orders: u64,
}

/// The `MAX_POSITION` filter defines the allowed maximum position an account can have on the
/// base asset of a symbol. An account's position defined as the sum of the account's:
///
/// * free balance of the base asset
/// * locked balance of the base asset
/// * sum of the qty of all open BUY orders
///
/// BUY orders will be rejected if the account's position is greater than the maximum position
/// allowed.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MaxPositionFilter {
    pub max_position: Decimal,
}

/// The `MAX_POSITION` filter defines the allowed maximum position an account can have on the
/// base asset of a symbol. An account's position defined as the sum of the account's:
///
/// * free balance of the base asset
/// * locked balance of the base asset
/// * sum of the qty of all open BUY orders
///
/// BUY orders will be rejected if the account's position is greater than the maximum position
/// allowed.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrailingDeltaFilter {
    pub min_trailing_above_delta: Decimal,
    pub max_trailing_above_delta: Decimal,
    pub min_trailing_below_delta: Decimal,
    pub max_trailing_below_delta: Decimal,
}
