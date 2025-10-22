use std::collections::HashMap;

use super::BucketName;

pub(super) type TaskCosts = HashMap<BucketName, u32>;

#[derive(Debug)]
pub struct TaskMetadata {
    pub costs: TaskCosts,
}
