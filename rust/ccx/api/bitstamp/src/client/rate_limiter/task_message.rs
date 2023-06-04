use futures::channel::oneshot;

use super::TaskCosts;
use crate::BitstampResult;

pub(super) type TaskMessageResult = BitstampResult<()>;

pub(super) struct TaskMessage {
    pub priority: u8,
    pub costs: TaskCosts,
    pub tx: oneshot::Sender<TaskMessageResult>,
}
