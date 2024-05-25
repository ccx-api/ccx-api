use ccx_api_lib::Atom;
use ccx_api_lib::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::maybe_str;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetDetails {
    /// The name of the asset.
    pub name: Atom,
    /// The mutable series of letters used to identify the asset.
    pub symbol: Atom,
    /// The number of decimals supported for the asset.
    pub decimal_precision: Decimal,
    /// Indicates whether this asset can be traded.
    pub trading_supported: bool,
    /// Base URL to our recommended block explorer (crypto only).
    #[serde(with = "maybe_str")]
    pub explorer_url: Option<Atom>,
}

#[cfg(test)]
mod tests {
    use ccx_coinbase_examples_util::d;

    use super::*;

    #[test]
    fn test_deserialize_product_details() {
        let json = r#"{
            "name": "Bitcoin",
            "symbol": "BTC",
            "decimal_precision": "8",
            "trading_supported": true,
            "explorer_url": "https://live.blockcypher.com/btc/"
        }"#;
        let sample = AssetDetails {
            name: "Bitcoin".into(),
            symbol: "BTC".into(),
            decimal_precision: d("8"),
            trading_supported: true,
            explorer_url: Some("https://live.blockcypher.com/btc/".into()),
        };
        let product_details: AssetDetails = serde_json::from_str(json).unwrap();
        assert_eq!(product_details, sample);
    }
}
