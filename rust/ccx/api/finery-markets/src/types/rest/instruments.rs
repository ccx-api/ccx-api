use crate::types::Pair;
use crate::types::Price;
use crate::types::Size;

pub const API_INSTRUMENTS: &str = "api/instruments";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct InstrumentsRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct InstrumentsResponse {
    pub currencies: Vec<Currency>,
    pub instruments: Vec<Instrument>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Currency {
    pub name: String,
    pub id: u32,
    pub size: Size,
    pub price: Price,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Instrument {
    pub pair: Pair,
    pub id: u64,
    /// repeat pair base
    pub asset_code: String,
    /// repeat pair quote
    pub balance_code: String,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest::test_serde_response;
    use crate::types::rest::test_serde_response_err;
    use crate::types::rest::test_serde_value_type;

    #[test]
    pub(crate) fn test_serde_instruments() {
        let json = r#"
        {}
        "#;
        test_serde_value_type::<InstrumentsRequest>(json);

        let json = r#"
        [
            [
                [
                    "USD",
                    234525425,
                    1000000,
                    100000000
                ],
                [
                    "BTC",
                    345465767,
                    1,
                    1000000000000
                ]
            ],
            [
                [
                    "BTC-USD",
                    35462742745,
                    "BTC",
                    "USD"
                ]
            ]
        ]
        "#;
        test_serde_response::<InstrumentsResponse>(json);
        test_serde_response_err::<InstrumentsResponse>();
    }
}
