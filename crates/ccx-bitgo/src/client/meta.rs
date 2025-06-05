use http::StatusCode;

use crate::api::error::BitGoApiError;

pub type BitGoError = ccx_lib::Error<BitGoApiError>;

#[derive(Debug)]
pub struct BitGoResponseWithMeta<T> {
    pub meta: BitGoResponseMeta,
    pub payload: T,
}

#[derive(Debug, derive_more::Error, derive_more::Display)]
#[display("{error}")]
pub struct BitGoErrorWithMeta {
    pub meta: Option<BitGoResponseMeta>,
    pub error: BitGoError,
}

#[derive(Debug)]
pub struct BitGoResponseMeta {
    pub http_status: StatusCode,
}

#[derive(Debug)]
pub struct RateLimitMeta {
    pub remain: u32,
    pub limit: u32,
}

impl<T> BitGoResponseWithMeta<T> {
    pub fn new(payload: T, meta: BitGoResponseMeta) -> Self {
        BitGoResponseWithMeta { meta, payload }
    }

    pub fn into_parts(self) -> (BitGoResponseMeta, T) {
        (self.meta, self.payload)
    }

    pub fn into_meta(self) -> BitGoResponseMeta {
        self.meta
    }

    pub fn into_payload(self) -> T {
        self.payload
    }
}

impl BitGoResponseMeta {
    pub(super) fn from_response(resp: &reqwest::Response) -> Self {
        let http_status = resp.status();

        BitGoResponseMeta { http_status }
    }

    pub fn error(self, error: impl Into<BitGoError>) -> BitGoErrorWithMeta {
        BitGoErrorWithMeta {
            error: error.into(),
            meta: Some(self),
        }
    }

    pub fn response<T>(self, payload: T) -> BitGoResponseWithMeta<T> {
        BitGoResponseWithMeta {
            payload,
            meta: self,
        }
    }
}

impl<T> From<T> for BitGoErrorWithMeta
where
    T: Into<BitGoError>,
{
    fn from(error: T) -> Self {
        Self {
            error: error.into(),
            meta: None,
        }
    }
}
