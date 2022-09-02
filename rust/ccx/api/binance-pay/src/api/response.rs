use crate::error::BinanceError;
use crate::error::LibError;
use crate::error::LibResult;
use crate::types::enums::StatusRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BinancePayResponse<T>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    pub status: StatusRequest,
    pub code: String,
    #[serde(with = "json_response", skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_none")]
    pub data: Option<T>,
    #[serde(rename = "errorMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl<T> BinancePayResponse<T>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    pub fn to_result(self) -> LibResult<T> {
        match (self.status, self.data) {
            (StatusRequest::Success, Some(data)) => Ok(data),
            (StatusRequest::Success, None) => Err(LibError::other("Unknown result."))?,
            (status @ StatusRequest::Fail, _) => Err(LibError::BinanceError(BinanceError {
                status,
                code: self.code,
                error_message: self.error_message.unwrap_or_default(),
                params: None,
            }))?,
        }
    }
}

fn default_none<T>() -> Option<T> {
    None::<T>
}

pub mod json_response {
    use serde::de::{Deserialize, DeserializeOwned, Deserializer};
    use serde::ser::{self, Serialize, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        match value {
            Some(data) => {
                let j = serde_json::to_string(data).map_err(ser::Error::custom)?;
                j.serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
    {
        let j = Option::<T>::deserialize(deserializer)?;
        Ok(j)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serde_binance_pay_response_none() {
        let example = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "errorMessage": ""
        }
        "#;
        let response: BinancePayResponse<bool> =
            serde_json::from_str(example).expect("Failed from_str");
        println!(
            "test_serde_binance_pay_response_bool response :: {:#?}",
            response
        );
    }

    #[test]
    fn test_serde_binance_pay_response_bool() {
        let example = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": false,
            "errorMessage": ""
        }
        "#;
        let response: BinancePayResponse<bool> =
            serde_json::from_str(example).expect("Failed from_str");
        println!(
            "test_serde_binance_pay_response_bool response :: {:#?}",
            response
        );
    }

    #[test]
    fn test_serde_binance_pay_response_query_order() {
        use crate::V1QueryOrderResult;

        let example = r#"
        {
            "status":"SUCCESS",
            "code":"000000",
            "data":{
                "merchantId":134697918,
                "prepayId":"99695089974435840",
                "merchantTradeNo":"9a1c04a06dbc432e94fa4e2bd693c663",
                "tradeType":"WEB",
                "status":"INITIAL",
                "currency":"BUSD",
                "totalFee":"0.50000000",
                "productName":"Ice Cream",
                "productDetail":"Greentea ice cream cone",
                "createTime":1624260944011
            }
        }
        "#;
        let response: BinancePayResponse<V1QueryOrderResult> =
            serde_json::from_str(example).expect("Failed from_str");
        println!(
            "test_serde_binance_pay_response_query_order response :: {:#?}",
            response
        );
    }
}
