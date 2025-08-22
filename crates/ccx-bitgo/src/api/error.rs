use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    derive_more::Error,
    derive_more::Display,
    derive_more::From,
)]
#[serde(untagged)]
pub enum BitGoApiError {
    Generic(BitGoGenericApiError),
    MinQuantity(BitGoMinQuantityError),
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, derive_more::Error, derive_more::Display,
)]
#[display("The minimum quantity for {currency_symbol} is {min_quantity}")]
#[serde(rename_all = "camelCase")]
pub struct BitGoMinQuantityError {
    currency_symbol: String,
    min_quantity: Decimal,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, derive_more::Error, derive_more::Display,
)]
#[display("{error_name}: {error}")]
#[serde(rename_all = "camelCase")]
pub struct BitGoGenericApiError {
    /// Human-readable error message
    error: String,
    /// Contains error code
    #[serde(default = "default_error_name")]
    error_name: String,
    #[serde(default = "default_error_name")]
    name: String,
    #[serde(alias = "reqId")]
    request_id: Option<String>,
}

fn default_error_name() -> String {
    "UnknownError".to_string()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_bitgo_express_error() {
        let json = json!({
            "error":"session expired",
            "errorName":"Error",
            "reqId":"unk-tkrurtbjfowiwu3t8es8",
            "context":{"errorName":"Error"},
            "message":"session expired",
            "name":"ApiResponseError",
            "bitgoJsVersion":"49.1.0",
            "bitgoExpressVersion":"14.1.7"
        });

        let error: BitGoApiError = serde_json::from_value(json).unwrap();

        assert!(matches!(error, BitGoApiError::Generic(_)));
    }
}
