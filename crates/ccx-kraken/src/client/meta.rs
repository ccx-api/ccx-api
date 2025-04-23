use http::StatusCode;

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

impl KrakenResponseMeta {
    pub(super) fn from_response(resp: &reqwest::Response) -> Self {
        let http_status = resp.status();

        KrakenResponseMeta { http_status }
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
