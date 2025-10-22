use derive_more::Deref;
use serde::Deserialize;
use serde::Serialize;

use super::ProductDetails;
use crate::api::prime::types::NextPage;

/// List all products, available in the portfolio.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Deref)]
pub struct ProductList {
    /// A list of products.
    #[deref]
    pub products: Vec<ProductDetails>,
    pub pagination: NextPage,
}
