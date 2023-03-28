use futures::channel::oneshot;

use crate::CoinbaseResult;
use super::TaskCosts;

pub(super) type TaskMessageResult = CoinbaseResult<()>;

pub(super) struct TaskMessage {
    pub priority: u8,
    pub costs: TaskCosts,
    pub tx: oneshot::Sender<TaskMessageResult>,
}
