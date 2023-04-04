use crate::api::prime::prelude::*;

#[cfg(feature = "db")]
use diesel_derives::{AsExpression, FromSqlRow};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum PortfolioOrderStatus {
    /// The order is open but unfilled.
    #[serde(rename = "OPEN")]
    Open,
    /// The order was filled.
    #[serde(rename = "FILLED")]
    Filled,
    /// The order was cancelled.
    #[serde(rename = "CANCELLED")]
    Cancelled,
    /// The order has expired.
    #[serde(rename = "EXPIRED")]
    Expired,
    /// Order submission failed.
    #[serde(rename = "FAILED")]
    Failed,
    /// The order has been sent but is not yet confirmed.
    #[serde(rename = "PENDING")]
    Pending,
}
#[cfg(feature = "db")]
forward_display_to_serde!(PortfolioOrderStatus);
#[cfg(feature = "db")]
forward_from_str_to_serde!(PortfolioOrderStatus);

impl PortfolioOrderStatus {
//     pub fn from_name(name: &str) -> Option<Self> {
//         Self::from_str(name).ok()
//     }
//
//     pub fn name(&self) -> String {
//         self.to_string()
//     }

    pub fn as_str(&self) -> &'static str {
        match self {
            PortfolioOrderStatus::Open => "OPEN",
            PortfolioOrderStatus::Filled => "FILLED",
            PortfolioOrderStatus::Cancelled => "CANCELLED",
            PortfolioOrderStatus::Expired => "EXPIRED",
            PortfolioOrderStatus::Failed => "FAILED",
            PortfolioOrderStatus::Pending => "PENDING",
        }
    }
}

impl AsRef<str> for PortfolioOrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
