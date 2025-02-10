use std::ops;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct RecvWindow(u32);

impl ops::Deref for RecvWindow {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RecvWindow {
    pub fn new(window: u32) -> Result<Self, OutOfBounds> {
        if window > 60000 {
            Err(OutOfBounds)?
        }
        Ok(RecvWindow(window))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, thiserror::Error)]
#[error("The value is out of bounds. It must be between 0 and 60000.")]
pub struct OutOfBounds;
