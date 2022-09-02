#[cfg(feature = "db")]
use diesel_derives::{AsExpression, FromSqlRow};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::api::Api;
use crate::error::LibResult;
use crate::types::enums::TransferStatus;
use crate::types::enums::TransferType;
use crate::types::time::Time;
use crate::uuid_simple;
use crate::BinancePayResponse;

pub const BINANCEPAY_OPENAPI_TRANSFER_FUND: &str = "/binancepay/openapi/wallet/transfer";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1TransferFundRequest {
    #[serde(rename = "requestId", with = "uuid_simple")]
    pub request_id: Uuid, //string	Y	maximum length 32	Represents the unique ID of each transfer request.Generated by the merchant
    #[serde(rename = "merchantId")]
    pub merchant_id: u64, //long	Y	-	The merchant account id, issued when merchant been created at Binance.
    pub currency: String, //string	Y	Not limited, as long as it is within the range.	transfer currency, e.g. "BUSD"
    pub amount: Decimal,  //  string	Y	Greater than 0	the transfer amount
    #[serde(rename = "transferType")]
    pub transfer_type: TransferType, //    string  Y   Only "TO_MAIN" OR "TO_PAY"	The transfer direction specified by the merchant
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1TransferResult {
    #[serde(rename = "tranId")]
    pub transfer_id: String, //	string	Y	-	Used to query the transfer status, query the necessary fields for the transfer status
    pub amount: Decimal,        //	string	Y	-	the transfer amount
    pub status: TransferStatus, //	string	Y	SUCCESS OR FAILURE OR PROCESS	SUCCESS (indicating that the transfer is completely successful), FAILURE (indicating that the transfer has failed, it may be that the transferor has a problem with the transferee), PROCESS (the transfer is in progress)
    pub currency: String, //	string	Y	Not limited, as long as it is within the range.	transfer currency, e.g. "BUSD"
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v1_transfer_fund(
        &self,
        request: V1TransferFundRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<BinancePayResponse<V1TransferResult>> {
        self.client
            .post_json(BINANCEPAY_OPENAPI_TRANSFER_FUND, request)?
            .signed(time_window)?
            .random_nonce()?
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn v1_test_serde_transfer_request() {
        let json = r#"
        {
            "requestId": "9a1c04a06dbc432e94fa4e2bd693c663",
            "merchantId": 98765987,
            "currency": "BNB",
            "amount": "0.01",
            "transferType": "TO_MAIN"
        }
        "#;
        let request: V1TransferFundRequest = serde_json::from_str(json).expect("Failed from_str");
        println!("test_serde_transfer_request :: {:#?}", request);

        let request = V1TransferFundRequest {
            request_id: uuid::Uuid::parse_str("9a1c04a0-6dbc-432e-94fa-4e2bd693c663")
                .expect("Failed parse_str"),
            merchant_id: 134697918,
            currency: "BUSD".to_string(),
            amount: Decimal::new(1, 2),
            transfer_type: TransferType::ToMain,
        };
        let json = serde_json::to_string(&request).expect("Failed to_string");
        println!("test_serde_transfer_request :: {}", json);
    }

    #[test]
    fn v1_test_serde_transfer_response() {
        let example = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": {
              "tranId": "4069044573",
              "amount": "0.01",
              "status": "SUCCESS",
              "currency": "BNB"
            },
            "errorMessage": ""
        }
        "#;
        let response: BinancePayResponse<V1TransferResult> =
            serde_json::from_str(example).expect("Failed from_str");
        println!("test_serde_transfer_response response :: {:#?}", response);
    }
}
