use chrono::Utc;
use uuid::Uuid;

use super::prelude::*;
use crate::api::prime::RL_PORTFOLIO_KEY;
use crate::client::Task;
use crate::dt_coinbase::DtCoinbase;

/// List all portfolios for which the current API key has read access. (Currently, an API key
/// is scoped to only one portfolio).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioCreditResponse {
    /// A list of portfolios.
    pub post_trade_credit: AccountPortfolioCredit,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioCredit {
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

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// Get Account Balance.
    ///
    /// Retrieve all cash balances, net of pending withdrawals.
    ///
    /// [https://docs.cloud.coinbase.com/prime/reference/primerestapi_getposttradecredit]
    pub fn get_portfolio_credit(&self, portfolio_id: Uuid) -> CoinbaseResult<Task<AccountPortfolioCreditResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/credit");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
