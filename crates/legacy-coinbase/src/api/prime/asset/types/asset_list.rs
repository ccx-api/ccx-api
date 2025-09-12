use derive_more::Deref;
use serde::Deserialize;
use serde::Serialize;

use super::AssetDetails;

/// List all assets, available for the entity.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Deref)]
pub struct AssetList {
    /// A list of assets.
    #[deref]
    pub assets: Vec<AssetDetails>,
}
