use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfolioDetails {
    /// The unique ID of the portfolio.
    pub id: Uuid,
    /// The name of the portfolio.
    pub name: String,
    /// The ID of the entity to which the portfolio is associated.
    pub entity_id: Uuid,
    /// The ID of the organization to which the portfolio is associated.
    pub organization_id: Uuid,
}
