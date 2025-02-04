use serde::Deserialize;
use serde::Serialize;

use crate::types::enums::AddressType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shipping {
    #[serde(rename = "shippingName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ShippingName>,
    #[serde(rename = "shippingAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ShippingAddress>,
    #[serde(rename = "shippingPhoneNo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_no: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingName {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "middleName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingAddress {
    pub region: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "zipCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(rename = "shippingAddressType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_type: Option<AddressType>,
}
