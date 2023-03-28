use crate::api::exchange::prelude::*;
use crate::api::exchange::product::ProductStatus;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Product {
    /// The unique identifier for the product.
    pub id: Atom,
    /// The display name of the product.
    pub display_name: Atom,
    /// The base currency of the product.
    pub base_currency: Atom,
    /// The quote currency of the product.
    pub quote_currency: Atom,
    /// The minimum increment for the price of the product in the base currency.
    pub base_increment: Decimal,
    /// The minimum increment for the price of the product in the quote currency.
    pub quote_increment: Decimal,
    // /// The minimum size of an order in the base currency.
    // pub base_min_size: Decimal,
    // /// The maximum size of an order in the base currency.
    // pub base_max_size: Decimal,
    /// The minimum size of an order in the quote currency.
    pub min_market_funds: Decimal,
    // /// The maximum size of an order in the quote currency.
    // pub max_market_funds: Decimal,
    /// The status of the product.
    pub status: ProductStatus,
    /// Additional information about the status of the product, if available.
    pub status_message: Option<String>,
    /// Whether the product is post-only.
    pub post_only: bool,
    /// Whether the product is limit-only.
    pub limit_only: bool,
    /// Whether the product is cancel-only.
    pub cancel_only: bool,
    /// Whether the product is trading-disabled.
    pub trading_disabled: Option<bool>,
    /// Whether margin trading is enabled for the product.
    pub margin_enabled: bool,
    /// Whether the product is a stablecoin pegged to a fiat currency.
    pub fx_stablecoin: Option<bool>,
    /// The maximum slippage percentage allowed for an order in the quote currency, if specified.
    pub max_slippage_percentage: Option<Decimal>,
    /// Whether the product is currently in auction mode.
    pub auction_mode: bool,
    /// Percentage to calculate highest price for limit buy order (Stable coin trading pair only).
    pub high_bid_limit_percentage: Option<String>,
}
