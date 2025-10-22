#[cfg(feature = "db")]
use diesel_derives::AsExpression;
#[cfg(feature = "db")]
use diesel_derives::FromSqlRow;

use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum PortfolioOrderTimeInForce {
    /// Expires at a certain date/time.
    #[serde(rename = "GOOD_UNTIL_DATE_TIME")]
    GoodUntilDateTime,
    /// Order stays on the books until cancelled.
    #[serde(rename = "GOOD_UNTIL_CANCELLED")]
    GoodUntilCancelled,
    /// Order is executed immediately at submission or is cancelled.
    #[serde(rename = "IMMEDIATE_OR_CANCEL")]
    ImmediateOrCancel,
}
#[cfg(feature = "db")]
forward_display_to_serde!(PortfolioOrderTimeInForce);
#[cfg(feature = "db")]
forward_from_str_to_serde!(PortfolioOrderTimeInForce);

impl PortfolioOrderTimeInForce {
    //     pub fn from_name(name: &str) -> Option<Self> {
    //         Self::from_str(name).ok()
    //     }
    //
    //     pub fn name(&self) -> String {
    //         self.to_string()
    //     }

    pub fn as_str(&self) -> &'static str {
        match self {
            PortfolioOrderTimeInForce::GoodUntilDateTime => "GOOD_UNTIL_DATE_TIME",
            PortfolioOrderTimeInForce::GoodUntilCancelled => "GOOD_UNTIL_CANCELLED",
            PortfolioOrderTimeInForce::ImmediateOrCancel => "IMMEDIATE_OR_CANCEL",
        }
    }
}

impl AsRef<str> for PortfolioOrderTimeInForce {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
