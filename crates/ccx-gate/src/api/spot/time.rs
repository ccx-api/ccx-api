use serde::{Deserialize, Serialize};

use crate::proto::{PublicRequest, Request, Response};

#[derive(Serialize, Debug)]
pub struct GetServerTime;

#[derive(Deserialize, Debug)]
pub struct ServerTime {
    pub server_time: u64,
}

impl Response for ServerTime {}

impl Request for GetServerTime {
    type Response = ServerTime;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const ENDPOINT: &'static str = "/api/v4/spot/time";
}

impl PublicRequest for GetServerTime {}
