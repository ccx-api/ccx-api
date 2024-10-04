use futures::channel::oneshot;

use crate::spot::types::rate_limits::RateLimitType;

pub(super) enum TaskMessage {
    Enqueue(Task),
}

pub type TaskCosts = &'static [(RateLimitType, u32)];

pub(super) struct Task {
    pub priority: u8,
    pub costs: TaskCosts,
    pub tx: oneshot::Sender<()>,
}
