use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Buyer {
    #[serde(rename = "referenceBuyerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "buyerName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<BuyerName>,

    #[serde(rename = "buyerPhoneCountryCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    #[serde(rename = "buyerPhoneNo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_no: Option<String>,
    #[serde(rename = "buyerEmail")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "buyerRegistrationTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_time: Option<i64>,
    #[serde(rename = "buyerBrowserLanguage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuyerName {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "middleName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: String,
}
