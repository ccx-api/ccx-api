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
    #[serde(default)]
    pub r#type: Option<String>,
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
                ["USD",4725,1000000,100000000,"fiat"],
                ["EUR",19109,1000000,109000000,"fiat"],
                ["USDT",660085,100,100085000,"stablecoin"],
                ["BTC",3714,1,3016218000000,"crypto"]
            ],
            [
                ["BTC-USD",4955410050,"BTC","USD"]
            ]
        ]
        "#;
        test_serde_response::<InstrumentsResponse>(json);
        test_serde_response_err::<InstrumentsResponse>();
    }
}
