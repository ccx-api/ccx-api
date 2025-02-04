use derive_more::Deref;
use serde::Deserialize;
use serde::Serialize;

use super::PortfolioDetails;

/// List all portfolios for which the current API key has read access. (Currently, an API key
/// is scoped to only one portfolio).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Deref)]
pub struct PortfolioInfo {
    /// A list of portfolios.
    pub portfolio: PortfolioDetails,
}
