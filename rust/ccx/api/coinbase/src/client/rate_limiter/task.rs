use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::FutureExt;

use super::TaskCosts;
use crate::client::TaskMetadata;
use crate::CoinbaseApiResult;

pub struct Task<V>
where
    V: serde::de::DeserializeOwned + fmt::Debug,
{
    fut: Pin<Box<dyn Future<Output = CoinbaseApiResult<V>>>>,
    costs: TaskCosts,
}

impl<V> Task<V>
where
    V: serde::de::DeserializeOwned + fmt::Debug,
{
    pub(super) fn new(
        fut: Pin<Box<dyn Future<Output = CoinbaseApiResult<V>>>>,
        costs: TaskCosts,
    ) -> Self {
        Task { fut, costs }
    }

    pub fn metadata(&self) -> TaskMetadata {
        TaskMetadata {
            costs: self.costs.clone(),
        }
    }
}

impl<V> Future for Task<V>
where
    V: serde::de::DeserializeOwned + fmt::Debug,
{
    type Output = CoinbaseApiResult<V>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.fut.poll_unpin(cx)
    }
}
