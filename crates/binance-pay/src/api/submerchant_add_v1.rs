use serde::Deserialize;
use serde::Serialize;

use crate::api::Api;
use crate::error::LibResult;
use crate::types::time::Time;
use crate::BinancePayResponse;

pub const BINANCEPAY_OPENAPI_SUBMERCHANT_ADD: &str = "/binancepay/openapi/submerchant/add";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1SubmerchantAddRequest {
    /// maximum length 128
    /// The sub merchant name maximum length 128,
    /// unique under one mainMerchantId.
    pub merchant_name: String,

    /// - 0: Online
    /// - 1: Physical
    /// - -1: Online&Physical
    pub store_type: i8,

    /// Specified code: four-digit number that classifies the business.
    /// Get from [here](1)
    ///
    /// [1]: https://developers.binance.com/docs/binance-pay/api-submerchant-add#MCC
    pub merchant_mcc: String,

    /// - 1: Personal(Individual)
    /// - 2: solo proprietor
    /// - 3: Partnership、
    /// - 4: Private company
    /// - 5: Others company
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_type: Option<u8>,

    /// [iso alpha 2 country code](1)
    /// use "GO" if global
    /// Country/Region of Business Operation,
    /// Can be multiple, split by "," eg:"SG,US"
    ///
    /// [1]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// maximum length 256
    /// For Online stores, the URL field is required.
    /// NOTE: If your online store is, APP, please pass its lint in the app store
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,

    /// maximum length 1024
    /// For Physical stores the address filed is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// maximum length 500
    /// Mandatory if merchantMcc is 9999.
    /// Please specify the industry of this sub merchant here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_industry_description: Option<String>,

    /// Specified code: four-digit number that classifies the business.
    /// Get from [here](1)
    ///
    /// [1]: https://developers.binance.com/docs/binance-pay/api-submerchant-add#MCC
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_pay_mcc_code: Option<String>,

    /// maximum length 500
    /// Mandatory if merchantMcc is 9999.
    /// Please specify the industry of this sub merchant here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_pay_industry_description: Option<String>,

    /// maximum length 256
    /// sub merchant logo url
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand_logo: Option<String>,

    /// maximum length 64
    /// The legal name that is used in the registration
    /// Required if merchantType is not Individual
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,

    /// maximum length 64
    /// Registration number/Company tax ID
    /// Required if merchantType is not Individual
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,

    /// [iso alpha 2 country code](1)
    /// Country of Registration
    /// Required if merchantType is not Individual
    ///
    /// [1]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_country: Option<String>,

    /// maximum length 1024
    /// Country of Registration
    /// Required if merchantType is not Individual
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_address: Option<String>,

    /// UnixTimestamp in milliseconds
    /// The date when the business registration is in effective
    /// Required if merchantType is not Individual
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<u64>,

    /// 1=ID
    /// 2=Passport
    /// Required if merchantType is Individual
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<u8>,

    /// [iso alpha 2 country code](1)
    /// Required if merchantType is Individual
    ///
    /// [1]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_country: Option<String>,

    /// maximum length 64
    /// Required if merchantType is Individual
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_number: Option<String>,

    /// UnixTimestamp in milliseconds
    /// Certificate Valid Date
    /// Required if merchantType is Individual
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_valid_date: Option<i64>,

    /// UnixTimestamp in milliseconds
    /// Contract date with ISV
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_time_isv: Option<i64>,

    /// ISO Alpha 2 country codes.
    /// Blocks payers from specified countries based on their KYC country.
    /// The list contains the countries to be blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_payer_kyc_countries: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1SubmerchantAddResult {
    /// unique sub-merchant id generated by payment
    pub sub_merchant_id: i64,
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn v1_submerchant_add(
        &self,
        request: V1SubmerchantAddRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<BinancePayResponse<V1SubmerchantAddResult>> {
        self.client
            .post_json(BINANCEPAY_OPENAPI_SUBMERCHANT_ADD, request)?
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
    fn v1_test_serde_add_submerchant_individual_request() {
        let json = r#"
        {
            "merchantName":"Individual",
            "merchantType":1,
            "merchantMcc":"5511",
            "brandLogo":null,
            "country":"CN,US",
            "address":null,
            "companyName":null,
            "registrationNumber":null,
            "registrationCountry":null,
            "registrationAddress":null,
            "incorporationDate":null,
            "storeType":1,
            "siteType":null,
            "siteUrl":null,
            "siteName":null,
            "certificateType":1,
            "certificateCountry":"US",
            "certificateNumber":"123456X",
            "certificateValidDate":1752422400000,
            "contractTimeIsv":1594656000000,
            "blockPayerKycCountries": ["US", "AE"]
        }
        "#;
        let request: V1SubmerchantAddRequest = serde_json::from_str(json).expect("Failed from_str");
        println!(
            "test_serde_add_submerchant_individual_request :: {:#?}",
            request
        );

        let request = V1SubmerchantAddRequest {
            merchant_name: "Individual".to_owned(),
            merchant_type: Some(1),
            merchant_mcc: "5511".to_owned(),
            pay_industry_description: None,
            sub_pay_mcc_code: None,
            sub_pay_industry_description: None,
            brand_logo: None,
            country: Some("CN,US".to_owned()),
            address: None,
            company_name: None,
            registration_number: None,
            registration_country: None,
            registration_address: None,
            incorporation_date: None,
            store_type: 1,
            site_url: None,
            certificate_type: Some(1),
            certificate_country: Some("US".to_owned()),
            certificate_number: Some("123456X".to_owned()),
            certificate_valid_date: Some(1752422400000),
            contract_time_isv: Some(1594656000000),
            block_payer_kyc_countries: Some(vec!["US".to_owned(), "AE".to_owned()]),
        };
        let json = serde_json::to_string(&request).expect("Failed to_string");
        println!("test_serde_add_submerchant_individual_request :: {}", json);
    }

    #[test]
    fn v1_test_serde_add_submerchant_non_individual_request() {
        let json = r#"
        {
            "merchantName":"Sole Proprietor",
            "merchantType":2,
            "merchantMcc":"5511",
            "payIndustryDescription": null,
            "subPayMccCode": null,
            "subPayIndustryDescription": null,
            "brandLogo":"logoUrlDemo",
            "country":"CN,US",
            "address":"store address demo",
            "companyName":"Sole Proprietor",
            "registrationNumber":"registration number demo",
            "registrationCountry":"US",
            "registrationAddress":"registration address demo",
            "incorporationDate":1588262400000,
            "storeType":1,
            "siteUrl":"site url demo",
            "certificateType":null,
            "certificateCountry":null,
            "certificateNumber":null,
            "certificateValidDate":null,
            "contractTimeIsv":1594656000000,
            "blockPayerKycCountries": ["US", "AE"]
        }
        "#;
        let request: V1SubmerchantAddRequest = serde_json::from_str(json).expect("Failed from_str");
        println!(
            "test_serde_add_submerchant_non_individual_request :: {:#?}",
            request
        );

        let request = V1SubmerchantAddRequest {
            merchant_name: "Sole Proprietor".to_owned(),
            merchant_type: Some(2),
            merchant_mcc: "5511".to_owned(),
            pay_industry_description: None,
            sub_pay_mcc_code: None,
            sub_pay_industry_description: None,
            brand_logo: Some("logoUrlDemo".to_owned()),
            country: Some("CN,US".to_owned()),
            address: Some("store address demo".to_owned()),
            company_name: Some("Sole Proprietor".to_owned()),
            registration_number: Some("registration number demo".to_owned()),
            registration_country: Some("US".to_owned()),
            registration_address: Some("registration address demo".to_owned()),
            incorporation_date: Some(1588262400000),
            store_type: 1,
            site_url: Some("site url demo".to_owned()),
            certificate_type: None,
            certificate_country: None,
            certificate_number: None,
            certificate_valid_date: None,
            contract_time_isv: Some(1594656000000),
            block_payer_kyc_countries: Some(vec!["US".to_owned(), "AE".to_owned()]),
        };
        let json = serde_json::to_string(&request).expect("Failed to_string");
        println!(
            "test_serde_add_submerchant_non_individual_request :: {}",
            json
        );
    }

    #[test]
    fn v1_test_serde_add_submerchant_response() {
        let example = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": {
                "subMerchantId": 2107268400000001
            }
        }
        "#;
        let response: BinancePayResponse<V1SubmerchantAddResult> =
            serde_json::from_str(example).expect("Failed from_str");
        println!(
            "test_serde_add_submerchant_response response :: {:#?}",
            response
        );
    }
}
