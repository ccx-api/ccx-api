use serde::{Deserialize, Serialize};

use crate::proto::{PublicRequest, Request, Response};

/// Current system status.
///
/// * online - Kraken is operating normally. All order types may be submitted and trades can occur.
/// * maintenance - The exchange is offline. No new orders or cancellations may be submitted.
/// * cancel_only - Resting (open) orders can be cancelled but no new orders may be submitted. No trades will occur.
/// * post_only - Only post-only limit orders can be submitted. Existing orders may still be cancelled. No trades will occur.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Online,
    Maintenance,
    CancelOnly,
    PostOnly,
}

/// Get System Status.
///
/// Get the current system status or trading mode.
#[derive(Serialize, Debug)]
pub struct SystemStatus;

#[derive(Deserialize, Debug)]
pub struct SystemStatusResponse {
    pub status: Status,
    /// Current timestamp (RFC3339)
    pub timestamp: String,
}

impl Response for SystemStatusResponse {}

impl Request for SystemStatus {
    type Response = SystemStatusResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const ENDPOINT: &'static str = "/0/public/SystemStatus";
}

impl PublicRequest for SystemStatus {}
