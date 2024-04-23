use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::Atom;
use crate::DtCoinbaseEx;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCoinbaseAccount {
    pub id: Uuid,
    pub address: Atom,
    pub address_info: AddressInfo,
    pub name: Atom,
    pub network: Atom,
    pub created_at: DtCoinbaseEx,
    pub updated_at: DtCoinbaseEx,
    pub uri_scheme: String,
    pub resource: String,
    pub resource_path: String,
    pub warnings: Vec<Warning>,
    #[serde(default)]
    pub qr_code_image_url: Option<String>,
    #[serde(default)]
    pub address_label: Option<String>,
    #[serde(default)]
    pub default_receive: bool,
    #[serde(default)]
    pub legacy_address: Option<String>,
    #[serde(default)]
    pub destination_tag: Option<u32>,
    #[serde(default)]
    pub deposit_uri: Option<String>,
    #[serde(default)]
    pub callback_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressInfo {
    pub address: Atom,
    pub destination_tag: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warning {
    pub title: String,
    pub details: String,
    pub image_url: Option<String>,
}
