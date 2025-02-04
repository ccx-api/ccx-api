use ccx_api_lib::Atom;
use ccx_api_lib::Decimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProductDetails {
    /// The product ID, written as `BASE-QUOTE`
    pub id: Atom,
    /// The smallest permitted unit of denomination for the base asset (varies by product)
    pub base_increment: Decimal,
    /// The smallest permitted unit of denomination for the quote asset (varies by product)
    pub quote_increment: Decimal,
    /// The minimum size (in base asset units) for which an order can be placed
    pub base_min_size: Decimal,
    /// The minimum size (in quote asset units) for which an order can be placed
    pub quote_min_size: Decimal,
    /// The maximum size (in base asset units) for which an order can be placed
    pub base_max_size: Decimal,
    /// The maximum size (in quote asset units) for which an order can be placed
    pub quote_max_size: Decimal,
    /// Permissions given to the user for a product
    pub permissions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use ccx_coinbase_examples_util::d;

    use super::*;

    #[test]
    fn test_deserialize_product_details() {
        let json = r#"{
            "id": "BTC-USD",
            "base_increment": "1",
            "quote_increment": "1",
            "base_min_size": "100",
            "quote_min_size": "100",
            "base_max_size": "1000",
            "quote_max_size": "1000",
            "permissions": [
              "string"
            ]
          }"#;
        let sample = ProductDetails {
            id: "BTC-USD".into(),
            base_increment: d("1"),
            quote_increment: d("1"),
            base_min_size: d("100"),
            quote_min_size: d("100"),
            base_max_size: d("1000"),
            quote_max_size: d("1000"),
            permissions: vec!["string".to_string()],
        };
        let product_details: ProductDetails = serde_json::from_str(json).unwrap();
        assert_eq!(product_details, sample);
    }
}
