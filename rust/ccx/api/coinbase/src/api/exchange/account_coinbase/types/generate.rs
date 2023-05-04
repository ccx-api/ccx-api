use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Atom;
use crate::DtCoinbase;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCryptoAddress {
    pub id: Uuid,
    pub address: Atom,
    pub network: Atom,
    pub address_info: AddressInfo,
    pub created_at: DtCoinbase,
    pub updated_at: DtCoinbase,
    pub uri_scheme: String,
    pub resource: String,
    pub resource_path: String,
    pub warnings: Vec<Warning>,
    pub legacy_address: Option<String>,
    pub destination_tag: Option<String>,
    pub deposit_uri: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressInfo {
    pub address: Atom,
    pub destination_tag: Option<Atom>,
    pub name: Atom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warning {
    pub title: String,
    pub details: String,
    pub image_url: Option<String>,
}
