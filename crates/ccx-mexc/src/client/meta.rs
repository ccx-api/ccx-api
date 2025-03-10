use http::StatusCode;

#[derive(Debug)]
pub struct MexcResponseWithMeta<T> {
    pub meta: MexcResponseMeta,
    pub payload: T,
}

#[derive(Debug)]
pub struct MexcResponseMeta {
    pub http_status: StatusCode,
}

impl<T> MexcResponseWithMeta<T> {
    pub fn new(meta: MexcResponseMeta, payload: T) -> Self {
        MexcResponseWithMeta { meta, payload }
    }

    pub fn into_parts(self) -> (MexcResponseMeta, T) {
        (self.meta, self.payload)
    }

    pub fn into_meta(self) -> MexcResponseMeta {
        self.meta
    }

    pub fn into_payload(self) -> T {
        self.payload
    }
}

impl MexcResponseMeta {
    pub(super) fn from_response(resp: &reqwest::Response) -> Self {
        let http_status = resp.status();
        tracing::debug!("Response status: {http_status}");

        MexcResponseMeta { http_status }
    }
}
