use std::borrow::Cow;

use ccx_lib::rate_limiter::RateLimiterError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use smallvec::smallvec;

use crate::client::BitGoClient;
use crate::client::meta::BitGoError;
use crate::client::ready::ReadyRequest;
use crate::client::signer::BitGoSigner;
use crate::client::stamped::Stamped;
use crate::error::BitGoResult;
use crate::rate_limiter::RateLimiter;
use crate::types::rate_limits::RateLimitType;

pub trait Request: Serialize + Send + Sync {
    type Response: Response;

    const HTTP_METHOD: http::Method;

    /// Rate limiter bucket type and score for this request.
    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> Cow<'static, str>;

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
            rate_limiter
                .enqueue(0, smallvec![(*Self::COSTS, 1)])
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
    const COSTS: &'static RateLimitType = T::COSTS;

    fn path(&self) -> Cow<'static, str> {
        (*self).path()
    }
}

pub trait PublicRequest: Request {}

pub trait SignedRequest: Request + Send {
    fn now(self) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self)
    }

    #[tracing::instrument(skip_all)]
    fn sign_now(
        self,
        signer: impl BitGoSigner,
    ) -> impl Future<Output = Result<ReadyRequest<Self>, BitGoError>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await }
    }

    #[tracing::instrument(skip_all)]
    fn sign_now_and_send(
        self,
        signer: impl BitGoSigner,
        client: &BitGoClient,
    ) -> impl Future<Output = BitGoResult<Self::Response>> + Send
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
    fn send(self, client: &BitGoClient) -> impl Future<Output = BitGoResult<T::Response>> + Send
    where
        Self: Sized + Send + Sync;
}
