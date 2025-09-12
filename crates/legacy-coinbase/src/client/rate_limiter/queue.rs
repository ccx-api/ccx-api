use std::collections::VecDeque;

use super::TaskMessage;

pub(super) struct Queue {
    inner: VecDeque<TaskMessage>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    pub fn add(&mut self, msg: TaskMessage) -> &Self {
        let priority = msg.priority;
        self.inner.push_back(msg);

        if priority > 0 {
            self.inner
                .make_contiguous()
                .sort_by(|a, b| b.priority.cmp(&a.priority));
        }

        self
    }

    pub fn is_first(&self) -> bool {
        self.inner.len() == 1
    }
}

impl Iterator for Queue {
    type Item = TaskMessage;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_front()
    }
}
