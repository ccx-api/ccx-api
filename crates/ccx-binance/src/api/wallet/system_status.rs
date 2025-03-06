use serde::Deserialize;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::proto::Response;
use crate::proto::{PublicRequest, Request};
use crate::types::rate_limits::RateLimitType;

/// System Status (System)
///
/// Fetch system status.
///
/// Weight(IP): 1
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus;

impl Request for SystemStatus {
    type Response = SystemStatusResponse;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/sapi/v1/system/status";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 1)];
}

impl PublicRequest for SystemStatus {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatusResponse {
    pub status: SystemMaintenanceStatus,
    pub msg: String,
}

impl Response for SystemStatusResponse {}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum SystemMaintenanceStatus {
    Normal = 0,
    SystemMaintenance = 1,
}
