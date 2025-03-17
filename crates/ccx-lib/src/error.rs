use crate::rate_limiter::RateLimiterError;

#[derive(Debug, derive_more::From, derive_more::Display)]
#[from(forward)]
#[display("Failed to sign request: {_0}")]
pub struct SignError(pub anyhow::Error);

#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum Error<ApiError: CcxApiError> {
    #[display("Api Error: {_0}")]
    Api(ApiError),
    Sign(SignError),
    #[display("Failed to serialize data to urlencoded: {_0}")]
    Serialize(serde_urlencoded::ser::Error),
    #[display("Failed to deserialize data from urlencoded: {_0}")]
    Deserialize(serde_urlencoded::de::Error),
    #[display("Failed to serialize/deserialize data: {_0}")]
    JsonConversion(serde_json::Error),
    #[display("Request error: {_0}")]
    Request(reqwest::Error),
    #[display("Failed to parse url")]
    UrlParse(url::ParseError),
    #[display("Failed to format argument: {_0}")]
    Format(std::fmt::Error),
    #[display("RequestLimiter error: {_0}")]
    RateLimiter(RateLimiterError),
}

pub trait CcxApiError: std::error::Error + Send + Sync {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, derive_more::Error, derive_more::Display)]
    #[display("test error")]
    struct TestError;

    impl CcxApiError for TestError {}

    #[test]
    fn test_api_error() {
        fn api_call() -> Result<(), TestError> {
            Err(TestError)
        }

        fn call() -> Result<(), Error<TestError>> {
            Ok(api_call().map_err(Error::Api)?)
        }

        let err = call().unwrap_err();

        assert_eq!(err.to_string(), "Api Error: test error");
    }

    #[test]
    fn test_sign_error() {
        fn any_error() -> Result<(), TestError> {
            Err(TestError)
        }

        fn sign() -> Result<(), SignError> {
            Ok(any_error()?)
        }

        fn call() -> Result<(), Error<TestError>> {
            Ok(sign()?)
        }

        let err = call().unwrap_err();

        assert_eq!(err.to_string(), "Failed to sign request: test error");
    }

    #[test]
    fn test_serialize_error() {
        fn serialize() -> Result<(), serde_json::Error> {
            serde_json::from_str("")
        }

        fn call() -> Result<(), Error<TestError>> {
            Ok(serialize()?)
        }

        let err = call().unwrap_err();

        assert_eq!(
            err.to_string(),
            "Failed to serialize/deserialize data: EOF while parsing a value at line 1 column 0"
        );
    }
}
