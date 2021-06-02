use std::ops;

use chrono::Utc;

use crate::{RequestError, LibResult};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TimeWindow {
    timestamp: u64,
    recv_window: RecvWindow,
}

impl TimeWindow {
    pub fn new(timestamp: u64) -> Self {
        let recv_window = RecvWindow::default();
        TimeWindow {
            timestamp,
            recv_window,
        }
    }

    pub fn now() -> Self {
        TimeWindow::new(Utc::now().timestamp_millis() as u64)
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn recv_window(&self) -> RecvWindow {
        self.recv_window
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RecvWindow(u32);

impl Default for RecvWindow {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl ops::Deref for RecvWindow {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RecvWindow {
    pub const DEFAULT: RecvWindow = RecvWindow(5000);

    pub fn new(window: u32) -> LibResult<Self> {
        match () {
            () if window > 60000 => Err(RequestError::OutOfBounds)?,
            () => Ok(RecvWindow(window)),
        }
    }

    pub fn is_default(self) -> bool {
        self == Self::DEFAULT
    }
}
