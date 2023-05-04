use derive_more::Deref;
use serde::{Deserialize, Serialize};

use super::PortfolioDetails;

/// List all portfolios for which the current API key has read access. (Currently, an API key
/// is scoped to only one portfolio).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Deref)]
pub struct PortfolioList {
    /// A list of portfolios.
    pub portfolios: Vec<PortfolioDetails>,
}
