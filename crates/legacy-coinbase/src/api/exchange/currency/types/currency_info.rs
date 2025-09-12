use crate::api::exchange::currency::CurrencyDetails;
use crate::api::exchange::currency::CurrencyStatus;
use crate::api::exchange::currency::SupportedNetwork;
use crate::api::exchange::prelude::*;
use crate::util::maybe_str;

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyInfo {
    pub id: Atom,
    pub name: Atom,
    pub min_size: Decimal,
    pub status: CurrencyStatus,
    #[serde(with = "maybe_str")]
    pub message: Option<Atom>,
    pub max_precision: Decimal,
    pub convertible_to: Vec<Atom>,
    pub details: CurrencyDetails,
    #[serde(with = "maybe_str")]
    pub default_network: Option<Atom>,
    pub supported_networks: Vec<SupportedNetwork>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_usd() {
        let json = r###"{
            "id":"USD",
            "name":"United States Dollar",
            "min_size":"0.01",
            "status":"online",
            "message":"",
            "max_precision":"0.01",
            "convertible_to":["USDC"],
            "details":{
                "type":"fiat",
                "symbol":"$",
                "network_confirmations":null,
                "sort_order":0,
                "crypto_address_link":null,
                "crypto_transaction_link":null,
                "push_payment_methods":[
                    "bank_wire",
                    "swift_bank_account",
                    "intra_bank_account"
                ],
                "group_types":[
                    "fiat",
                    "usd"
                ],
                "display_name":"US Dollar",
                "processing_time_seconds":null,
                "min_withdrawal_amount":null,
                "max_withdrawal_amount":null
            },
            "default_network":"",
            "supported_networks":[]
        }"###;
        let _decoded: CurrencyInfo = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_btc() {
        let json = r###"{
            "id":"BTC",
            "name":"Bitcoin",
            "min_size":"0.00000001",
            "status":"online",
            "message":"",
            "max_precision":"0.00000001",
            "convertible_to":[],
            "details":{
                "type":"crypto",
                "symbol":null,
                "network_confirmations":2,
                "sort_order":0,
                "crypto_address_link":"https://live.blockcypher.com/btc/address/{{address}}",
                "crypto_transaction_link":"https://live.blockcypher.com/btc/tx/{{txId}}",
                "push_payment_methods":[],
                "group_types":[],
                "display_name":null,
                "processing_time_seconds":null,
                "min_withdrawal_amount":0.0001,
                "max_withdrawal_amount":2400
            },
            "default_network":"bitcoin",
            "supported_networks":[
                {
                    "id":"bitcoin",
                    "name":"Bitcoin",
                    "status":"online",
                    "contract_address":"",
                    "crypto_address_link":"https://live.blockcypher.com/btc/address/{{address}}",
                    "crypto_transaction_link":"https://live.blockcypher.com/btc/tx/{{txId}}",
                    "min_withdrawal_amount":0.0001,
                    "max_withdrawal_amount":2400,
                    "network_confirmations":2,
                    "processing_time_seconds":null
                }
            ]
        }"###;
        let _decoded: CurrencyInfo = serde_json::from_str(json).unwrap();
    }
}
