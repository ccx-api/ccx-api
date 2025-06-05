use macro_rules_attribute::apply;

use crate::prelude::Coin;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::Request;
use crate::types::derive::Response;
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
pub struct ListCurrencies {
    /// The id of the trading account to retrieve
    #[serde(skip)]
    account_id: String,
}

/// Currency information
#[apply(Response)]
pub struct Currency {
    /// Unique identifier for the currency
    pub id: String,
    /// Currency symbol (e.g., "BTC", "ETH")
    pub symbol: Coin,
    /// Human readable name of the currency
    pub name: String,
}

#[apply(Response)]
pub struct ListCurrenciesResponse {
    /// Array of available currencies
    #[serde(rename = "data")]
    pub currencies: Vec<Currency>,
}

impl Response for ListCurrenciesResponse {}

impl Request for ListCurrencies {
    type Response = ListCurrenciesResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let account_id = &self.account_id;

        format!("/api/prime/trading/v1/accounts/{account_id}/currencies").into()
    }
}

impl SignedRequest for ListCurrencies {}

#[cfg(test)]
mod tests {
    use super::*;

    mod serialize {
        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn serialize_list_currencies() {
            let list_currencies = ListCurrencies::builder()
                .account_id("6016e0a9-545a-45fb-8370-caab1680956a")
                .build();

            let actual = serde_json::to_value(&list_currencies).unwrap();
            let expected = json!({});

            assert_eq!(actual, expected);
        }
    }

    mod deserialize {
        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn deserialize_list_currencies_response() {
            let json = json!({
                "data": [
                    {
                        "id": "6016e0a9-545a-45fb-8370-caab1680956a",
                        "symbol": "BTC",
                        "name": "Bitcoin"
                    },
                    {
                        "id": "7027f1ba-656b-56gc-9481-dbbc2791067b",
                        "symbol": "ETH",
                        "name": "Ethereum"
                    }
                ]
            });

            let response: ListCurrenciesResponse = serde_json::from_value(json).unwrap();

            assert_eq!(response.currencies.len(), 2);

            let btc = &response.currencies[0];
            assert_eq!(btc.id, "6016e0a9-545a-45fb-8370-caab1680956a");
            assert_eq!(btc.symbol, "BTC");
            assert_eq!(btc.name, "Bitcoin");

            let eth = &response.currencies[1];
            assert_eq!(eth.id, "7027f1ba-656b-56gc-9481-dbbc2791067b");
            assert_eq!(eth.symbol, "ETH");
            assert_eq!(eth.name, "Ethereum");
        }

        #[test]
        fn deserialize_empty_currencies_response() {
            let json = json!({
                "data": []
            });

            let response: ListCurrenciesResponse = serde_json::from_value(json).unwrap();

            assert_eq!(response.currencies.len(), 0);
        }
    }
}
