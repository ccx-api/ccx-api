use futures::channel::oneshot;

use super::TaskCosts;
use crate::CoinbaseResult;

pub(super) type TaskMessageResult = CoinbaseResult<()>;

pub(super) struct TaskMessage {
    pub priority: u8,
    pub costs: TaskCosts,
    pub tx: oneshot::Sender<TaskMessageResult>,
}
