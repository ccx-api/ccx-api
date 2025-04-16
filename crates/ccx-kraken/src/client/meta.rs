use http::{HeaderValue, StatusCode};

use crate::api::error::KrakenApiError;

pub type KrakenError = ccx_lib::Error<KrakenApiError>;

#[derive(Debug)]
pub struct KrakenResponseWithMeta<T> {
    pub meta: KrakenResponseMeta,
    pub payload: T,
}

#[derive(Debug, derive_more::Error, derive_more::Display)]
#[display("{error}")]
pub struct KrakenErrorWithMeta {
    pub meta: Option<KrakenResponseMeta>,
    pub error: KrakenError,
}

#[derive(Debug)]
pub struct KrakenResponseMeta {
    pub http_status: StatusCode,
    pub trace_id: Option<String>,
    pub rate_limit: RateLimitMeta,
}

#[derive(Debug)]
pub struct RateLimitMeta {
    pub remain: u32,
    pub limit: u32,
}

impl<T> KrakenResponseWithMeta<T> {
    pub fn new(payload: T, meta: KrakenResponseMeta) -> Self {
        KrakenResponseWithMeta { meta, payload }
    }

    pub fn into_parts(self) -> (KrakenResponseMeta, T) {
        (self.meta, self.payload)
    }

    pub fn into_meta(self) -> KrakenResponseMeta {
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

impl KrakenResponseMeta {
    pub(super) fn from_response(resp: &reqwest::Response) -> Self {
        let http_status = resp.status();

        let trace_id = resp
            .headers()
            .get("x-kraken-trace-id")
            .and_then(|h| h.to_str().ok())
            .map(ToString::to_string);

        let rate_limit = RateLimitMeta {
            remain: parse_num(resp.headers().get("x-kraken-ratelimit-requests-remain"))
                .unwrap_or_default(),
            limit: parse_num(resp.headers().get("x-kraken-ratelimit-limit")).unwrap_or_default(),
        };

        KrakenResponseMeta {
            http_status,
            trace_id,
            rate_limit,
        }
    }

    pub fn error(self, error: impl Into<KrakenError>) -> KrakenErrorWithMeta {
        KrakenErrorWithMeta {
            error: error.into(),
            meta: Some(self),
        }
    }

    pub fn response<T>(self, payload: T) -> KrakenResponseWithMeta<T> {
        KrakenResponseWithMeta {
            payload,
            meta: self,
        }
    }
}

impl<T> From<T> for KrakenErrorWithMeta
where
    T: Into<KrakenError>,
{
    fn from(error: T) -> Self {
        Self {
            error: error.into(),
            meta: None,
        }
    }
}
