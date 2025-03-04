use futures::channel::oneshot;

pub(super) enum RateLimiterMessage<RateLimitType: 'static> {
    Enqueue(Task<RateLimitType>),
}

pub type TaskCosts<RateLimitType> = &'static [(RateLimitType, u32)];

#[derive(Debug)]
pub(super) struct Task<RateLimitType: 'static> {
    pub priority: u8,
    pub costs: TaskCosts<RateLimitType>,
    pub tx: oneshot::Sender<()>,
}
