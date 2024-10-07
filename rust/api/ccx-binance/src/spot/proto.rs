use std::future::Future;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::spot::client::BinanceSpotClient;
use crate::spot::client::BinanceSpotSigner;
use crate::spot::client::SignedRequest;
use crate::spot::client::Stamped;
use crate::spot::client::TimeWindow;
use crate::spot::error::BinanceSpotError;
use crate::spot::error::BinanceSpotSendError;
use crate::spot::meta::BinanceSpotResponseMeta;
use crate::spot::rate_limiter::RateLimiter;
use crate::spot::rate_limiter::RateLimiterError;
use crate::spot::types::rate_limits::RateLimitType;

pub trait BinanceSpotRequest: Serialize {
    type Response: BinanceSpotResponse;
    const HTTP_METHOD: http::Method;
    const ENDPOINT: &'static str;

    /// Rate limiter bucket type and score for this request.
    const COSTS: &'static [(RateLimitType, u32)];

    fn costs(&self) -> &'static [(RateLimitType, u32)] {
        Self::COSTS
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
            rate_limiter.enqueue(0, Self::COSTS).await?;
            Ok(self)
        }
    }
}

pub trait BinanceSpotResponse: DeserializeOwned {}

impl<T> BinanceSpotRequest for &T
where
    T: BinanceSpotRequest,
{
    type Response = T::Response;
    const HTTP_METHOD: http::Method = T::HTTP_METHOD;
    const ENDPOINT: &'static str = T::ENDPOINT;
    const COSTS: &'static [(RateLimitType, u32)] = T::COSTS;
}

pub trait BinanceSpotPublic: BinanceSpotRequest {
    fn send(
        self,
        client: &BinanceSpotClient,
    ) -> impl Future<Output = Result<BinanceSpotResponseMeta<Self::Response>, BinanceSpotSendError>> + Send
    where
        Self: Send + Sync + Sized,
    {
        let client = client.clone();
        async move { client.send_public(self).await }
    }
}

pub trait BinanceSpotPrivate: BinanceSpotRequest {
    // fn send(
    //     self,
    //     client: &BinanceSpotClient,
    //     api_key: &str,
    // ) -> impl Future<Output = Result<Self::Response, BinanceSpotError>> + Send
    // where
    //     Self: Send + Sync + Sized,
    // {
    //     let client = client.clone();
    //     async move { client.send_private(self, api_key).await }
    // }
}

pub trait BinanceSpotSigned: BinanceSpotRequest + Send {
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
        signer: impl BinanceSpotSigner,
    ) -> impl Future<Output = Result<SignedRequest<Self>, BinanceSpotError>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await }
    }

    fn sign_now_and_send(
        self,
        signer: impl BinanceSpotSigner,
        client: &BinanceSpotClient,
    ) -> impl Future<Output = Result<BinanceSpotResponseMeta<Self::Response>, BinanceSpotSendError>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await?.send(client).await }
    }
}

pub trait BinanceSpotReadyToSend<T>
where
    T: BinanceSpotRequest,
{
    fn send(
        self,
        client: &BinanceSpotClient,
    ) -> impl Future<Output = Result<BinanceSpotResponseMeta<T::Response>, BinanceSpotSendError>> + Send
    where
        Self: Sized + Send + Sync;
}
