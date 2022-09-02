#![allow(unused_imports)]

use std::str::FromStr;

#[cfg(feature = "db")]
use diesel_derives::{AsExpression, FromSqlRow};
use rust_decimal::Decimal;

use crate::json_string;
use crate::uuid_simple;
use crate::PayerInfo;
use crate::TradeType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum BizStatus {
    #[serde(rename = "PAY_SUCCESS")]
    PaySuccess,
    #[serde(rename = "PAY_CLOSED")]
    PayClosed,
}
forward_display_to_serde!(BizStatus);
forward_from_str_to_serde!(BizStatus);

impl BizStatus {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BinancePayWebHookRequest<T>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    #[serde(rename = "bizType")]
    pub biz_type: String, //  string  Y   -   "PAY"
    #[serde(rename = "bizId")]
    /// TODO FixMe!!!
    pub biz_id: u64, //  string	Y	-	Prepay order id
    #[serde(rename = "bizStatus")]
    pub biz_status: BizStatus, //	string	Y	-	"PAY_SUCCESS"
    #[serde(rename = "data")]
    #[serde(with = "json_string")]
    pub notification: T, //	string	Y	-	JSON string, data details refer to
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum ReturnCode {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAIL")]
    Fail,
}
forward_display_to_serde!(ReturnCode);
forward_from_str_to_serde!(ReturnCode);

impl ReturnCode {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BinancePayWebHookResponse {
    #[serde(rename = "returnCode")]
    pub return_code: ReturnCode, //  string	Y	-   "SUCCESS" or "FAIL"	result code of notification processing, if process fail, Binance Pay will retry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnMessage")]
    pub return_message: Option<String>, //  string	N	-	return message
}

impl BinancePayWebHookResponse {
    pub fn new(return_code: ReturnCode, return_message: Option<String>) -> Self {
        Self {
            return_code,
            return_message,
        }
    }

    pub fn success() -> Self {
        Self::new(ReturnCode::Success, None)
    }

    pub fn fail() -> Self {
        Self::new(ReturnCode::Fail, None)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    #[serde(rename = "merchantTradeNo", with = "uuid_simple")]
    pub merchant_trade_no: uuid::Uuid, //	string	Y	-   letter or digit, no other symbol allowed	The order id, Unique identifier for the request
    #[serde(rename = "productType")]
    pub product_type: String, //  string	Y	-   maximum length 16	product type
    #[serde(rename = "productName")]
    pub product_name: String, //  string	Y	-   maximum length 256	product name
    #[serde(rename = "tradeType")]
    pub trade_type: TradeType, //  string	Y	-   "WEB", "APP"	operate entrance
    #[serde(rename = "totalFee")]
    pub total_fee: Decimal, //  decimal	Y	-	order amount
    pub currency: String, //  string	Y	-	order currency
    #[serde(rename = "transactTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<i64>, //  long	N	-	Timestamp when transaction happened
    #[serde(rename = "openUserId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_user_id: Option<String>, //  string	N	-	Consumer unique id
    #[serde(rename = "transactionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>, // string	N	-	issued once the payment is successful
    pub commission: Decimal, //  decimal	Y	-	Commission fee if any
    #[serde(rename = "payerInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payer_info: Option<PayerInfo>, //  string	N	-   only merchant got approved by Binance Operation's approval will receive this payerInfo	payer information, refer to
}

#[cfg(feature = "db")]
mod db_impl {
    use std::io::Write;

    use diesel::deserialize::FromSql;
    use diesel::serialize::ToSql;

    use super::{BizStatus, ReturnCode};

    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for BizStatus
    where
        DB: diesel::backend::Backend,
        str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
    {
        fn to_sql<W: std::io::Write>(
            &self,
            out: &mut diesel::serialize::Output<W, DB>,
        ) -> diesel::serialize::Result {
            self.name().as_str().to_sql(out)
        }
    }

    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for BizStatus
    where
        DB: diesel::backend::Backend,
        String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
            let name = String::from_sql(bytes)?;
            Self::from_name(name.as_str()).ok_or_else(|| {
                format!("Unrecognized name {:?} for {}", name, stringify!($name)).into()
            })
        }
    }

    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for ReturnCode
    where
        DB: diesel::backend::Backend,
        str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
    {
        fn to_sql<W: std::io::Write>(
            &self,
            out: &mut diesel::serialize::Output<W, DB>,
        ) -> diesel::serialize::Result {
            self.name().as_str().to_sql(out)
        }
    }

    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for ReturnCode
    where
        DB: diesel::backend::Backend,
        String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
            let name = String::from_sql(bytes)?;
            Self::from_name(name.as_str()).ok_or_else(|| {
                format!("Unrecognized name {:?} for {}", name, stringify!($name)).into()
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serde_response() {
        let example = r#"{"returnCode":"SUCCESS","returnMessage":null}"#;
        let response: BinancePayWebHookResponse =
            serde_json::from_str(example).expect("Failed from_str");
        println!("test_serde_response response :: {:#?}", response);
    }

    #[test]
    #[should_panic]
    fn test_serde_request_example() {
        let example = r#"
        {
            "bizType": "PAY",
            "data": "{\"merchantTradeNo\":\"9a1c04a06dbc432e94fa4e2bd693c663\",\"productType\":\"Food\",\"productName\":\"Ice Cream\",\"tradeType\":\"WEB\",\"totalFee\":0.88000000,\"currency\":\"EUR\",\"transactTime\":1619508939664,\"openUserId\":\"1211HS10K81f4273ac031\",\"commission\":0,\"transactionId\":\"M_R_282737362839373\"}",
            "bizId": 29383937493038367292,
            "bizStatus": "PAY_SUCCESS"
        }
        "#;
        let request: BinancePayWebHookRequest<Notification> =
            serde_json::from_str(example).expect("Failed from_str");
        println!("test_serde_request biz_id :: {:?}", request.biz_id);
        // println!("test_serde_request biz_id :: {:?}", request.biz_id.as_f64());
        println!("test_serde_request biz_id :: {}", request.biz_id);
        println!("test_serde_request request :: {:#?}", request);
    }

    #[test]
    fn test_serde_request_real() {
        println!("test_serde_request request :: {:?}", u64::MAX);
        println!("test_serde_request request :: {:?}", f64::MAX);
        // 18446744073709551615
        // 993143214321423154315154321
        // 29383937493038367292
        // 100450325614108672
        let example = r#"
        {
            "bizType": "PAY",
            "data": "{\"merchantTradeNo\":\"91f56e5d2b124c94a77e448ac886fc12\",\"productType\":\"Food\",\"productName\":\"Ice Cream\",\"tradeType\":\"WEB\",\"totalFee\":0.50000000,\"currency\":\"BUSD\",\"transactTime\":1624612653893,\"openUserId\":\"8656d908d8b19648d714ef9e49de070a\",\"commission\":0,\"transactionId\":\"M_R_100450379917516801\"}",
            "bizId": 100450325614108672,
            "bizStatus": "PAY_SUCCESS"
        }
        "#;
        let request: BinancePayWebHookRequest<Notification> =
            serde_json::from_str(example).expect("Failed from_str");
        println!("test_serde_request_real biz_id :: {:?}", request.biz_id);
        println!("test_serde_request_real biz_id :: {}", request.biz_id);
        println!("test_serde_request_real request :: {:#?}", request);
    }
}
