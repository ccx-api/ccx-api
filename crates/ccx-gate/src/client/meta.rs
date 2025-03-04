use http::{HeaderValue, StatusCode};

use crate::api::error::GateApiError;

pub type GateError = ccx_lib::Error<GateApiError>;

#[derive(Debug)]
pub struct GateResponseWithMeta<T> {
    pub meta: GateResponseMeta,
    pub payload: T,
}

#[derive(Debug, derive_more::Error, derive_more::Display)]
#[display("{error}")]
pub struct GateErrorWithMeta {
    pub meta: Option<GateResponseMeta>,
    pub error: GateError,
}

#[derive(Debug)]
pub struct GateResponseMeta {
    pub http_status: StatusCode,
    pub trace_id: Option<String>,
    pub rate_limit: RateLimitMeta,
}

#[derive(Debug)]
pub struct RateLimitMeta {
    pub remain: u32,
    pub limit: u32,
}

impl<T> GateResponseWithMeta<T> {
    pub fn new(payload: T, meta: GateResponseMeta) -> Self {
        GateResponseWithMeta { meta, payload }
    }

    pub fn into_parts(self) -> (GateResponseMeta, T) {
        (self.meta, self.payload)
    }

    pub fn into_meta(self) -> GateResponseMeta {
        self.meta
    }

    pub fn into_payload(self) -> T {
        self.payload
    }
}

fn parse_num(header: Option<&HeaderValue>) -> Option<u32> {
    header
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.parse().ok())
}

impl GateResponseMeta {
    pub(super) fn from_response(resp: &reqwest::Response) -> Self {
        let http_status = resp.status();

        let trace_id = resp
            .headers()
            .get("x-gate-trace-id")
            .and_then(|h| h.to_str().ok())
            .map(ToString::to_string);

        let rate_limit = RateLimitMeta {
            remain: parse_num(resp.headers().get("x-gate-ratelimit-requests-remain"))
                .unwrap_or_default(),
            limit: parse_num(resp.headers().get("x-gate-ratelimit-limit")).unwrap_or_default(),
        };

        GateResponseMeta {
            http_status,
            trace_id,
            rate_limit,
        }
    }

    pub fn error(self, error: impl Into<GateError>) -> GateErrorWithMeta {
        GateErrorWithMeta {
            error: error.into(),
            meta: Some(self),
        }
    }

    pub fn response<T>(self, payload: T) -> GateResponseWithMeta<T> {
        GateResponseWithMeta {
            payload,
            meta: self,
        }
    }
}

impl<T> From<T> for GateErrorWithMeta
where
    T: Into<GateError>,
{
    fn from(error: T) -> Self {
        Self {
            error: error.into(),
            meta: None,
        }
    }
}
