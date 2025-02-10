use std::collections::VecDeque;

use crate::spot::rate_limiter::types::Task;

pub struct Queue {
    deq: VecDeque<Task>,
}

impl Queue {
    pub(crate) fn new() -> Self {
        let inner = VecDeque::new();
        Self { deq: inner }
    }

    // TODO optimize
    pub(crate) fn add(&mut self, msg: Task) {
        let priority = msg.priority;
        self.deq.push_back(msg);

        if priority > 0 {
            self.deq
                .make_contiguous()
                .sort_by(|a, b| b.priority.cmp(&a.priority));
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.deq.is_empty()
    }

    pub(crate) fn pop(&mut self) -> Option<Task> {
        self.deq.pop_front()
    }

    pub(crate) fn first(&self) -> Option<&Task> {
        self.deq.front()
    }
}
