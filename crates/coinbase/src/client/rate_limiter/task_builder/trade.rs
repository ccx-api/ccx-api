use std::fmt;

use ccx_api_lib::LibError;
use futures::FutureExt;
use futures::SinkExt;
use futures::channel::mpsc;
use futures::channel::oneshot;

use crate::client::CoinbaseTradeSigner;
use crate::client::Task;
use crate::client::TradeRequestBuilder;
use crate::client::rate_limiter::BucketName;
use crate::client::rate_limiter::task_message::TaskMessage;
use crate::client::rate_limiter::task_message::TaskMessageResult;
use crate::client::rate_limiter::task_metadata::TaskCosts;

#[allow(dead_code)]
pub(crate) struct TradeTaskBuilder<S>
where
    S: CoinbaseTradeSigner + Unpin + 'static,
{
    priority: u8,
    costs: TaskCosts,
    req_builder: TradeRequestBuilder<S>,
    tasks_tx: mpsc::UnboundedSender<TaskMessage>,
}

impl<S> TradeTaskBuilder<S>
where
    S: CoinbaseTradeSigner + Unpin + 'static,
{
    #[allow(dead_code)]
    pub(in super::super) fn new(
        priority: u8,
        costs: TaskCosts,
        req_builder: TradeRequestBuilder<S>,
        tasks_tx: mpsc::UnboundedSender<TaskMessage>,
    ) -> Self {
        TradeTaskBuilder {
            priority,
            costs,
            req_builder,
            tasks_tx,
        }
    }

    pub fn _priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    #[allow(dead_code)]
    pub fn cost(mut self, key: impl Into<BucketName>, weight: u32) -> Self {
        self.costs
            .entry(key.into())
            .and_modify(|e| *e = weight)
            .or_insert(weight);
        self
    }

    #[allow(dead_code)]
    pub fn send<V>(self) -> Task<V>
    where
        V: serde::de::DeserializeOwned + fmt::Debug,
    {
        let priority = self.priority;
        let costs = self.costs.clone();
        let req_builder = self.req_builder;
        let mut tasks_tx = self.tasks_tx.clone();

        let fut = async move {
            let (tx, rx) = oneshot::channel::<TaskMessageResult>();
            tasks_tx
                .send(TaskMessage {
                    priority,
                    costs,
                    tx,
                })
                .await
                .map_err(|_| LibError::other("RateLimiter: task channel was dropped"))?;
            rx.await
                .map_err(|_| LibError::other("RateLimiter: task channel was dropped"))?
                .map_err(|e| {
                    log::error!("RateLimiter: task err. {:?}", e);
                    e
                })?;

            req_builder.send::<V>().await
        };

        Task::new(fut.boxed_local(), self.costs)
    }
}
