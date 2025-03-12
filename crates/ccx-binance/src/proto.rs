use std::future::Future;

use ccx_lib::rate_limiter::TaskCosts;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::client::BinanceClient;
use crate::client::BinanceSigner;
use crate::client::SignedReadyRequest;
use crate::client::Stamped;
use crate::client::TimeWindow;
use crate::client::meta::BinanceResponseWithMeta;
use crate::error::BinanceError;
use crate::error::BinanceErrorWithMeta;
use crate::rate_limiter::RateLimiter;
use crate::rate_limiter::RateLimiterError;
use crate::types::rate_limits::RateLimitType;

pub trait Request: Serialize {
    type Response: Response;
    const HTTP_METHOD: http::Method;
    const ENDPOINT: &'static str;

    /// Rate limiter bucket type and score for this request.
    const COSTS: &'static [(RateLimitType, u32)];

    fn costs(&self) -> TaskCosts<RateLimitType> {
        Self::COSTS.into()
    }

    fn throttle(
        self,
        rate_limiter: &RateLimiter,
    ) -> impl Future<Output = Result<Self, RateLimiterError>> + Send
    where
        Self: Sized + Send,
    {
        let mut rate_limiter = rate_limiter.clone();
        async move {
            let mut costs = self.costs();
            // increment counter by 1 for every request here
            // instead of defining the same cost for every request
            // definition
            costs.push((RateLimitType::RawRequests, 1));
            rate_limiter.enqueue(0, costs).await?;
            Ok(self)
        }
    }
}

pub trait Response: DeserializeOwned {}

impl Response for () {}

impl<T> Response for Vec<T> where T: Response {}

impl<T> Request for &T
where
    T: Request,
{
    type Response = T::Response;
    const HTTP_METHOD: http::Method = T::HTTP_METHOD;
    const ENDPOINT: &'static str = T::ENDPOINT;
    const COSTS: &'static [(RateLimitType, u32)] = T::COSTS;
}

pub trait PublicRequest: Request {}

pub trait SignedRequest: Request + Send {
    fn stamp(self, time_window: TimeWindow) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self, time_window)
    }

    fn now(self) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self, TimeWindow::now())
    }

    fn sign_now(
        self,
        signer: impl BinanceSigner,
    ) -> impl Future<Output = Result<SignedReadyRequest<Self>, BinanceError>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await }
    }

    fn sign_now_and_send(
        self,
        signer: impl BinanceSigner,
        client: &BinanceClient,
    ) -> impl Future<Output = Result<BinanceResponseWithMeta<Self::Response>, BinanceErrorWithMeta>> + Send
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
    fn send(
        self,
        client: &BinanceClient,
    ) -> impl Future<Output = Result<BinanceResponseWithMeta<T::Response>, BinanceErrorWithMeta>> + Send
    where
        Self: Sized + Send + Sync;
}
