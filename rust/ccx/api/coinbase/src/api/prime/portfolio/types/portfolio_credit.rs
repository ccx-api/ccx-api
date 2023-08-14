use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::dt_coinbase::DtCoinbase;
use crate::Decimal;

/// List all portfolios for which the current API key has read access. (Currently, an API key
/// is scoped to only one portfolio).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfolioCredit {
    /// A list of portfolios.
    pub post_trade_credit: CreditDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CreditDetails {
    /// The unique ID of the portfolio.
    pub portfolio_id: Uuid,
    /// The currency symbol credit is denoted in.
    pub currency: String,
    /// The maximum credit limit.
    pub limit: Decimal,
    /// The amount of credit used.
    pub utilized: Decimal,
    /// The amount of credit available.
    pub available: Decimal,
    /// Whether or not a portfolio is frozen due to balance outstanding or other reason.
    pub frozen: bool,
    /// The reason why the portfolio is frozen.
    pub frozen_reason: String,
    pub amounts_due: Vec<AmountDue>,
    /// Whether the portfolio has credit enabled.
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AmountDue {
    /// The currency this loan is due in.
    pub currency: String,
    /// The amount due.
    pub amount: Decimal,
    /// The date this settlement is due, expressed in UTC.
    pub due_date: DtCoinbase,
}
