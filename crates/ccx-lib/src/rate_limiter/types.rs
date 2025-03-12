use futures::channel::oneshot;
use smallvec::SmallVec;

pub(super) enum RateLimiterMessage<RateLimitType: 'static> {
    Enqueue(Task<RateLimitType>),
}

pub type TaskCosts<RateLimitType> = SmallVec<[(RateLimitType, u32); 3]>;
pub type TaskCostsRef<'a, RateLimitType> = &'a [(RateLimitType, u32)];

#[derive(Debug)]
pub(super) struct Task<RateLimitType: 'static> {
    pub priority: u8,
    pub costs: TaskCosts<RateLimitType>,
    pub tx: oneshot::Sender<()>,
}
