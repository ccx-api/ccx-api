use std::borrow::Cow;

use ccx_lib::rate_limiter::RateLimiterError;
use chrono::Utc;
use serde::Serialize;
use serde::de::DeserializeOwned;
use smallvec::smallvec;

use crate::client::KrakenClient;
use crate::client::meta::KrakenError;
use crate::client::ready::ReadyRequest;
use crate::client::signer::KrakenSigner;
use crate::client::stamped::Stamped;
use crate::error::KrakenResult;
use crate::prelude::Nonce;
use crate::rate_limiter::{RateLimitKey, RateLimiter};
use crate::types::rate_limits::{RateLimitPrivateType, RateLimitType};

pub trait Request: Serialize + Send + Sync {
    type Response: Response;

    const HTTP_METHOD: http::Method;
    /// IMPORTANT: endpoint should define the whole path i.e. starting from /0/public/
    const ENDPOINT: &'static str;

    fn path(&self) -> Cow<'static, str> {
        Self::ENDPOINT.into()
    }

    /// Rate limiter bucket type and score for this request.
    const COSTS: &'static RateLimitType = &RateLimitType::Private(RateLimitPrivateType::Normal);

    #[tracing::instrument(skip_all)]
    fn throttle(
        self,
        rate_limiter: &RateLimiter,
    ) -> impl Future<Output = Result<Self, RateLimiterError>> + Send
    where
        Self: Sized + Send,
    {
        let mut rate_limiter = rate_limiter.clone();

        async move {
            let cost = match Self::COSTS {
                RateLimitType::Public => 1,
                RateLimitType::Private(ty) => match ty {
                    RateLimitPrivateType::Normal => 1,
                    RateLimitPrivateType::History => 2,
                },
                // TODO: for orders rate limits are more complex
                // https://docs.kraken.com/api/docs/guides/spot-ratelimits/
                // at least it should be implemented separately for every
                // [CurrencyPair] instead of generic counter
                RateLimitType::Order => 0,
            };

            rate_limiter
                .enqueue(0, smallvec![(RateLimitKey::from(Self::COSTS), cost)])
                .await?;
            Ok(self)
        }
    }
}

pub trait Response: DeserializeOwned + Send + Sync {}

impl<T> Response for Vec<T> where T: Response {}
impl<T, const N: usize> Response for smallvec::SmallVec<[T; N]> where T: Response {}

impl<T> Request for &T
where
    T: Request,
{
    type Response = T::Response;

    const HTTP_METHOD: http::Method = T::HTTP_METHOD;
    const ENDPOINT: &'static str = T::ENDPOINT;
    const COSTS: &'static RateLimitType = T::COSTS;
}

pub trait PublicRequest: Request {}

pub trait SignedRequest: Request + Send {
    fn stamp(self, nonce: Nonce) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self, nonce)
    }

    fn now(self) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self, Utc::now().timestamp_millis() as u64)
    }

    #[tracing::instrument(skip_all)]
    fn sign_now(
        self,
        signer: impl KrakenSigner,
    ) -> impl Future<Output = Result<ReadyRequest<Self>, KrakenError>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await }
    }

    #[tracing::instrument(skip_all)]
    fn sign_now_and_send(
        self,
        signer: impl KrakenSigner,
        client: &KrakenClient,
    ) -> impl Future<Output = KrakenResult<Self::Response>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await?.send(client).await }
    }
}

pub trait RequestReadyToSend<T>
where
    T: Request,
{
    fn send(self, client: &KrakenClient) -> impl Future<Output = KrakenResult<T::Response>> + Send
    where
        Self: Sized + Send + Sync;
}
