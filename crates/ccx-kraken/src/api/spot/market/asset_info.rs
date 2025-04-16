use std::collections::HashMap;

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::StringWithSeparator;
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;

use crate::proto::{PublicRequest, Request, Response};
use crate::types::asset_info::AssetName;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum AssetClass {
    #[serde(rename = "currency")]
    Currency,
    // TODO other classes ?
}

#[serde_as]
#[derive(Serialize, Debug, Builder)]
pub struct AssetInfo {
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    asset: Option<Vec<String>>,
    aclass: Option<AssetClass>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AssetInfoEntry {
    /// Asset Class.
    pub aclass: AssetClass,
    /// Alternate name.
    pub altname: AssetName,
    /// Scaling decimal places for record keeping.
    pub decimals: u32,
    /// Scaling decimal places for output display.
    pub display_decimals: u32,
}

#[derive(Deserialize, Debug)]
pub struct AssetInfoResponse {
    #[serde(flatten)]
    pub assets: HashMap<AssetName, AssetInfoEntry>,
}

impl Response for AssetInfoResponse {}

impl Request for AssetInfo {
    type Response = AssetInfoResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const ENDPOINT: &'static str = "/0/public/Assets";
}

impl PublicRequest for AssetInfo {}
